use crate::{
    generate_u128,
    mem::{mem_queue::MemQueueError, slave_context::SlaveContext, MemContext},
    IpcNode, ipc_event::IpcEvent,
};
use core::slice;
use std::{error::Error, ffi::c_void, sync::atomic::Ordering};
use tlib::figure::Rect;

pub struct IpcSlave<T: 'static + Copy, M: 'static + Copy> {
    width: usize,
    height: usize,
    slave_context: SlaveContext<T, M>,
}

/// SAFETY: MemQueue and memory context use `Mutex` to ensure thread safety.
unsafe impl<T: 'static + Copy, M: 'static + Copy> Send for IpcSlave<T, M> {}
unsafe impl<T: 'static + Copy, M: 'static + Copy> Sync for IpcSlave<T, M> {}

impl<T: 'static + Copy, M: 'static + Copy> IpcSlave<T, M> {
    /// The name should be same with the [`IpcMaster`]
    pub fn new(name: &str) -> Self {
        let slave_context = SlaveContext::open(name);
        let width = slave_context.width();
        let height = slave_context.height();

        Self {
            width: width as usize,
            height: height as usize,
            slave_context: slave_context,
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
            slice::from_raw_parts_mut(self.slave_context.buffer(), self.height * self.width * 4)
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
    fn wait(&self) {
        self.slave_context.wait()
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
    fn resize(&mut self, width: u32, height: u32) -> shared_memory::Shmem {
        self.slave_context.resize(width, height)
    }
}
