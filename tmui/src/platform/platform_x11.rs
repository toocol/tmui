#![cfg(free_unix)]
use std::sync::{mpsc::Sender, Arc};
use tipc::{ipc_master::IpcMaster, WithIpcMaster};
use crate::graphics::bitmap::Bitmap;
use super::{shared_channel::SharedChannel, PlatformContext};

pub(crate) struct PlatformX11<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    title: String,
    width: u32,
    height: u32,

    bitmap: Option<Bitmap>,

    // The memory area of pixels managed by `PlatformWin32`.
    _buffer: Option<Vec<u8>>,
    /// Shared memory ipc
    master: Option<Arc<IpcMaster<T, M>>>,
    user_ipc_event_sender: Option<Sender<Vec<T>>>,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformX11<T, M> {
    #[inline]
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            bitmap: None,
            _buffer: None,
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
    for PlatformX11<T, M>
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

    fn bitmap(&self) -> crate::graphics::bitmap::Bitmap {
        todo!()
    }

    fn set_input_sender(&mut self, input_sender: Sender<super::Message>) {
        todo!()
    }

    fn input_sender(&self) -> &Sender<super::Message> {
        todo!()
    }

    fn create_window(&mut self) -> super::window_context::WindowContext {
        todo!()
    }

    fn platform_main(&mut self, window_context: super::window_context::WindowContext) {
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
    for PlatformX11<T, M>
{
    fn proc_ipc_master(&mut self, master: tipc::ipc_master::IpcMaster<T, M>) {
        self.master = Some(Arc::new(master))
    }
}
