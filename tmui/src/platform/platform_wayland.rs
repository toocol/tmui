#![cfg(free_unix)]
use super::PlatformContext;
use crate::{
    primitive::{
        bitmap::Bitmap,
        shared_channel::{self, SharedChannel},
    },
    runtime::window_context::WindowContext,
};
use std::sync::{mpsc::Sender, Arc, RwLock};
use tipc::{ipc_master::IpcMaster, WithIpcMaster};

pub(crate) struct PlatformWayland<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send>
{
    title: String,
    width: u32,
    height: u32,

    bitmap: Option<Arc<RwLock<Bitmap>>>,

    /// Shared memory ipc
    master: Option<Arc<IpcMaster<T, M>>>,
    user_ipc_event_sender: Option<Sender<Vec<T>>>,
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
            user_ipc_event_sender: None,
        }
    }

    // Wrap trait `PlatfomContext` with [`Box`].
    #[inline]
    pub fn wrap(self) -> Box<dyn PlatformContext> {
        Box::new(self)
    }

    #[inline]
    pub fn shared_channel(&mut self) -> SharedChannel<T, M> {
        todo!()
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformContext
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

    fn region(&self) -> tlib::figure::Rect {
        todo!()
    }

    fn resize(&mut self, width: u32, height: u32) {
        todo!()
    }

    fn bitmap(&self) -> Arc<RwLock<Bitmap>> {
        todo!()
    }

    fn set_input_sender(&mut self, input_sender: Sender<super::Message>) {
        todo!()
    }

    fn input_sender(&self) -> &Sender<super::Message> {
        todo!()
    }

    fn create_window(&mut self) -> WindowContext {
        todo!()
    }

    fn platform_main(&mut self, window_context: WindowContext) {
        todo!()
    }

    fn request_redraw(&mut self, window: &tlib::winit::window::Window) {}

    fn redraw(&mut self) {
        todo!()
    }

    fn wait(&self) {
        todo!()
    }

    fn signal(&self) {
        todo!()
    }

    fn add_shared_region(&self, id: &'static str, rect: tlib::figure::Rect) {
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
