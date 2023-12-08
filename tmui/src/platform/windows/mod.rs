#![cfg(windows_platform)]
pub(crate) mod physical_window;

use self::physical_window::PhysicalWindow;

use super::PlatformContext;
use crate::{
    primitive::Message,
    winit::{
        dpi::{PhysicalSize, Size},
        event_loop::EventLoopBuilder,
        window::WindowBuilder,
    },
};
use crate::{
    primitive::{
        bitmap::Bitmap,
        shared_channel::{self, SharedChannel},
    },
    runtime::{
        window_context::{OutputSender, WindowContext},
        window_process,
    },
};
use std::{
    mem::size_of,
    os::raw::c_void,
    sync::{
        mpsc::{channel, Sender},
        Arc,
    },
};
use tipc::{
    ipc_master::IpcMaster, lock_api::RwLockWriteGuard, IpcNode, RawRwLock, RwLock, WithIpcMaster,
};
use tlib::{
    figure::Rect,
    ptr_ref,
    winit::raw_window_handle::{HasWindowHandle, RawWindowHandle},
};
use windows::Win32::{Foundation::*, Graphics::Gdi::*};

pub(crate) struct PlatformWin32<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    title: String,
    width: u32,
    height: u32,

    bitmap: Option<Arc<RwLock<Bitmap>>>,

    /// The fileds associated with win32
    hwnd: Option<HWND>,
    input_sender: Option<Sender<Message>>,

    /// Shared memory ipc
    master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
    user_ipc_event_sender: Option<Sender<Vec<T>>>,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformWin32<T, M> {
    #[inline]
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            bitmap: None,
            hwnd: None,
            input_sender: None,
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
        let (sender, receiver) = channel();
        self.user_ipc_event_sender = Some(sender);
        shared_channel::master_channel(self.master.as_ref().unwrap().clone(), receiver)
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformContext
    for PlatformWin32<T, M>
{
    fn initialize(&mut self) {
        match self.master {
            Some(ref master) => {
                let guard = master.read();
                self.bitmap = Some(Arc::new(RwLock::new(Bitmap::from_raw_pointer(
                    guard.buffer_raw_pointer(),
                    self.width,
                    self.height,
                    guard.buffer_lock(),
                    guard.name(),
                    guard.ty(),
                ))));
            }
            None => {
                self.bitmap = Some(Arc::new(RwLock::new(Bitmap::new(self.width, self.height))));
            }
        }
    }

    #[inline]
    fn title(&self) -> &str {
        &self.title
    }

    #[inline]
    fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    fn region(&self) -> Rect {
        unreachable!()
    }

    #[inline]
    fn resize(&mut self, width: u32, height: u32) {
        let mut bitmap_guard = self.bitmap.as_ref().unwrap().write();
        self.width = width;
        self.height = height;

        match self.master {
            Some(ref master) => {
                let _guard =
                    ptr_ref!(&bitmap_guard as *const RwLockWriteGuard<'_, RawRwLock, Bitmap>)
                        .ipc_write();

                let mut master = master.write();
                let old_shmem = master.resize(width, height);

                bitmap_guard.update_raw_pointer(
                    master.buffer_raw_pointer(),
                    old_shmem,
                    width,
                    height,
                );
            }
            None => bitmap_guard.resize(width, height),
        }
    }

    #[inline]
    fn bitmap(&self) -> Arc<RwLock<Bitmap>> {
        self.bitmap.as_ref().unwrap().clone()
    }

    #[inline]
    fn set_input_sender(&mut self, input_sender: Sender<Message>) {
        self.input_sender = Some(input_sender)
    }

    // #[inline]
    // fn input_sender(&self) -> &Sender<Message> {
    //     self.input_sender.as_ref().unwrap()
    // }

    fn create_window(&mut self) -> WindowContext {
        let event_loop = EventLoopBuilder::<Message>::with_user_event()
            .build()
            .unwrap();

        let window = WindowBuilder::new()
            .with_title(&self.title)
            .with_inner_size(Size::Physical(PhysicalSize::new(self.width, self.height)))
            .build(&event_loop)
            .unwrap();

        let window_handle = window.window_handle().unwrap().as_raw();
        match window_handle {
            RawWindowHandle::Win32(hwnd) => self.hwnd = Some(HWND(hwnd.hwnd.into())),
            _ => {}
        };
        let event_loop_proxy = event_loop.create_proxy();

        WindowContext::Default(
            window,
            event_loop,
            Some(OutputSender::EventLoopProxy(event_loop_proxy)),
        )
    }

    fn platform_main(&mut self, window_context: WindowContext) {
        if let WindowContext::Default(window, event_loop, _) = window_context {
            window_process::WindowProcess::new().event_handle::<T, M>(
                PhysicalWindow::new(self.hwnd.unwrap(), self.bitmap()),
                window,
                event_loop,
                self.master.clone(),
                self.user_ipc_event_sender.take(),
                self.input_sender.take().unwrap(),
            )
        } else {
            panic!("Invalid window context.")
        }
    }

    #[inline]
    fn request_redraw(&mut self, _window: &tlib::winit::window::Window) {
        let hwnd = self.hwnd.unwrap();

        unsafe {
            InvalidateRect(hwnd, None, false);
        }
    }

    fn redraw(&mut self) {
        if self.width == 0 || self.height == 0 {
            return;
        }

        let bitmap_guard = self.bitmap.as_ref().unwrap().read();
        if !bitmap_guard.is_prepared() {
            return;
        }

        unsafe {
            let hwnd = self.hwnd.unwrap();

            let width = self.width();
            let height = self.height();

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

    #[inline]
    fn wait(&self) {
        if let Some(ref master) = self.master {
            master.read().wait()
        }
    }

    #[inline]
    fn signal(&self) {
        if let Some(ref master) = self.master {
            master.read().signal()
        }
    }

    #[inline]
    fn add_shared_region(&self, id: &'static str, rect: Rect) {
        if let Some(ref master) = self.master {
            master.read().add_rect(id, rect)
        }
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> WithIpcMaster<T, M>
    for PlatformWin32<T, M>
{
    fn proc_ipc_master(&mut self, master: tipc::ipc_master::IpcMaster<T, M>) {
        self.master = Some(Arc::new(RwLock::new(master)))
    }
}
