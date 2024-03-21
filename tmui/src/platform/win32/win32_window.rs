#![cfg(windows_platform)]
use crate::{
    platform::gl_bootstrap::GlEnv,
    primitive::{bitmap::Bitmap, Message},
    runtime::window_context::{OutputReceiver, PhysicalWindowContext},
};
use log::error;
use std::{
    ffi::c_void,
    mem::size_of,
    sync::{mpsc::Sender, Arc},
};
use tipc::{ipc_master::IpcMaster, parking_lot::RwLock};
use tlib::{
    typedef::WinitWindow,
    winit::{event_loop::EventLoop, window::WindowId},
};
use windows::Win32::{Foundation::*, Graphics::Gdi::*};

pub(crate) struct Win32Window<T: 'static + Copy + Send + Sync, M: 'static + Copy + Send + Sync> {
    window_id: WindowId,
    winit_window: Option<WinitWindow>,

    hwnd: HWND,
    bitmap: Arc<RwLock<Bitmap>>,

    gl_env: Option<Arc<GlEnv>>,

    pub master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
    pub context: PhysicalWindowContext,
    pub user_ipc_event_sender: Option<Sender<Vec<T>>>,
}

impl<T: 'static + Copy + Send + Sync, M: 'static + Copy + Send + Sync> Win32Window<T, M> {
    #[inline]
    pub fn new(
        window_id: WindowId,
        winit_window: WinitWindow,
        hwnd: HWND,
        bitmap: Arc<RwLock<Bitmap>>,
        gl_env: Option<Arc<GlEnv>>,
        master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
        context: PhysicalWindowContext,
        user_ipc_event_sender: Option<Sender<Vec<T>>>,
    ) -> Self {
        Self {
            window_id,
            winit_window: Some(winit_window),
            hwnd,
            bitmap,
            gl_env,
            master,
            context: context,
            user_ipc_event_sender,
        }
    }

    #[inline]
    pub fn is_gl_backend(&self) -> bool {
        self.gl_env.is_some()
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
    #[inline]
    pub fn request_redraw(&self) {
        unsafe {
            InvalidateRect(self.hwnd, None, false);
        }
    }

    /// Redraw the window.
    pub fn redraw(&self) {
        if self.is_gl_backend() {
            return;
        }

        let bitmap_guard = self.bitmap.read();
        if !bitmap_guard.is_prepared() {
            return;
        }

        unsafe {
            let hwnd = self.hwnd;

            let width = bitmap_guard.width();
            let height = bitmap_guard.height();

            let mut bmi = BITMAPINFO::default();
            bmi.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as u32;
            bmi.bmiHeader.biWidth = width as i32;
            // Drawing start at top-left.
            bmi.bmiHeader.biHeight = -(height as i32);
            bmi.bmiHeader.biPlanes = 1;
            bmi.bmiHeader.biBitCount = 32;
            bmi.bmiHeader.biCompression = BI_RGB;

            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);

            let _guard = bitmap_guard.ipc_read();
            let pixels = bitmap_guard.get_pixels();
            StretchDIBits(
                hdc,
                0,
                0,
                width as i32,
                height as i32,
                0,
                0,
                width as i32,
                height as i32,
                Some(pixels.as_ptr() as *const c_void),
                &bmi,
                DIB_RGB_COLORS,
                SRCCOPY,
            );

            EndPaint(self.hwnd, &ps);

            ReleaseDC(hwnd, hdc);
        }
    }
}
