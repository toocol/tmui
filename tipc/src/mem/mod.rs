use std::sync::atomic::AtomicU32;

pub mod mem_queue;
pub mod master_context;
pub mod slave_context;

pub const IPC_SHARED_MSG_SIZE: usize = 4096;
pub const IPC_KEY_EVT_SIZE: usize = 8;
pub const IPC_NATIVE_EVT_SIZE: usize = 1024;

pub(crate) trait MemContext {
    fn get_primary_buffer(&self) -> *mut u8;

    fn get_secodary_buffer(&self) -> *mut u8;

    fn send_shared_message(&mut self, message: &str, message_type: i32) -> String;

    fn try_recv_shared_message(&self) -> (String, i32);

    fn response_shared_message(&mut self);
}

#[repr(C)]
pub(crate) enum SharedMsgSide {
    Master,
    Slave,
}

pub(crate) struct SharedMem {
    width: AtomicU32,
    height: AtomicU32,

    msg_side: SharedMsgSide,
    shared_msg: [u8; IPC_SHARED_MSG_SIZE],
    shared_res: [u8; IPC_SHARED_MSG_SIZE],
}