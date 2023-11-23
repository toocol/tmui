use ipc_event::IpcEvent;
use ipc_master::IpcMaster;
use ipc_slave::IpcSlave;
use lazy_static::lazy_static;
use mem::{mem_queue::MemQueueError, mem_rw_lock::MemRwLock};
use std::{collections::HashMap, error::Error, ffi::c_void, marker::PhantomData, sync::Arc};
use tlib::figure::Rect;

pub mod ipc_event;
pub mod ipc_master;
pub mod ipc_slave;
pub mod mem;

pub use parking_lot::*;
pub use shared_memory::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcType {
    Master,
    Slave,
}

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
    fn name(&self) -> &str;

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

    fn region(&self, id: &'static str) -> Option<Rect>;

    fn width(&self) -> u32;

    fn height(&self) -> u32;

    fn buffer_lock(&self) -> Arc<MemRwLock>;

    fn ty(&self) -> IpcType;

    fn resize(&mut self, width: u32, height: u32) -> Shmem;
}

lazy_static! {
    static ref CHARCTER_MAP: HashMap<u8, u8> = {
        let mut mapping = HashMap::new();
        let mut cur = 0;

        for c in b'0'..b'9' {
            mapping.insert(c, cur);
            cur += 1;
        }

        for c in b'a'..b'z' {
            mapping.insert(c, cur);
            cur += 1;
        }

        for c in b'A'..b'z' {
            mapping.insert(c, cur);
            cur += 1;
        }

        for c in b"!@#$%^&*()-_+=/\\" {
            mapping.insert(*c, cur);
            cur += 1;
        }

        mapping
    };
}

#[inline]
pub fn generate_u128(input: &str) -> Option<u128> {
    let bytes = input.as_bytes();
    let mut result = Vec::with_capacity(bytes.len() * 2); // Assuming max 3 digits per byte

    for b in bytes {
        result.extend_from_slice(CHARCTER_MAP.get(b)?.to_string().as_bytes());
    }

    let result_str = String::from_utf8(result).ok()?;
    result_str.parse::<u128>().ok()
}

#[cfg(test)]
mod tests {
    use crate::generate_u128;

    #[test]
    fn test_generate_u128() {
        let res1 = generate_u128(&"shmem_widgetdddddd".to_uppercase());
        assert!(res1.is_some());

        let res2 = generate_u128("shmem_widggt");
        assert!(res2.is_some());

        assert!(res1.unwrap() != res2.unwrap());
    }
}
