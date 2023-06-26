use tlib::figure::Rect;
use self::mem_queue::MemQueueError;
use crate::ipc_event::IpcEvent;
use std::{
    error::Error,
    fmt::Display,
    sync::atomic::{AtomicBool, AtomicU32, AtomicUsize}, mem::MaybeUninit,
};

pub mod master_context;
pub mod mem_queue;
pub mod slave_context;

pub(crate) const IPC_QUEUE_SIZE: usize = 10000;
pub(crate) const MAX_REGION_SIZE: usize = 10;

pub(crate) const IPC_KEY_EVT_SIZE: usize = 8;
pub(crate) const IPC_TEXT_EVT_SIZE: usize = 4096;

pub(crate) const IPC_MEM_PRIMARY_BUFFER_NAME: &'static str = "_mem_bf";
pub(crate) const IPC_MEM_SHARED_INFO_NAME: &'static str = "_mem_sh_info";
pub(crate) const IPC_MEM_MASTER_QUEUE: &'static str = "_mem_m_q";
pub(crate) const IPC_MEM_SLAVE_QUEUE: &'static str = "_mem_s_q";
pub(crate) const IPC_MEM_SIGNAL_EVT: &'static str = "_mem_e_s";

pub(crate) trait MemContext<T: 'static + Copy, M: 'static + Copy> {
    fn buffer(&self) -> *mut u8;

    fn width(&self) -> u32;

    fn height(&self) -> u32;

    fn try_send(&self, evt: IpcEvent<T>) -> Result<(), MemQueueError>;

    fn has_event(&self) -> bool;

    fn try_recv(&self) -> Option<IpcEvent<T>>;

    fn try_recv_vec(&self) -> Vec<IpcEvent<T>>;

    fn send_request(&self, request: M) -> Result<Option<M>, Box<dyn Error>>;

    fn try_recv_request(&self) -> Option<M>;

    fn response_request(&self, response: Option<M>);

    fn wait(&self);

    fn signal(&self);
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum RequestSide {
    None,
    Master,
    Slave,
}

#[repr(C)]
pub(crate) struct SharedInfo<M: 'static + Copy> {
    /// The size of application.
    pub(crate) width: AtomicU32,
    pub(crate) height: AtomicU32,

    /// The clip region to renderer in slave.
    pub(crate) regions: [MaybeUninit<(u64, Rect)>; MAX_REGION_SIZE],
    pub(crate) region_idx: AtomicUsize,

    pub(crate) occupied: AtomicBool,
    pub(crate) request_side: RequestSide,
    pub(crate) request: M,
    pub(crate) response: Option<M>,
}

#[derive(Debug)]
pub struct IpcError {
    msg: &'static str,
}
impl IpcError {
    pub fn new(msg: &'static str) -> Self {
        Self { msg }
    }
}
impl Error for IpcError {}
impl Display for IpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg)
    }
}
