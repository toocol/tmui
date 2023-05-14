use super::{
    mem_queue::{MemQueue, MemQueueError},
    MemContext, IPC_MEM_SIGNAL_EVT, IPC_QUEUE_SIZE,
};
use crate::{
    ipc_event::{InnerIpcEvent, IpcEvent},
    mem::{
        mem_queue::{BuildType, MemQueueBuilder},
        IpcError, RequestSide, SharedInfo, IPC_MEM_MASTER_QUEUE, IPC_MEM_PRIMARY_BUFFER_NAME,
        IPC_MEM_SECONDARY_BUFFER_NAME, IPC_MEM_SHARED_INFO_NAME, IPC_MEM_SLAVE_QUEUE,
    },
};
use raw_sync::{
    events::{Event, EventInit, EventState},
    Timeout,
};
use shared_memory::{Shmem, ShmemConf};
use std::{error::Error, marker::PhantomData, mem::size_of, sync::atomic::Ordering};

pub(crate) struct MasterContext<T: 'static + Copy, M: 'static + Copy> {
    primary_buffer: Shmem,
    secondary_buffer: Shmem,
    shared_info: Shmem,
    event_signal_mem: Shmem,
    master_queue: MemQueue<IPC_QUEUE_SIZE, InnerIpcEvent<T>>,
    slave_queue: MemQueue<IPC_QUEUE_SIZE, InnerIpcEvent<T>>,
    _request_type: PhantomData<M>,
}

impl<T: 'static + Copy, M: 'static + Copy> MasterContext<T, M> {
    pub(crate) fn create<P: ToString>(name: P, width: u32, height: u32) -> Self {
        let mut primary_buffer_name = name.to_string();
        primary_buffer_name.push_str(IPC_MEM_PRIMARY_BUFFER_NAME);
        let primary_buffer = ShmemConf::new()
            .size((width * height * 4) as usize)
            .os_id(primary_buffer_name)
            .create()
            .unwrap();

        let mut secondary_buffer_name = name.to_string();
        secondary_buffer_name.push_str(IPC_MEM_SECONDARY_BUFFER_NAME);
        let secondary_buffer = ShmemConf::new()
            .size((width * height * 4) as usize)
            .os_id(secondary_buffer_name)
            .create()
            .unwrap();

        let mut shared_info_name = name.to_string();
        shared_info_name.push_str(IPC_MEM_SHARED_INFO_NAME);
        let shared_info = ShmemConf::new()
            .size(size_of::<SharedInfo<M>>())
            .os_id(shared_info_name)
            .create()
            .unwrap();
        let info_data = unsafe {
            (shared_info.as_ptr() as *mut SharedInfo<M>)
                .as_mut()
                .unwrap()
        };
        info_data.width.store(width, Ordering::SeqCst);
        info_data.height.store(height, Ordering::SeqCst);

        let mut event_signal_name = name.to_string();
        event_signal_name.push_str(IPC_MEM_SIGNAL_EVT);
        let event_signal_mem = ShmemConf::new()
            .size(size_of::<Event>())
            .os_id(event_signal_name)
            .create()
            .unwrap();

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
            primary_buffer,
            secondary_buffer,
            shared_info,
            event_signal_mem,
            master_queue,
            slave_queue,
            _request_type: Default::default(),
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
    fn primary_buffer(&self) -> *mut u8 {
        self.primary_buffer.as_ptr()
    }

    #[inline]
    fn secondary_buffer(&self) -> *mut u8 {
        self.secondary_buffer.as_ptr()
    }

    #[inline]
    fn width(&self) -> u32 {
        self.shared_info().width.load(Ordering::Relaxed)
    }

    #[inline]
    fn height(&self) -> u32 {
        self.shared_info().height.load(Ordering::Relaxed)
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
        match self.slave_queue.try_read() {
            Some(ipc_evt) => Some(ipc_evt.into()),
            None => None
        }
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
        let info = self.shared_info();
        if info.occupied.load(Ordering::Acquire) {
            return Err(Box::new(IpcError::new("`send_request()` failed")));
        }
        let (evt, _) = unsafe { Event::new(self.event_signal_mem.as_ptr(), true)? };

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
        let info = self.shared_info();
        if info.request_side != RequestSide::Slave {
            return;
        }

        let info = self.shared_info();
        let (evt, _) = unsafe { Event::from_existing(self.event_signal_mem.as_ptr()).unwrap() };

        info.response = response;
        info.request_side = RequestSide::None;
        evt.set(EventState::Signaled).unwrap();
    }
}
