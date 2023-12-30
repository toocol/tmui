#![cfg(free_unix)]
use crate::{primitive::bitmap::Bitmap, runtime::window_context::PhysicalWindowContext};
use std::sync::{mpsc::Sender, Arc};
use tipc::{ipc_master::IpcMaster, RwLock};
use tlib::winit::window::Window;

pub(crate) struct X11Window<T: 'static + Copy + Send + Sync, M: 'static + Copy + Send + Sync> {
    bitmap: Arc<RwLock<Bitmap>>,

    pub master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
    pub context: Option<PhysicalWindowContext>,
    pub user_ipc_event_sender: Option<Sender<Vec<T>>>,
}

impl<T: 'static + Copy + Send + Sync, M: 'static + Copy + Send + Sync> X11Window<T, M> {
    #[inline]
    pub fn new(
        bitmap: Arc<RwLock<Bitmap>>,
        master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
        context: PhysicalWindowContext,
        user_ipc_event_sender: Option<Sender<Vec<T>>>,
    ) -> Self {
        Self {
            bitmap,
            master,
            context: Some(context),
            user_ipc_event_sender,
        }
    }

    /// Request to redraw the window.
    pub fn request_redraw(&self, _window: &Window) {}

    /// Redraw the window.
    pub fn redraw(&self) {}
}
