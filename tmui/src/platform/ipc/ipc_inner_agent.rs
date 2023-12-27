use std::sync::Arc;
use tipc::{ipc_master::IpcMaster, ipc_slave::IpcSlave, IpcNode, RwLock};
use tlib::global::SemanticExt;

pub(crate) trait IpcInnerAgent {
    fn release_retention(&self);

    fn prepared(&self);
}

pub(crate) struct InnerAgent<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
    slave: Option<Arc<RwLock<IpcSlave<T, M>>>>,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> InnerAgent<T, M> {
    #[inline]
    pub(crate) fn master(master: Arc<RwLock<IpcMaster<T, M>>>) -> Box<dyn IpcInnerAgent> {
        Self {
            master: Some(master),
            slave: None,
        }
        .boxed()
    }

    #[inline]
    pub(crate) fn slave(slave: Arc<RwLock<IpcSlave<T, M>>>) -> Box<dyn IpcInnerAgent> {
        Self {
            master: None,
            slave: Some(slave),
        }
        .boxed()
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> IpcInnerAgent
    for InnerAgent<T, M>
{
    fn release_retention(&self) {
        if let Some(ref master) = self.master {
            master.write().release_retention();
            return;
        }

        if let Some(ref slave) = self.slave {
            slave.write().release_retention();
            return;
        }

        unreachable!()
    }

    #[inline]
    fn prepared(&self) {
        if let Some(ref slave) = self.slave {
            slave.read().prepared();
            return;
        }

        unreachable!()
    }
}
