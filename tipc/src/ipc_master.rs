use crate::{
    generate_u128,
    ipc_event::IpcEvent,
    mem::{master_context::MasterContext, mem_queue::MemQueueError, MemContext, MAX_REGION_SIZE},
    IpcNode,
};
use core::{panic, slice};
use shared_memory::Shmem;
use std::{collections::VecDeque, error::Error, ffi::c_void, sync::atomic::Ordering};
use tlib::figure::Rect;

pub struct IpcMaster<T: 'static + Copy, M: 'static + Copy> {
    master_context: MasterContext<T, M>,
    retentions: VecDeque<Shmem>,
}

/// SAFETY: MemQueue and memory context use `Mutex` to ensure thread safety.
unsafe impl<T: 'static + Copy, M: 'static + Copy> Send for IpcMaster<T, M> {}
unsafe impl<T: 'static + Copy, M: 'static + Copy> Sync for IpcMaster<T, M> {}

impl<T: 'static + Copy, M: 'static + Copy> IpcMaster<T, M> {
    pub fn new(name: &str) -> Self {
        let master_context = MasterContext::create(name);

        Self {
            // width: width as usize,
            // height: height as usize,
            master_context: master_context,
            retentions: VecDeque::new(),
        }
    }

    pub fn add_rect(&self, id: &'static str, rect: Rect) {
        let id = generate_u128(id).expect(&format!("Invalid id: {}", id));

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
            slice::from_raw_parts_mut(
                self.master_context.buffer(),
                (self.master_context.width() * self.master_context.height() * 4) as usize,
            )
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
        let id = generate_u128(id).expect(&format!("Invalid id: {}", id));

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
    fn pretreat_resize(&mut self, width: u32, height: u32) {
        self.master_context.pretreat_resize(width, height)
    }

    #[inline]
    fn create_buffer(&mut self, width: u32, height: u32) {
        self.master_context.create_buffer(width, height)
    }

    #[inline]
    fn recreate_buffer(&mut self) {
        if let Some(old) = self.master_context.recreate_buffer() {
            self.retentions.push_back(old);
        }
    }

    fn release_retention(&mut self) {
        let shmem_info = self.master_context.shared_info();

        shmem_info
            .release_idx
            .fetch_update(Ordering::Release, Ordering::Acquire, |release_idx| {
                self.retentions
                    .drain(0..release_idx.min(self.retentions.len()));
                Some(0)
            })
            .expect("Shared buffer release retention failed.");
    }

    #[inline]
    fn is_invalidate(&self) -> bool {
        self.master_context.shared_info().invalidate.load(Ordering::Acquire)
    }

    #[inline]
    fn set_invalidate(&self, invalidate: bool) {
        self.master_context.shared_info().invalidate.store(invalidate, Ordering::Release)
    }
}

impl<T: 'static + Copy, M: 'static + Copy> IpcMaster<T, M> {
    #[inline]
    pub fn wait_prepared(&self) {
        while !self
            .master_context
            .shared_info()
            .prepared
            .load(Ordering::Acquire)
        {}
    }
}
