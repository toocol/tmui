use crate::{
    ipc_event::IpcEvent,
    mem::{
        master_context::{self, MasterContext},
        MemContext,
    },
};
use core::slice;
use std::{error::Error, ffi::c_void};

pub struct IpcMaster {
    width: usize,
    height: usize,
    primary_buffer_raw_pointer: *mut u8,
    secondary_buffer_raw_pointer: *mut u8,
    master_context: MasterContext,
}

impl IpcMaster {
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
    pub fn try_send(&self, evt: IpcEvent) {
        self.master_context.try_send(evt.into()).unwrap()
    }

    #[inline]
    pub fn try_recv(&self) -> Vec<IpcEvent> {
        self.master_context
            .try_recv()
            .into_iter()
            .map(|e| e.into())
            .collect()
    }

    #[inline]
    pub fn send_shared_message(&self, evt: IpcEvent) -> Result<String, Box<dyn Error>> {
        todo!()
    }

    #[inline]
    pub fn try_recv_shared_message(&self) -> Option<String> {
        todo!()
    }

    #[inline]
    fn terminate(&self) {
        todo!()
    }

    #[inline]
    pub fn respose_shared_msg(id: i32, resp: Option<&str>) {
        todo!()
    }
}

impl Drop for IpcMaster {
    fn drop(&mut self) {
        self.terminate()
    }
}
