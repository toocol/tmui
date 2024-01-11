use crate::{
    generate_u128,
    ipc_event::IpcEvent,
    mem::{mem_queue::MemQueueError, slave_context::SlaveContext, MemContext},
    IpcNode,
};
use core::slice;
use raw_sync::Timeout;
use shared_memory::Shmem;
use std::{collections::VecDeque, error::Error, ffi::c_void, sync::atomic::Ordering};
use tlib::figure::Rect;

pub struct IpcSlave<T: 'static + Copy, M: 'static + Copy> {
    slave_context: SlaveContext<T, M>,
    retentions: VecDeque<Shmem>,
}

/// SAFETY: MemQueue and memory context use `Mutex` to ensure thread safety.
unsafe impl<T: 'static + Copy, M: 'static + Copy> Send for IpcSlave<T, M> {}
unsafe impl<T: 'static + Copy, M: 'static + Copy> Sync for IpcSlave<T, M> {}

impl<T: 'static + Copy, M: 'static + Copy> IpcSlave<T, M> {
    /// The name should be same with the [`IpcMaster`]
    pub fn new(name: &str) -> Self {
        let slave_context = SlaveContext::open(name);

        Self {
            slave_context: slave_context,
            retentions: VecDeque::new(),
        }
    }
}

impl<T: 'static + Copy, M: 'static + Copy> IpcNode<T, M> for IpcSlave<T, M> {
    #[inline]
    fn name(&self) -> &str {
        self.slave_context.name()
    }

    #[inline]
    fn buffer(&self) -> &'static mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self.slave_context.buffer(),
                (self.slave_context.height() * self.slave_context.width() * 4) as usize,
            )
        }
    }

    #[inline]
    fn buffer_raw_pointer(&self) -> *mut c_void {
        self.slave_context.buffer() as *mut c_void
    }

    #[inline]
    fn try_send(&self, evt: IpcEvent<T>) -> Result<(), MemQueueError> {
        self.slave_context.try_send(evt.into())
    }

    #[inline]
    fn has_event(&self) -> bool {
        self.slave_context.has_event()
    }

    #[inline]
    fn try_recv(&self) -> Option<IpcEvent<T>> {
        self.slave_context.try_recv()
    }

    #[inline]
    fn try_recv_vec(&self) -> Vec<IpcEvent<T>> {
        self.slave_context
            .try_recv_vec()
            .into_iter()
            .map(|e| e.into())
            .collect()
    }

    #[inline]
    fn send_request(&self, rqst: M) -> Result<Option<M>, Box<dyn Error>> {
        self.slave_context.send_request(rqst)
    }

    #[inline]
    fn try_recv_request(&self) -> Option<M> {
        self.slave_context.try_recv_request()
    }

    #[inline]
    fn respose_request(&self, resp: Option<M>) {
        self.slave_context.response_request(resp)
    }

    #[inline]
    fn terminate(&self) {}

    #[inline]
    fn wait(&self, timeout: Timeout) {
        self.slave_context.wait(timeout)
    }

    #[inline]
    fn signal(&self) {
        self.slave_context.signal()
    }

    #[inline]
    fn region(&self, id: &'static str) -> Option<Rect> {
        let id = generate_u128(id).expect(&format!("Invalid id: {}", id));

        let shared_info = self.slave_context.shared_info();
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
        self.slave_context.width()
    }

    #[inline]
    fn height(&self) -> u32 {
        self.slave_context.height()
    }

    #[inline]
    fn buffer_lock(&self) -> std::sync::Arc<crate::mem::mem_rw_lock::MemRwLock> {
        self.slave_context.buffer_lock()
    }

    #[inline]
    fn ty(&self) -> crate::IpcType {
        crate::IpcType::Slave
    }

    #[inline]
    fn pretreat_resize(&mut self, width: u32, height: u32) {
        self.slave_context.pretreat_resize(width, height)
    }

    #[inline]
    fn create_buffer(&mut self, _: u32, _: u32) {
        unreachable!()
    }

    #[inline]
    fn recreate_buffer(&mut self) {
        if let Some(old) = self.slave_context.recreate_buffer() {
            self.retentions.push_back(old);
        }
    }

    fn release_retention(&mut self) {
        let shmem_info = self.slave_context.shared_info();

        self.retentions.pop_front();

        shmem_info.release_idx.fetch_add(1, Ordering::Release);
    }

    #[inline]
    fn is_invalidate(&self) -> bool {
        self.slave_context
            .shared_info()
            .invalidate
            .load(Ordering::Acquire)
    }

    #[inline]
    fn set_invalidate(&self, invalidate: bool) {
        self.slave_context
            .shared_info()
            .invalidate
            .store(invalidate, Ordering::Release)
    }
}

impl<T: 'static + Copy, M: 'static + Copy> IpcSlave<T, M> {
    #[inline]
    pub fn prepared(&self) {
        self.slave_context
            .shared_info()
            .prepared
            .store(true, Ordering::Release);
    }
}
