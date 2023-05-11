use crate::{
    ipc_channel::{self, IpcError, IpcReceiver, IpcSender},
    ipc_event::IpcEvent,
    native::IpcAdapter,
};
use core::slice;
use std::ffi::c_void;

pub struct IpcMaster {
    id: i32,
    width: usize,
    height: usize,
    primary_buffer_raw_pointer: *mut u8,
    secondary_buffer_raw_pointer: *mut u8,
    sender: IpcSender,
    receiver: IpcReceiver,
}

impl IpcMaster {
    pub fn new(name: &str, width: u32, height: u32) -> Self {
        let id = IpcAdapter::create_master_context(name, width as i32, height as i32);
        let primary_buffer_ptr = IpcAdapter::get_primary_buffer_master(id);
        let secondary_buffer_ptr = IpcAdapter::get_secondary_buffer_master(id);

        let (sender, receiver) = ipc_channel::channel(id, ipc_channel::IpcType::Master);

        Self {
            id,
            width: width as usize,
            height: height as usize,
            primary_buffer_raw_pointer: primary_buffer_ptr,
            secondary_buffer_raw_pointer: secondary_buffer_ptr,
            sender,
            receiver,
        }
    }

    #[inline]
    pub fn id(&self) -> i32 {
        self.id
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
    pub fn send_shared_message(&self, evt: IpcEvent) -> Result<String, IpcError> {
        self.sender.send_shared_message(evt)
    }

    #[inline]
    pub fn has_event(&self) -> bool {
        self.receiver.has_event()
    }

    #[inline]
    pub fn recv(&self) -> IpcEvent {
        self.receiver.recv()
    }

    #[inline]
    pub fn try_recv(&self) -> IpcEvent {
        self.receiver.try_recv()
    }

    #[inline]
    pub fn try_recv_shared_message(&self) -> Option<String> {
        self.receiver.try_recv_shared_message()
    }

    #[inline]
    fn terminate(&self) {
        IpcAdapter::terminate_by_master(self.id);
    }

    #[inline]
    pub fn respose_shared_msg(id: i32, resp: Option<&str>) {
        IpcAdapter::resp_shared_msg_master(id, if resp.is_some() { resp.unwrap() } else { "" })
    }
}

impl Drop for IpcMaster {
    fn drop(&mut self) {
        self.terminate()
    }
}
