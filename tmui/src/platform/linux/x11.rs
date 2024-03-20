#![cfg(free_unix)]
use crate::{platform::{
    logic_window::LogicWindow, physical_window::PhysicalWindow,
    PlatformContext, PlatformType,
}, window::win_config::WindowConfig, primitive::Message, backend::BackendType};
use std::{sync::Arc, cell::Cell};
use tipc::{ipc_master::IpcMaster, WithIpcMaster};
use tlib::winit::event_loop::{EventLoopWindowTarget, EventLoopProxy};

pub(crate) struct PlatformX11<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    /// Shared memory ipc
    master: Option<Arc<IpcMaster<T, M>>>,

    main_win_create: Cell<bool>,
    platform_type: PlatformType,
    backend_type: BackendType,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformX11<T, M> {
    #[inline]
    pub fn new(platform_type: PlatformType, backend_type: BackendType) -> Self {
        Self {
            master: None,
            main_win_create: Cell::new(true),
            platform_type,
            backend_type,
        }
    }

    // Wrap trait `PlatfomContext` with [`Box`].
    #[inline]
    pub fn wrap(self) -> Box<dyn PlatformContext<T, M>> {
        Box::new(self)
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformContext<T, M>
    for PlatformX11<T, M>
{
    fn create_window(
        &self,
        win_config: WindowConfig,
        target: Option<&EventLoopWindowTarget<Message>>,
        proxy: Option<EventLoopProxy<Message>>,
    ) -> (LogicWindow<T, M>, PhysicalWindow<T, M>) {
        todo!()
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> WithIpcMaster<T, M>
    for PlatformX11<T, M>
{
    fn proc_ipc_master(&mut self, master: tipc::ipc_master::IpcMaster<T, M>) {
        self.master = Some(Arc::new(master))
    }
}
