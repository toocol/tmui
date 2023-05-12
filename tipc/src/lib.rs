use ipc_master::IpcMaster;
use ipc_slave::IpcSlave;
use std::marker::PhantomData;

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
