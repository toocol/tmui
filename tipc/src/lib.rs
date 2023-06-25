use ipc_event::IpcEvent;
use ipc_master::IpcMaster;
use ipc_slave::IpcSlave;
use mem::mem_queue::MemQueueError;
use std::{error::Error, ffi::c_void, marker::PhantomData};
use tlib::figure::Rect;

pub mod ipc_event;
pub mod ipc_master;
pub mod ipc_slave;
pub mod mem;

pub struct IpcBuilder<T: 'static + Copy, M: 'static + Copy> {
    name: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    _user_event: PhantomData<T>,
    _request_response: PhantomData<M>,
}

impl IpcBuilder<(), ()> {
    #[inline]
    pub fn new() -> Self {
        Self::with_customize()
    }
}

impl<T: 'static + Copy, M: 'static + Copy> IpcBuilder<T, M> {
    #[inline]
    pub fn with_customize() -> Self {
        Self {
            name: None,
            width: None,
            height: None,
            _user_event: Default::default(),
            _request_response: Default::default(),
        }
    }

    #[inline]
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    #[inline]
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    #[inline]
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    #[inline]
    pub fn master(mut self) -> IpcMaster<T, M> {
        IpcMaster::<T, M>::new(
            self.name
                .as_ref()
                .expect("Build IpcMaster require `name` not none."),
            self.width
                .take()
                .expect("Build IpcMaster require `width` not none."),
            self.height
                .take()
                .expect("Build IpcMaster require `height` not none."),
        )
    }

    #[inline]
    pub fn slave(self) -> IpcSlave<T, M> {
        IpcSlave::<T, M>::new(
            self.name
                .as_ref()
                .expect("Build IpcSlave require `name` not none."),
        )
    }
}

pub trait WithIpcMaster<T: 'static + Copy, M: 'static + Copy> {
    fn create_ipc_master(name: &'static str, width: u32, height: u32) -> IpcMaster<T, M> {
        IpcBuilder::<T, M>::with_customize()
            .name(name)
            .width(width)
            .height(height)
            .master()
    }

    fn with_ipc_master(&mut self, name: &'static str, width: u32, height: u32) {
        self.proc_ipc_master(Self::create_ipc_master(name, width, height))
    }

    fn proc_ipc_master(&mut self, master: IpcMaster<T, M>);
}

pub trait WithIpcSlave<T: 'static + Copy, M: 'static + Copy> {
    fn create_ipc_slave(name: &'static str) -> IpcSlave<T, M> {
        IpcBuilder::<T, M>::with_customize().name(name).slave()
    }

    fn with_ipc_slave(&mut self, name: &'static str) {
        self.proc_ipc_slave(Self::create_ipc_slave(name))
    }

    fn proc_ipc_slave(&mut self, slave: IpcSlave<T, M>);
}

pub trait IpcNode<T: 'static + Copy, M: 'static + Copy> {
    fn buffer(&self) -> &'static mut [u8];

    fn buffer_raw_pointer(&self) -> *mut c_void;

    fn try_send(&self, evt: IpcEvent<T>) -> Result<(), MemQueueError>;

    fn has_event(&self) -> bool;

    fn try_recv(&self) -> Option<IpcEvent<T>>;

    fn try_recv_vec(&self) -> Vec<IpcEvent<T>>;

    fn send_request(&self, rqst: M) -> Result<Option<M>, Box<dyn Error>>;

    fn try_recv_request(&self) -> Option<M>;

    fn respose_request(&self, resp: Option<M>);

    fn terminate(&self);

    fn wait(&self);

    fn signal(&self);

    fn regions(&self) -> &[Rect];

    fn width(&self) -> u32;

    fn height(&self) -> u32;
}
