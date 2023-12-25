use std::sync::Arc;

use tipc::{ipc_master::IpcMaster, ipc_slave::IpcSlave, IpcNode, RwLock};
use tlib::{figure::Rect, global::SemanticExt};

pub(crate) trait IpcBridge {
    fn region(&self) -> Rect;

    fn wait(&self);

    fn signal(&self);

    fn add_shared_region(&self, id: &'static str, rect: Rect);

    fn size(&self) -> (u32, u32);
}

pub struct IpcBridgeModel<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    shared_widget_id: Option<&'static str>,
    master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
    slave: Option<Arc<RwLock<IpcSlave<T, M>>>>,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> IpcBridgeModel<T, M> {
    #[inline]
    pub fn new(
        shared_widget_id: Option<&'static str>,
        master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
        slave: Option<Arc<RwLock<IpcSlave<T, M>>>>,
    ) -> Box<dyn IpcBridge> {
        Self {
            shared_widget_id,
            master,
            slave,
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

    fn wait(&self) {
        if let Some(ref master) = self.master {
            master.read().wait();
            return;
        }

        if let Some(ref slave) = self.slave {
            slave.read().wait();
            return;
        }
    }

    fn signal(&self) {
        if let Some(ref master) = self.master {
            master.read().signal();
            return;
        }

        if let Some(ref slave) = self.slave {
            slave.read().signal();
            return;
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
}
