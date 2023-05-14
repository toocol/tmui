use crate::{
    ipc_event::IpcEvent,
    mem::{master_context::MasterContext, mem_queue::MemQueueError, MemContext},
};
use core::slice;
use std::{error::Error, ffi::c_void};

pub struct IpcMaster<T: 'static + Copy, M: 'static + Copy> {
    width: usize,
    height: usize,
    primary_buffer_raw_pointer: *mut u8,
    secondary_buffer_raw_pointer: *mut u8,
    master_context: MasterContext<T, M>,
}

impl<T: 'static + Copy, M: 'static + Copy> IpcMaster<T, M> {
    pub fn new(name: &str, width: u32, height: u32) -> Self {
        let master_context = MasterContext::create(name, width, height);

        Self {
            width: width as usize,
            height: height as usize,
            primary_buffer_raw_pointer: master_context.primary_buffer(),
            secondary_buffer_raw_pointer: master_context.secondary_buffer(),
            master_context: master_context,
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
        self.master_context.try_send(evt.into())
    }

    #[inline]
    pub fn has_event(&self) -> bool {
        self.master_context.has_event()
    }

    #[inline]
    pub fn try_recv(&self) -> Option<IpcEvent<T>> {
        self.master_context.try_recv()
    }

    #[inline]
    pub fn try_recv_vec(&self) -> Vec<IpcEvent<T>> {
        self.master_context
            .try_recv_vec()
            .into_iter()
            .map(|e| e.into())
            .collect()
    }

    #[inline]
    pub fn send_request(&self, rqst: M) -> Result<Option<M>, Box<dyn Error>> {
        self.master_context.send_request(rqst)
    }

    #[inline]
    pub fn try_recv_request(&self) -> Option<M> {
        self.master_context.try_recv_request()
    }

    #[inline]
    pub fn respose_request(&self, resp: Option<M>) {
        self.master_context.response_request(resp)
    }

    #[inline]
    fn terminate(&self) {}
}

impl<T: 'static + Copy, M: 'static + Copy> Drop for IpcMaster<T, M> {
    fn drop(&mut self) {
        self.terminate()
    }
}
