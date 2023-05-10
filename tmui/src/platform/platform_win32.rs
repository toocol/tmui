#![cfg(target_os = "windows")]
use super::window_context::{OutputSender, WindowContext};
use super::Message;
use super::{window_process, PlatformContext};
use crate::{application::PLATFORM_CONTEXT, graphics::bitmap::Bitmap};
use std::{
    mem::size_of,
    os::raw::c_void,
    sync::{atomic::Ordering, mpsc::Sender},
};
use windows::Win32::{Foundation::*, Graphics::Gdi::*};
use winit::dpi::{PhysicalSize, Size};
use winit::event_loop::EventLoopBuilder;
use winit::platform::windows::WindowExtWindows;
use winit::window::WindowBuilder;

pub(crate) struct PlatformWin32 {
    title: String,
    width: u32,
    height: u32,

    front_bitmap: Bitmap,
    back_bitmap: Bitmap,
    // The memory area of pixels managed by `PlatformWin32`.
    _front_buffer: Vec<u8>,
    _back_buffer: Vec<u8>,

    /// The fileds associated with win32
    // _hins: HINSTANCE,
    hwnd: Option<HWND>,
    input_sender: Option<Sender<Message>>,
}

impl PlatformWin32 {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let mut front_buffer = vec![0u8; (width * height * 4) as usize];
        let front_bitmap = Bitmap::new(front_buffer.as_mut_ptr() as *mut c_void, width, height);

        let mut back_buffer = vec![0u8; (width * height * 4) as usize];
        let back_bitmap = Bitmap::new(back_buffer.as_mut_ptr() as *mut c_void, width, height);

        Self {
            title: title.to_string(),
            width,
            height,
            front_bitmap,
            back_bitmap,
            _front_buffer: front_buffer,
            _back_buffer: back_buffer,
            // _hins: hins,
            hwnd: None,
            input_sender: None,
        }
    }

    // Wrap trait `PlatfomContext` with [`Box`].
    pub fn wrap(self) -> Box<dyn PlatformContext> {
        Box::new(self)
    }
}

impl PlatformContext for PlatformWin32 {
    fn title(&self) -> &str {
        &self.title
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        todo!()
    }

    fn front_bitmap(&self) -> Bitmap {
        self.front_bitmap
    }

    fn back_bitmap(&self) -> Bitmap {
        self.back_bitmap
    }

    fn set_input_sender(&mut self, input_sender: Sender<super::Message>) {
        self.input_sender = Some(input_sender)
    }

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

    fn platform_main(&self, window_context: WindowContext) {
        unsafe {
            let platform = PLATFORM_CONTEXT
                .load(Ordering::SeqCst)
                .as_ref()
                .expect("`PLATFORM_WIN32` is None.");

            if let WindowContext::Default(window, event_loop, _) = window_context {
                window_process::WindowProcess::new().event_handle(
                    platform.as_ref(),
                    window,
                    event_loop,
                )
            } else {
                panic!("Invalid window context.")
            }
        }
    }

    fn redraw(&self) {
        unsafe {
            let hwnd = self.hwnd.unwrap();

            InvalidateRect(hwnd, None, true);
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);
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
                Some(self.front_bitmap().get_pixels().as_ptr() as *const c_void),
                &bmi,
                DIB_RGB_COLORS,
                SRCCOPY,
            );

            EndPaint(self.hwnd, &ps);
        }
    }
}
