#![cfg(target_os = "macos")]
use super::{
    window_context::{OutputSender, WindowContext},
    window_process, Message, PlatformContext,
};
use crate::{application::PLATFORM_CONTEXT, graphics::bitmap::Bitmap};
use cocoa::{
    appkit::{
        NSApp, NSApplication, NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
        NSImage, NSImageView, NSView, NSWindow,
    },
    base::{id, nil, NO},
    foundation::{NSAutoreleasePool, NSSize},
};
use core_graphics::{
    base::{kCGImageAlphaLast, kCGRenderingIntentDefault},
    color_space::{kCGColorSpaceSRGB, CGColorSpace},
    data_provider::CGDataProvider,
    image::CGImage,
};
use objc::*;
use std::{
    ffi::c_void,
    sync::{atomic::Ordering, mpsc::Sender},
};
use tipc::{ipc_master::IpcMaster, WithIpcMaster};
use winit::{
    dpi::{PhysicalSize, Size},
    event_loop::EventLoopBuilder,
    platform::macos::WindowExtMacOS,
    window::WindowBuilder,
};

pub(crate) struct PlatformMacos<T: 'static + Copy, M: 'static + Copy> {
    title: String,
    width: u32,
    height: u32,

    front_bitmap: Bitmap,
    back_bitmap: Bitmap,
    // The memory area of pixels managed by `PlatformMacos`.
    _front_buffer: Vec<u8>,
    _back_buffer: Vec<u8>,
    input_sender: Option<Sender<Message>>,

    ns_window: Option<id>,
    ns_image_view: Option<id>,
    color_space: CGColorSpace,

    // Ipc shared memory context.
    master: Option<IpcMaster<T, M>>,
}

impl<T: 'static + Copy, M: 'static + Copy> PlatformMacos<T, M> {
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
            ns_window: None,
            ns_image_view: None,
            color_space: unsafe { CGColorSpace::create_with_name(kCGColorSpaceSRGB).unwrap() },
            input_sender: None,
            master: None,
        }
    }

    // Wrap trait `PlatfomContext` with [`Box`].
    pub fn wrap(self) -> Box<dyn PlatformContext> {
        Box::new(self)
    }
}

impl<T: 'static + Copy, M: 'static + Copy> PlatformContext for PlatformMacos<T, M> {
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

        unsafe {
            let ns_app = NSApp();
            ns_app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
        }

        let window = WindowBuilder::new()
            .with_title(&self.title)
            .with_inner_size(Size::Physical(PhysicalSize::new(self.width, self.height)))
            .build(&event_loop)
            .unwrap();

        self.ns_window = Some(window.ns_window() as id);

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
                .as_mut()
                .expect("`PLATFORM_WIN32` is None.");

            if let WindowContext::Default(window, event_loop, _) = window_context {
                window_process::WindowProcess::new().event_handle(
                    platform.as_mut(),
                    window,
                    event_loop,
                )
            } else {
                panic!("Invalid window context.")
            }
        }
    }

    fn redraw(&mut self) {
        unsafe {
            // Create NSImage by CGImage
            let ns_window = self.ns_window.unwrap();

            let content_view = ns_window.contentView();
            let rect = content_view.bounds();

            // Create the CGImage from memory pixels buffer.
            let data_provider = CGDataProvider::from_slice(&self._front_buffer);
            let cg_image = CGImage::new(
                self.width as usize,
                self.height as usize,
                8,
                32,
                self.width as usize * 4,
                &self.color_space,
                kCGImageAlphaLast,
                &data_provider,
                NO,
                kCGRenderingIntentDefault,
            );
            let cg_img_ref = cg_image.as_ref();

            let image_size = NSSize::new(rect.size.width, rect.size.height);
            let ns_image = NSImage::alloc(nil);
            let ns_image: id = msg_send![ns_image, initWithCGImage:cg_img_ref size:image_size];

            // Set NSImage to NSImageView
            if self.ns_image_view.is_none() {
                let ns_image_view =
                    NSImageView::initWithFrame_(NSImageView::alloc(nil), rect).autorelease();
                content_view.addSubview_(ns_image_view);
                self.ns_image_view = Some(ns_image_view);
            }
            self.ns_image_view.as_mut().unwrap().setImage_(ns_image);

            let _: id = msg_send![ns_image, release];
        }
    }
}

impl<T: 'static + Copy, M: 'static + Copy> WithIpcMaster<T, M> for PlatformMacos<T, M> {
    fn proc_ipc_master(&mut self, master: tipc::ipc_master::IpcMaster<T, M>) {
        self.master = Some(master)
    }
}
