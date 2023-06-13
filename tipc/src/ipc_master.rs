use crate::{
    ipc_event::IpcEvent,
    mem::{master_context::MasterContext, mem_queue::MemQueueError, MemContext},
    IpcNode,
};
use core::slice;
use std::{error::Error, ffi::c_void};
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
}

impl<T: 'static + Copy, M: 'static + Copy> Drop for IpcMaster<T, M> {
    fn drop(&mut self) {
        self.terminate()
    }
}

impl<T: 'static + Copy, M: 'static + Copy> IpcNode<T, M> for IpcMaster<T, M> {
    #[inline]
    fn primary_buffer(&self) -> &'static mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self.master_context.primary_buffer(),
                self.height * self.width * 4,
            )
        }
    }

    #[inline]
    fn secondary_buffer(&self) -> &'static mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self.master_context.secondary_buffer(),
                self.height * self.width * 4,
            )
        }
    }

    #[inline]
    fn primary_buffer_raw_pointer(&self) -> *mut c_void {
        self.master_context.primary_buffer() as *mut c_void
    }

    #[inline]
    fn secondary_buffer_raw_pointer(&self) -> *mut c_void {
        self.master_context.primary_buffer() as *mut c_void
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
    fn region(&self) -> Rect {
        self.master_context.shared_info().region.as_rect()
    }
}
