#![cfg(free_unix)]
use crate::{
    platform::{logic_window::LogicWindow, physical_window::PhysicalWindow, PlatformContext},
    primitive::bitmap::Bitmap,
};
use std::sync::{mpsc::Sender, Arc};
use tipc::{ipc_master::IpcMaster, RwLock, WithIpcMaster};

pub(crate) struct PlatformWayland<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send>
{
    title: String,
    width: u32,
    height: u32,

    bitmap: Option<Arc<RwLock<Bitmap>>>,

    /// Shared memory ipc
    master: Option<Arc<IpcMaster<T, M>>>,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformWayland<T, M> {
    #[inline]
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            bitmap: None,
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

    fn title(&self) -> &str {
        todo!()
    }

    fn width(&self) -> u32 {
        todo!()
    }

    fn height(&self) -> u32 {
        todo!()
    }

    fn bitmap(&self) -> Arc<RwLock<Bitmap>> {
        todo!()
    }

    fn create_window(&mut self) -> (LogicWindow<T, M>, PhysicalWindow<T, M>) {
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
