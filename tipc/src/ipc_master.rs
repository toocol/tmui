use crate::{
    ipc_event::IpcEvent,
    mem::{master_context::MasterContext, mem_queue::MemQueueError, MemContext, MAX_REGION_SIZE},
    IpcNode,
};
use core::{panic, slice};
use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    ffi::c_void,
    hash::{Hash, Hasher},
    sync::atomic::Ordering,
};
use tlib::figure::Rect;

pub struct IpcMaster<T: 'static + Copy, M: 'static + Copy> {
    width: usize,
    height: usize,
    master_context: MasterContext<T, M>,
}

/// SAFETY: MemQueue and memory context use `Mutex` to ensure thread safety.
unsafe impl<T: 'static + Copy, M: 'static + Copy> Send for IpcMaster<T, M> {}
unsafe impl<T: 'static + Copy, M: 'static + Copy> Sync for IpcMaster<T, M> {}

impl<T: 'static + Copy, M: 'static + Copy> IpcMaster<T, M> {
    pub fn new(name: &str, width: u32, height: u32) -> Self {
        let master_context = MasterContext::create(name, width, height);

        Self {
            width: width as usize,
            height: height as usize,
            master_context: master_context,
        }
    }

    pub fn add_rect(&self, id: &'static str, rect: Rect) {
        let mut hasher = DefaultHasher::default();
        id.hash(&mut hasher);
        let id = hasher.finish();

        let shared_ifo = self.master_context.shared_info();
        let idx = shared_ifo.region_idx.load(Ordering::Acquire);
        for uninit in shared_ifo.regions[..idx].iter_mut() {
            let (sid, r) = unsafe { uninit.assume_init_mut() };
            if *sid == id {
                *r = rect;
                return;
            }
        }
        if idx >= MAX_REGION_SIZE {
            panic!("Only support {} `SharedWidget`", MAX_REGION_SIZE);
        }
        shared_ifo.regions[idx].write((id, rect));
        shared_ifo.region_idx.fetch_add(1, Ordering::Release);
    }
}

impl<T: 'static + Copy, M: 'static + Copy> Drop for IpcMaster<T, M> {
    fn drop(&mut self) {
        self.terminate()
    }
}

impl<T: 'static + Copy, M: 'static + Copy> IpcNode<T, M> for IpcMaster<T, M> {
    #[inline]
    fn name(&self) -> &str {
        self.master_context.name()
    }

    #[inline]
    fn buffer(&self) -> &'static mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self.master_context.buffer(), self.height * self.width * 4)
        }
    }

    #[inline]
    fn buffer_raw_pointer(&self) -> *mut c_void {
        self.master_context.buffer() as *mut c_void
    }

    #[inline]
    fn try_send(&self, evt: IpcEvent<T>) -> Result<(), MemQueueError> {
        self.master_context.try_send(evt.into())
    }

    #[inline]
    fn has_event(&self) -> bool {
        self.master_context.has_event()
    }

    #[inline]
    fn try_recv(&self) -> Option<IpcEvent<T>> {
        self.master_context.try_recv()
    }

    #[inline]
    fn try_recv_vec(&self) -> Vec<IpcEvent<T>> {
        self.master_context
            .try_recv_vec()
            .into_iter()
            .map(|e| e.into())
            .collect()
    }

    #[inline]
    fn send_request(&self, rqst: M) -> Result<Option<M>, Box<dyn Error>> {
        self.master_context.send_request(rqst)
    }

    #[inline]
    fn try_recv_request(&self) -> Option<M> {
        self.master_context.try_recv_request()
    }

    #[inline]
    fn respose_request(&self, resp: Option<M>) {
        self.master_context.response_request(resp)
    }

    #[inline]
    fn terminate(&self) {}

    #[inline]
    fn wait(&self) {
        self.master_context.wait()
    }

    #[inline]
    fn signal(&self) {
        self.master_context.signal()
    }

    #[inline]
    fn region(&self, id: &'static str) -> Option<Rect> {
        let mut hasher = DefaultHasher::default();
        id.hash(&mut hasher);
        let id = hasher.finish();

        let shared_info = self.master_context.shared_info();
        let idx = shared_info.region_idx.load(Ordering::Acquire);
        for uninit in shared_info.regions[..idx].iter() {
            let (sid, r) = unsafe { uninit.assume_init_ref() };
            if *sid == id {
                return Some(*r);
            }
        }
        None
    }

    #[inline]
    fn width(&self) -> u32 {
        self.master_context.width()
    }

    #[inline]
    fn height(&self) -> u32 {
        self.master_context.height()
    }

    #[inline]
    fn buffer_lock(&self) -> std::sync::Arc<crate::mem::mem_rw_lock::MemRwLock> {
        self.master_context.buffer_lock()
    }

    #[inline]
    fn ty(&self) -> crate::IpcType {
        crate::IpcType::Master
    }

    #[inline]
    fn resize(&mut self, width: u32, height: u32) -> shared_memory::Shmem {
        self.master_context.resize(width, height)
    }
}
