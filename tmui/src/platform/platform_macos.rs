#![cfg(target_os = "macos")]
use std::{ffi::c_void, sync::{mpsc::Sender, atomic::Ordering}};
use winit::{
    dpi::{PhysicalSize, Size},
    event_loop::EventLoopBuilder,
    window::WindowBuilder,
};
use super::{
    window_context::{OutputSender, WindowContext},
    Message, PlatformContext, window_process,
};
use crate::{graphics::bitmap::Bitmap, application::PLATFORM_CONTEXT};

pub(crate) struct PlatformMacos {
    title: String,
    width: u32,
    height: u32,

    front_bitmap: Bitmap,
    back_bitmap: Bitmap,
    // The memory area of pixels managed by `PlatformWin32`.
    _front_buffer: Vec<u8>,
    _back_buffer: Vec<u8>,
    input_sender: Option<Sender<Message>>,
}

impl PlatformMacos {
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
            input_sender: None,
        }
    }

    // Wrap trait `PlatfomContext` with [`Box`].
    pub fn wrap(self) -> Box<dyn PlatformContext> {
        Box::new(self)
    }
}

impl PlatformContext for PlatformMacos {
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

    fn set_input_sender(&mut self, input_sender: std::sync::mpsc::Sender<super::Message>) {
        self.input_sender = Some(input_sender)
    }

    fn input_sender(&self) -> &std::sync::mpsc::Sender<super::Message> {
        self.input_sender.as_ref().unwrap()
    }

    fn create_window(&mut self) -> super::window_context::WindowContext {
        let event_loop = EventLoopBuilder::<Message>::with_user_event().build();

        let window = WindowBuilder::new()
            .with_title(&self.title)
            .with_inner_size(Size::Physical(PhysicalSize::new(self.width, self.height)))
            .build(&event_loop)
            .unwrap();

        let event_loop_proxy = event_loop.create_proxy();

        WindowContext::Default(
            window,
            event_loop,
            Some(OutputSender::EventLoopProxy(event_loop_proxy)),
        )
    }

    fn platform_main(&self, window_context: super::window_context::WindowContext) {
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

    fn redraw(&self) {}
}
