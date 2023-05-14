use crate::{
    ipc_event::IpcEvent,
    mem::{mem_queue::MemQueueError, slave_context::SlaveContext, MemContext},
};
use core::slice;
use std::{error::Error, ffi::c_void};

pub struct IpcSlave<T: 'static + Copy, M: 'static + Copy> {
    width: usize,
    height: usize,
    primary_buffer_raw_pointer: *mut u8,
    secondary_buffer_raw_pointer: *mut u8,
    slave_context: SlaveContext<T, M>,
}

impl<T: 'static + Copy, M: 'static + Copy> IpcSlave<T, M> {
    /// The name should be same with the [`IpcMaster`]
    pub fn new(name: &str) -> Self {
        let slave_context = SlaveContext::open(name);
        let width = slave_context.width();
        let height = slave_context.height();

        Self {
            width: width as usize,
            height: height as usize,
            primary_buffer_raw_pointer: slave_context.primary_buffer(),
            secondary_buffer_raw_pointer: slave_context.secondary_buffer(),
            slave_context: slave_context,
        }
    }

    #[inline]
    pub fn primary_buffer(&self) -> &'static mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self.primary_buffer_raw_pointer,
                self.height * self.width * 4,
            )
        }
    }

    #[inline]
    pub fn secondary_buffer(&self) -> &'static mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self.secondary_buffer_raw_pointer,
                self.height * self.width * 4,
            )
        }
    }

    #[inline]
    pub fn primary_buffer_raw_pointer(&self) -> *mut c_void {
        self.primary_buffer_raw_pointer as *mut c_void
    }

    #[inline]
    pub fn secondary_buffer_raw_pointer(&self) -> *mut c_void {
        self.secondary_buffer_raw_pointer as *mut c_void
    }

    #[inline]
    pub fn try_send(&self, evt: IpcEvent<T>) -> Result<(), MemQueueError> {
        self.slave_context.try_send(evt.into())
    }

    #[inline]
    pub fn has_event(&self) -> bool {
        self.slave_context.has_event()
    }

    #[inline]
    pub fn try_recv(&self) -> Option<IpcEvent<T>> {
        self.slave_context.try_recv()
    }

    #[inline]
    pub fn try_recv_vec(&self) -> Vec<IpcEvent<T>> {
        self.slave_context
            .try_recv_vec()
            .into_iter()
            .map(|e| e.into())
            .collect()
    }

    #[inline]
    pub fn send_request(&self, rqst: M) -> Result<Option<M>, Box<dyn Error>> {
        self.slave_context.send_request(rqst)
    }

    #[inline]
    pub fn try_recv_request(&self) -> Option<M> {
        self.slave_context.try_recv_request()
    }

    #[inline]
    pub fn respose_request(&self, resp: Option<M>) {
        self.slave_context.response_request(resp)
    }

    #[inline]
    pub fn terminate_at(&self) -> bool {
        todo!()
    }
}
