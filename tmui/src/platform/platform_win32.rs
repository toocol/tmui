#![cfg(target_os = "windows")]
use super::{
    shared_channel::{self, SharedChannel},
    PlatformContext,
};
use crate::{winit::{
    dpi::{PhysicalSize, Size},
    event_loop::EventLoopBuilder,
    platform::windows::WindowExtWindows,
    window::WindowBuilder,
}, primitive::Message};
use crate::{application::PLATFORM_CONTEXT, primitive::bitmap::Bitmap, runtime::{window_process, window_context::{OutputSender, WindowContext}}};
use std::{
    mem::size_of,
    os::raw::c_void,
    sync::{
        atomic::Ordering,
        mpsc::{channel, Sender},
        Arc,
    },
};
use tipc::{ipc_master::IpcMaster, IpcNode, WithIpcMaster};
use tlib::figure::Rect;
use windows::Win32::{Foundation::*, Graphics::Gdi::*};

pub(crate) struct PlatformWin32<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    title: String,
    width: u32,
    height: u32,

    bitmap: Option<Bitmap>,
    // The memory area of pixels managed by `PlatformWin32`.
    _buffer: Option<Vec<u8>>,

    /// The fileds associated with win32
    // _hins: HINSTANCE,
    hwnd: Option<HWND>,
    input_sender: Option<Sender<Message>>,

    /// Shared memory ipc
    master: Option<Arc<IpcMaster<T, M>>>,
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
            _buffer: None,
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
                let front_bitmap =
                    Bitmap::new(master.buffer_raw_pointer(), self.width, self.height);

                self.bitmap = Some(front_bitmap);
            }
            None => {
                let mut buffer = vec![0u8; (self.width * self.height * 4) as usize];
                let bitmap =
                    Bitmap::new(buffer.as_mut_ptr() as *mut c_void, self.width, self.height);

                self._buffer = Some(buffer);
                self.bitmap = Some(bitmap);
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
        self.width = width;
        self.height = height;

        // Recreate the bitmap
        match self.master {
            Some(ref _master) => {}
            None => {
                let mut buffer = vec![0u8; (self.width * self.height * 4) as usize];
                let bitmap =
                    Bitmap::new(buffer.as_mut_ptr() as *mut c_void, self.width, self.height);

                self._buffer = Some(buffer);
                self.bitmap = Some(bitmap);
            }
        }
    }

    #[inline]
    fn bitmap(&self) -> Bitmap {
        self.bitmap.unwrap()
    }

    #[inline]
    fn set_input_sender(&mut self, input_sender: Sender<Message>) {
        self.input_sender = Some(input_sender)
    }

    #[inline]
    fn input_sender(&self) -> &Sender<Message> {
        self.input_sender.as_ref().unwrap()
    }

    fn create_window(&mut self) -> WindowContext {
        let event_loop = EventLoopBuilder::<Message>::with_user_event().build();

        let window = WindowBuilder::new()
            .with_title(&self.title)
            .with_inner_size(Size::Physical(PhysicalSize::new(self.width, self.height)))
            .build(&event_loop)
            .unwrap();

        self.hwnd = Some(HWND(window.hwnd()));
        let event_loop_proxy = event_loop.create_proxy();

        WindowContext::Default(
            window,
            event_loop,
            Some(OutputSender::EventLoopProxy(event_loop_proxy)),
        )
    }

    fn platform_main(&mut self, window_context: WindowContext) {
        unsafe {
            let platform = PLATFORM_CONTEXT
                .load(Ordering::SeqCst)
                .as_mut()
                .expect("`PLATFORM_WIN32` is None.");

            if let WindowContext::Default(window, event_loop, _) = window_context {
                window_process::WindowProcess::new().event_handle::<T, M>(
                    platform.as_mut(),
                    window,
                    event_loop,
                    self.master.clone(),
                    self.user_ipc_event_sender.take(),
                )
            } else {
                panic!("Invalid window context.")
            }
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
                Some(self.bitmap().get_pixels().as_ptr() as *const c_void),
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
            master.wait()
        }
    }

    #[inline]
    fn signal(&self) {
        if let Some(ref master) = self.master {
            master.signal()
        }
    }

    #[inline]
    fn add_shared_region(&self, id: &'static str, rect: Rect) {
        if let Some(ref master) = self.master {
            master.add_rect(id, rect)
        }
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> WithIpcMaster<T, M>
    for PlatformWin32<T, M>
{
    fn proc_ipc_master(&mut self, master: tipc::ipc_master::IpcMaster<T, M>) {
        self.master = Some(Arc::new(master))
    }
}
