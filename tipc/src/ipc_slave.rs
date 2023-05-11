use core::slice;
use std::{ffi::c_void, error::Error};

use crate::{
    ipc_event::IpcEvent, mem::{slave_context::SlaveContext, MemContext},
};

pub struct IpcSlave {
    width: usize,
    height: usize,
    primary_buffer_raw_pointer: *mut u8,
    secondary_buffer_raw_pointer: *mut u8,
    slave_context: SlaveContext,
}

impl IpcSlave {
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
    pub fn try_send(&self, evt: IpcEvent) {
        self.slave_context.try_send(evt.into()).unwrap()
    }

    #[inline]
    pub fn try_recv(&self) -> Vec<IpcEvent> {
        self.slave_context
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
    pub fn respose_shared_msg(id: i32, resp: Option<&str>) {
        todo!()
    }

    #[inline]
    pub fn terminate_at(id: i32) -> bool {
        todo!()
    }
}
