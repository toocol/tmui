use self::{mem_queue::MemQueueError, mem_rw_lock::MemRwLock};
use crate::ipc_event::IpcEvent;
use shared_memory::Shmem;
use std::{
    error::Error,
    fmt::Display,
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicBool, AtomicU32, AtomicUsize},
        Arc,
    },
};
use tlib::figure::Rect;

pub mod master_context;
pub mod mem_mutex;
pub mod mem_queue;
pub mod mem_rw_lock;
pub mod slave_context;

pub(crate) const IPC_QUEUE_SIZE: usize = 10000;
pub(crate) const MAX_REGION_SIZE: usize = 10;

pub(crate) const IPC_MEM_BUFFER_NAME: &'static str = "_mem_bf";
pub(crate) const IPC_MEM_SHARED_INFO_NAME: &'static str = "_mem_sh_info";
pub(crate) const IPC_MEM_LOCK_NAME: &'static str = "_mem_rwl";
pub(crate) const IPC_MEM_MASTER_QUEUE: &'static str = "_mem_m_q";
pub(crate) const IPC_MEM_SLAVE_QUEUE: &'static str = "_mem_s_q";
pub(crate) const IPC_MEM_SIGNAL_EVT: &'static str = "_mem_e_s";

pub(crate) trait MemContext<T: 'static + Copy, M: 'static + Copy> {
    fn name(&self) -> &str;

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

    fn buffer_lock(&self) -> Arc<MemRwLock>;

    fn pretreat_resize(&mut self, width: u32, height: u32);

    fn create_buffer(&mut self, width: u32, height: u32);

    fn recreate_buffer(&mut self) -> Option<Shmem>;
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum RequestSide {
    None,
    Master,
    Slave,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum BuildType {
    #[default]
    Create,
    Open,
}

#[repr(C)]
pub(crate) struct SharedInfo<M: 'static + Copy> {
    pub(crate) name_helper: AtomicU32,

    /// The size of application.
    pub(crate) width: AtomicU32,
    pub(crate) height: AtomicU32,

    /// The clip region to renderer in slave.
    pub(crate) regions: [MaybeUninit<(u128, Rect)>; MAX_REGION_SIZE],
    pub(crate) region_idx: AtomicUsize,

    pub(crate) occupied: AtomicBool,
    pub(crate) request_side: RequestSide,
    pub(crate) request: M,
    pub(crate) response: Option<M>,

    pub(crate) resized: AtomicBool,
    pub(crate) release_idx: AtomicUsize,
    pub(crate) prepared: AtomicBool,
    pub(crate) invalidate: AtomicBool,
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
