use core::slice;
use std::ffi::c_void;

use crate::{
    ipc_channel::{self, IpcReceiver, IpcSender},
    native::IpcAdapter, ipc_event::IpcEvent,
};

pub struct IpcSlave {
    _id: i32,
    width: usize,
    height: usize,
    primary_buffer_raw_pointer: *mut u8,
    secondary_buffer_raw_pointer: *mut u8,
    sender: IpcSender,
    receiver: IpcReceiver,
}

impl IpcSlave {
    /// The name should be same with the [`IpcMaster`]
    pub fn new(name: &str) -> Self {
        let id = IpcAdapter::connect_to(name);
        let width = IpcAdapter::get_w(id);
        let height = IpcAdapter::get_h(id);
        let primary_buffer_raw_pointer = IpcAdapter::get_primary_buffer(id);
        let secondary_buffer_raw_pointer = IpcAdapter::get_secondary_buffer(id);

        let (sender, receiver) = ipc_channel::channel(id, ipc_channel::ChannelType::Slave);

        Self {
            _id: id,
            width: width as usize,
            height: height as usize,
            primary_buffer_raw_pointer,
            secondary_buffer_raw_pointer,
            sender,
            receiver,
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
    pub fn send(&self, evt: IpcEvent) {
        self.sender.send(evt)
    }

    #[inline]
    pub fn recv(&self) -> IpcEvent {
        self.receiver.recv()
    }

    #[inline]
    pub fn try_recv(&self) -> IpcEvent {
        self.receiver.try_recv()
    }
}
