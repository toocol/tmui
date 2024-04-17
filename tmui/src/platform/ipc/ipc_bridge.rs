use std::sync::Arc;
use tipc::{
    ipc_master::IpcMaster,
    ipc_slave::IpcSlave,
    mem::mem_rw_lock::{MemRwLock, MemRwLockGuard},
    IpcNode, parking_lot::RwLock, raw_sync::Timeout, 
};
use tlib::{figure::Rect, global::SemanticExt};

pub(crate) trait IpcBridge {
    fn region(&self) -> Rect;

    fn wait(&self, timeout: Timeout);

    fn signal(&self);

    fn add_shared_region(&self, id: &'static str, rect: Rect);

    fn size(&self) -> (u32, u32);

    fn create_buffer(&self, width: u32, height: u32);

    fn buffer(&self) -> (&'static [u8], MemRwLockGuard);

    fn buffer_mut(&self) -> (&'static mut [u8], MemRwLockGuard);

    fn pretreat_resize(&self, width: i32, height: i32);

    fn wait_prepared(&self);

    fn is_invalidate(&self) -> bool;

    fn set_invalidate(&self, invalidate: bool);
}

pub struct IpcBridgeModel<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    shared_widget_id: Option<&'static str>,
    master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
    slave: Option<Arc<RwLock<IpcSlave<T, M>>>>,
    lock: Arc<MemRwLock>,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> IpcBridgeModel<T, M> {
    #[allow(clippy::new_ret_no_self)]
    #[inline]
    pub fn new(
        shared_widget_id: Option<&'static str>,
        master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
        slave: Option<Arc<RwLock<IpcSlave<T, M>>>>,
    ) -> Box<dyn IpcBridge> {
        let lock = if let Some(ref master) = master {
            master.read().buffer_lock()
        } else if let Some(ref slave) = slave {
            slave.read().buffer_lock()
        } else {
            unreachable!()
        };

        Self {
            shared_widget_id,
            master,
            slave,
            lock,
        }
        .boxed()
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> IpcBridge
    for IpcBridgeModel<T, M>
{
    fn region(&self) -> Rect {
        if self.master.is_some() {
            unreachable!()
        }

        if let Some(ref slave) = self.slave {
            return slave
                .read()
                .region(self.shared_widget_id.unwrap())
                .expect("The `SharedWidget` with id `{}` was not exist.");
        }

        unreachable!()
    }

    fn wait(&self, timeout: Timeout) {
        if let Some(ref master) = self.master {
            master.read().wait(timeout);
            return;
        }

        if let Some(ref slave) = self.slave {
            slave.read().wait(timeout);
        }
    }

    fn signal(&self) {
        if let Some(ref master) = self.master {
            master.read().signal();
            return;
        }

        if let Some(ref slave) = self.slave {
            slave.read().signal();
        }
    }

    fn add_shared_region(&self, id: &'static str, rect: Rect) {
        if let Some(ref master) = self.master {
            master.read().add_rect(id, rect)
        }
        if self.slave.is_some() {
            unreachable!()
        }
    }

    fn size(&self) -> (u32, u32) {
        if let Some(ref master) = self.master {
            let guard = master.read();
            return (guard.width(), guard.height());
        }

        if let Some(ref slave) = self.slave {
            let guard = slave.read();
            return (guard.width(), guard.height());
        }

        unreachable!()
    }

    #[inline]
    fn create_buffer(&self, width: u32, height: u32) {
        if let Some(ref master) = self.master {
            master.write().create_buffer(width, height)
        }
    }

    #[inline]
    fn buffer(&self) -> (&'static [u8], MemRwLockGuard) {
        if let Some(ref master) = self.master {
            let guard = master.read();
            return (guard.buffer(), self.lock.read());
        }

        unreachable!()
    }

    #[inline]
    fn buffer_mut(&self) -> (&'static mut [u8], MemRwLockGuard) {
        if let Some(ref master) = self.master {
            let guard = master.read();
            return (guard.buffer(), self.lock.write());
        }

        unreachable!()
    }

    #[inline]
    fn pretreat_resize(&self, width: i32, height: i32) {
        if let Some(ref master) = self.master {
            master.write().pretreat_resize(width as u32, height as u32);
            return;
        }

        unreachable!()
    }

    #[inline]
    fn wait_prepared(&self) {
        if let Some(ref master) = self.master {
            master.read().wait_prepared();
            return;
        }

        unreachable!()
    }

    #[inline]
    fn is_invalidate(&self) -> bool {
        if let Some(ref master) = self.master {
            return master.read().is_invalidate();
        }
        if let Some(ref slave) = self.slave {
            return slave.read().is_invalidate();
        }

        unreachable!()
    }

    #[inline]
    fn set_invalidate(&self, invalidate: bool) {
        if let Some(ref master) = self.master {
            master.write().set_invalidate(invalidate);
            return;
        }
        if let Some(ref slave) = self.slave {
            slave.write().set_invalidate(invalidate);
            return;
        }

        unreachable!()
    }
}
