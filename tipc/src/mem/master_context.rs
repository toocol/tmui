use super::{
    mem_queue::{MemQueue, MemQueueError},
    mem_rw_lock::MemRwLock,
    BuildType, MemContext, IPC_MEM_LOCK_NAME, IPC_MEM_SIGNAL_EVT, IPC_QUEUE_SIZE,
};
use crate::{
    ipc_event::{InnerIpcEvent, IpcEvent},
    mem::{
        mem_queue::MemQueueBuilder, IpcError, RequestSide, SharedInfo, IPC_MEM_BUFFER_NAME,
        IPC_MEM_MASTER_QUEUE, IPC_MEM_SHARED_INFO_NAME, IPC_MEM_SLAVE_QUEUE,
    },
};
use log::error;
use parking_lot::Mutex;
use raw_sync::{
    events::{Event, EventInit, EventState},
    Timeout,
};
use shared_memory::{Shmem, ShmemConf};
use std::{
    error::Error,
    marker::PhantomData,
    mem::size_of,
    sync::{atomic::Ordering, Arc},
};
use tlib::global::SemanticExt;

pub(crate) struct MasterContext<T: 'static + Copy, M: 'static + Copy> {
    name: String,
    buffer: Option<Shmem>,
    shared_info: Shmem,
    wait_signal_mem: Shmem,
    buffer_lock: Arc<MemRwLock>,
    master_queue: MemQueue<IPC_QUEUE_SIZE, InnerIpcEvent<T>>,
    slave_queue: MemQueue<IPC_QUEUE_SIZE, InnerIpcEvent<T>>,
    _request_type: PhantomData<M>,
    mutex: Mutex<()>,
    pretreat_size: (u32, u32),
}

impl<T: 'static + Copy, M: 'static + Copy> MasterContext<T, M> {
    pub(crate) fn create<P: ToString>(name: P) -> Self {
        let mut shared_info_name = name.to_string();
        shared_info_name.push_str(IPC_MEM_SHARED_INFO_NAME);
        let shared_info = ShmemConf::new()
            .size(size_of::<SharedInfo<M>>())
            .os_id(shared_info_name)
            .create()
            .unwrap();

        unsafe {
            (shared_info.as_ptr() as *mut SharedInfo<M>)
                .as_mut()
                .unwrap()
                .prepared
                .store(true, Ordering::Release)
        };

        let mut event_signal_name = name.to_string();
        event_signal_name.push_str(IPC_MEM_SIGNAL_EVT);
        let event_signal_mem = ShmemConf::new()
            .size(size_of::<Event>())
            .os_id(event_signal_name)
            .create()
            .unwrap();

        let mut lock_name = name.to_string();
        lock_name.push_str(IPC_MEM_LOCK_NAME);
        let buffer_lock = MemRwLock::builder()
            .os_id(lock_name)
            .build_type(BuildType::Create)
            .build()
            .unwrap()
            .arc();

        let mut master_queue_name = name.to_string();
        master_queue_name.push_str(IPC_MEM_MASTER_QUEUE);
        let master_queue = MemQueueBuilder::new()
            .build_type(BuildType::Create)
            .os_id(master_queue_name)
            .build()
            .unwrap();

        let mut slave_queue_name = name.to_string();
        slave_queue_name.push_str(IPC_MEM_SLAVE_QUEUE);
        let slave_queue = MemQueueBuilder::new()
            .build_type(BuildType::Create)
            .os_id(slave_queue_name)
            .build()
            .unwrap();

        Self {
            name: name.to_string(),
            buffer: None,
            shared_info,
            buffer_lock,
            wait_signal_mem: event_signal_mem,
            master_queue,
            slave_queue,
            _request_type: Default::default(),
            mutex: Mutex::new(()),
            pretreat_size: (0, 0),
        }
    }

    pub(crate) fn shared_info(&self) -> &'static mut SharedInfo<M> {
        unsafe {
            (self.shared_info.as_ptr() as *mut SharedInfo<M>)
                .as_mut()
                .unwrap()
        }
    }
}

impl<T: 'static + Copy, M: 'static + Copy> MemContext<T, M> for MasterContext<T, M> {
    #[inline]
    fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    fn buffer(&self) -> *mut u8 {
        self.buffer.as_ref().unwrap().as_ptr()
    }

    #[inline]
    fn width(&self) -> u32 {
        self.shared_info().width.load(Ordering::Acquire)
    }

    #[inline]
    fn height(&self) -> u32 {
        self.shared_info().height.load(Ordering::Acquire)
    }

    #[inline]
    fn try_send(&self, evt: IpcEvent<T>) -> Result<(), MemQueueError> {
        self.master_queue.try_write(evt.into())
    }

    #[inline]
    fn has_event(&self) -> bool {
        self.slave_queue.has_event()
    }

    #[inline]
    fn try_recv(&self) -> Option<IpcEvent<T>> {
        self.slave_queue.try_read().map(|ipc_evt| ipc_evt.into())
    }

    #[inline]
    fn try_recv_vec(&self) -> Vec<IpcEvent<T>> {
        let mut vec = vec![];
        while self.slave_queue.has_event() {
            if let Some(evt) = self.slave_queue.try_read() {
                vec.push(evt.into())
            }
        }
        vec
    }

    #[inline]
    fn send_request(&self, request: M) -> Result<Option<M>, Box<dyn Error>> {
        let _guard = self.mutex.lock();
        let info = self.shared_info();
        if info.occupied.load(Ordering::Acquire) {
            return Err(Box::new(IpcError::new("`send_request()` failed")));
        }
        let (evt, _) = unsafe { Event::new(self.wait_signal_mem.as_ptr(), true)? };

        // Set the request.
        info.occupied.store(true, Ordering::Release);
        info.request = request;
        info.request_side = RequestSide::Master;

        // Wait the response.
        evt.wait(Timeout::Infinite)?;

        // Get response.
        let response = info.response.take();
        info.occupied.store(false, Ordering::Release);
        info.request_side = RequestSide::None;
        Ok(response)
    }

    #[inline]
    fn try_recv_request(&self) -> Option<M> {
        let info = self.shared_info();
        if info.request_side != RequestSide::Slave {
            return None;
        }

        Some(info.request)
    }

    #[inline]
    fn response_request(&self, response: Option<M>) {
        let _guard = self.mutex.lock();
        let info = self.shared_info();
        if info.request_side != RequestSide::Slave {
            return;
        }

        let info = self.shared_info();
        let (evt, _) = unsafe { Event::from_existing(self.wait_signal_mem.as_ptr()).unwrap() };

        info.response = response;
        info.request_side = RequestSide::None;
        evt.set(EventState::Signaled).unwrap();
    }

    #[inline]
    fn wait(&self, timeout: Timeout) {
        if let Ok((evt, _)) = unsafe { Event::new(self.wait_signal_mem.as_ptr(), true) } {
            if let Err(e) = evt.wait(timeout) {
                error!("Ipc => master context wait failed. {:?}", e)
            }
        }
    }

    #[inline]
    fn signal(&self) {
        if let Ok((evt, _)) = unsafe { Event::from_existing(self.wait_signal_mem.as_ptr()) } {
            if let Err(e) = evt.set(EventState::Signaled) {
                error!("Ipc => master context signal failed. {:?}", e)
            }
        }
    }

    #[inline]
    fn buffer_lock(&self) -> Arc<MemRwLock> {
        self.buffer_lock.clone()
    }

    #[inline]
    fn pretreat_resize(&mut self, width: u32, height: u32) {
        let sinfo = self.shared_info();
        if width == 0 || height == 0 {
            sinfo.resized.store(false, Ordering::Release);
            return;
        }

        sinfo.resized.store(true, Ordering::Release);
        self.pretreat_size = (width, height);
    }

    fn create_buffer(&mut self, width: u32, height: u32) {
        let buffer_name = format!("{}{}_{}", self.name, IPC_MEM_BUFFER_NAME, 0);

        self.buffer = Some(
            ShmemConf::new()
                .size((width * height * 4) as usize)
                .os_id(buffer_name)
                .create()
                .unwrap(),
        );

        let info_data = self.shared_info();
        info_data.width.store(width, Ordering::Release);
        info_data.height.store(height, Ordering::Release);
    }

    fn recreate_buffer(&mut self) -> Option<Shmem> {
        let info_data = unsafe {
            (self.shared_info.as_ptr() as *mut SharedInfo<M>)
                .as_mut()
                .unwrap()
        };
        if !info_data.resized.load(Ordering::Acquire) {
            return None;
        }

        info_data
            .width
            .store(self.pretreat_size.0, Ordering::Release);
        info_data
            .height
            .store(self.pretreat_size.1, Ordering::Release);
        info_data.prepared.store(false, Ordering::SeqCst);

        let name_helper = info_data.name_helper.fetch_add(1, Ordering::Release);

        let buffer_name = format!("{}{}_{}", self.name, IPC_MEM_BUFFER_NAME, name_helper + 1);

        let (width, height) = (self.pretreat_size.0, self.pretreat_size.1);
        let buffer = ShmemConf::new()
            .size((width * height * 4) as usize)
            .os_id(buffer_name)
            .create()
            .unwrap();

        self.buffer.replace(buffer)
    }
}
