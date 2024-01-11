#![cfg(free_unix)]
use crate::{primitive::{bitmap::Bitmap, Message}, runtime::window_context::{PhysicalWindowContext, OutputReceiver}};
use std::sync::{mpsc::Sender, Arc};
use log::error;
use tipc::{ipc_master::IpcMaster, parking_lot::RwLock};
use tlib::{winit::{window::WindowId, event_loop::EventLoop}, typedef::WinitWindow};

pub(crate) struct X11Window<T: 'static + Copy + Send + Sync, M: 'static + Copy + Send + Sync> {
    window_id: WindowId,
    winit_window: Option<WinitWindow>,

    bitmap: Arc<RwLock<Bitmap>>,

    pub master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
    pub context: PhysicalWindowContext,
    pub user_ipc_event_sender: Option<Sender<Vec<T>>>,
}

impl<T: 'static + Copy + Send + Sync, M: 'static + Copy + Send + Sync> X11Window<T, M> {
    #[inline]
    pub fn new(
        window_id: WindowId,
        winit_window: WinitWindow,
        bitmap: Arc<RwLock<Bitmap>>,
        master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
        context: PhysicalWindowContext,
        user_ipc_event_sender: Option<Sender<Vec<T>>>,
    ) -> Self {
        Self {
            window_id,
            winit_window: Some(winit_window),
            bitmap,
            master,
            context,
            user_ipc_event_sender,
        }
    }

    #[inline]
    pub fn window_id(&self) -> WindowId {
        self.window_id
    }

    #[inline]
    pub fn take_event_loop(&mut self) -> EventLoop<Message> {
        match self.context.0 {
            OutputReceiver::EventLoop(ref mut event_loop) => {
                event_loop.take().expect("event_loop is None.")
            }
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn send_input(&self, msg: Message) {
        self.input_sender().send(msg).unwrap_or_else(|_| {
            error!("Error sending Message: The UI thread may have been closed.");
        });
    }

    #[inline]
    pub fn input_sender(&self) -> &Sender<Message> {
        &self.context.1 .0
    }

    #[inline]
    pub fn winit_window(&self) -> &WinitWindow {
        self.winit_window.as_ref().unwrap()
    }

    #[inline]
    pub fn take_winit_window(&mut self) -> Option<WinitWindow> {
        self.winit_window.take()
    }

    /// Request to redraw the window.
    pub fn request_redraw(&self) {}

    /// Redraw the window.
    pub fn redraw(&self) {}
}
