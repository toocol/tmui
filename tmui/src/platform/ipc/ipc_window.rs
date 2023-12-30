use std::sync::{mpsc::Sender, Arc};

use tipc::{ipc_slave::IpcSlave, RwLock};

use crate::runtime::window_context::PhysicalWindowContext;

pub(crate) struct IpcWindow<T: 'static + Copy + Send + Sync, M: 'static + Copy + Sync + Send> {
    pub slave: Arc<RwLock<IpcSlave<T, M>>>,

    pub context: PhysicalWindowContext,

    pub user_ipc_event_sender: Sender<Vec<T>>,
}

impl<T: 'static + Copy + Send + Sync, M: 'static + Copy + Sync + Send> IpcWindow<T, M> {
    #[inline]
    pub fn new(
        slave: Arc<RwLock<IpcSlave<T, M>>>,
        context: PhysicalWindowContext,
        user_ipc_event_sender: Sender<Vec<T>>,
    ) -> Self {
        Self {
            slave,
            context,
            user_ipc_event_sender,
        }
    }
}
