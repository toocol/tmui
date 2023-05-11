use std::sync::atomic::AtomicU32;

use crate::ipc_event::InnerIpcEvent;

use self::mem_queue::MemQueueError;

pub mod mem_queue;
pub mod master_context;
pub mod slave_context;

pub(crate) const IPC_QUEUE_SIZE: usize = 10000;

pub(crate) const IPC_KEY_EVT_SIZE: usize = 8;
pub(crate) const IPC_NATIVE_EVT_SIZE: usize = 1024;
pub(crate) const IPC_SHARED_MSG_SIZE: usize = 4096;

pub(crate) const IPC_MEM_PRIMARY_BUFFER_NAME: &'static str = "_mem_primary_buffer_";
pub(crate) const IPC_MEM_SECONDARY_BUFFER_NAME: &'static str = "_mem_primary_buffer_";
pub(crate) const IPC_MEM_SHARED_INFO_NAME: &'static str = "_mem_shared_info_";
pub(crate) const IPC_MEM_MASTER_QUEUE: &'static str = "_mem_master_queue_";
pub(crate) const IPC_MEM_SLAVE_QUEUE: &'static str = "_mem_slave_queue_";

pub(crate) trait MemContext {
    fn primary_buffer(&self) -> *mut u8;

    fn secondary_buffer(&self) -> *mut u8;

    fn width(&self) -> u32;

    fn height(&self) -> u32;

    fn try_send(&self, evt: InnerIpcEvent) -> Result<(), MemQueueError>;

    fn try_recv(&self) -> Vec<InnerIpcEvent>;
}

#[repr(C)]
pub(crate) enum SharedMsgSide {
    Master,
    Slave,
}

pub(crate) struct SharedInfo {
    pub(crate) width: AtomicU32,
    pub(crate) height: AtomicU32,

    pub(crate) msg_side: SharedMsgSide,
    pub(crate) shared_msg: [u8; IPC_SHARED_MSG_SIZE],
    pub(crate) shared_res: [u8; IPC_SHARED_MSG_SIZE],
}