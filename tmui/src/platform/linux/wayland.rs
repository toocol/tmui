#![cfg(free_unix)]
use crate::platform::{logic_window::LogicWindow, physical_window::PhysicalWindow, PlatformContext, win_config::WindowConfig};
use std::sync::Arc;
use tipc::{ipc_master::IpcMaster, WithIpcMaster};

pub(crate) struct PlatformWayland<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send>
{
    /// Shared memory ipc
    master: Option<Arc<IpcMaster<T, M>>>,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformWayland<T, M> {
    #[inline]
    pub fn new() -> Self {
        Self {
            master: None,
        }
    }

    // Wrap trait `PlatfomContext` with [`Box`].
    #[inline]
    pub fn wrap(self) -> Box<dyn PlatformContext<T, M>> {
        Box::new(self)
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformContext<T, M>
    for PlatformWayland<T, M>
{
    fn initialize(&mut self) {
        todo!()
    }

    fn create_window(&mut self, win_config: WindowConfig) -> (LogicWindow<T, M>, PhysicalWindow<T, M>) {
        todo!()
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> WithIpcMaster<T, M>
    for PlatformWayland<T, M>
{
    fn proc_ipc_master(&mut self, master: tipc::ipc_master::IpcMaster<T, M>) {
        self.master = Some(Arc::new(master))
    }
}
