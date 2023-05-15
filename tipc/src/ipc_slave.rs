use crate::{
    ipc_event::IpcEvent,
    mem::{mem_queue::MemQueueError, slave_context::SlaveContext, MemContext},
    IpcNode,
};
use core::slice;
use std::{error::Error, ffi::c_void};

pub struct IpcSlave<T: 'static + Copy, M: 'static + Copy> {
    width: usize,
    height: usize,
    slave_context: SlaveContext<T, M>,
}

/// SAFETY: Two different shared memory queues are used for sending and receiving ipc context.
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

impl<T: 'static + Copy, M: 'static + Copy> IpcNode<T, M>
    for IpcSlave<T, M>
{
    #[inline]
    fn primary_buffer(&self) -> &'static mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self.slave_context.primary_buffer(),
                self.height * self.width * 4,
            )
        }
    }

    #[inline]
    fn secondary_buffer(&self) -> &'static mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self.slave_context.secondary_buffer(),
                self.height * self.width * 4,
            )
        }
    }

    #[inline]
    fn primary_buffer_raw_pointer(&self) -> *mut c_void {
        self.slave_context.primary_buffer() as *mut c_void
    }

    #[inline]
    fn secondary_buffer_raw_pointer(&self) -> *mut c_void {
        self.slave_context.secondary_buffer() as *mut c_void
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
}
