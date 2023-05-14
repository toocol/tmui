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
    sync::{atomic::Ordering, mpsc::{Sender, Receiver, channel}, Arc},
};
use tipc::{ipc_master::IpcMaster, WithIpcMaster, IpcNode};
use winit::{
    dpi::{PhysicalSize, Size},
    event_loop::EventLoopBuilder,
    platform::macos::WindowExtMacOS,
    window::WindowBuilder,
};

pub(crate) struct PlatformMacos<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    title: String,
    width: u32,
    height: u32,

    front_bitmap: Option<Bitmap>,
    back_bitmap: Option<Bitmap>,
    // The memory area of pixels managed by `PlatformMacos`.
    _front_buffer: Option<Vec<u8>>,
    _back_buffer: Option<Vec<u8>>,
    input_sender: Option<Sender<Message>>,

    ns_window: Option<id>,
    ns_image_view: Option<id>,
    color_space: CGColorSpace,

    // Ipc shared memory context.
    master: Option<Arc<IpcMaster<T, M>>>,
    user_ipc_event_sender: Option<Sender<Vec<T>>>,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformMacos<T, M> {
    #[inline]
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            front_bitmap: None,
            back_bitmap: None,
            _front_buffer: None,
            _back_buffer: None,
            ns_window: None,
            ns_image_view: None,
            color_space: unsafe { CGColorSpace::create_with_name(kCGColorSpaceSRGB).unwrap() },
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
    pub fn gen_user_ipc_event_channel(&mut self) -> Receiver<Vec<T>> {
        let (sender, receiver) = channel();
        self.user_ipc_event_sender = Some(sender);
        receiver
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformContext for PlatformMacos<T, M> {
    fn initialize(&mut self) {
        match self.master {
            Some(ref master) => {
                let front_bitmap = Bitmap::new(
                    master.primary_buffer_raw_pointer(),
                    self.width,
                    self.height,
                );

                let back_bitmap = Bitmap::new(
                    master.secondary_buffer_raw_pointer(),
                    self.width,
                    self.height,
                );

                self.front_bitmap = Some(front_bitmap);
                self.back_bitmap = Some(back_bitmap);
            }
            None => {
                let mut front_buffer = vec![0u8; (self.width * self.height * 4) as usize];
                let front_bitmap = Bitmap::new(
                    front_buffer.as_mut_ptr() as *mut c_void,
                    self.width,
                    self.height,
                );

                let mut back_buffer = vec![0u8; (self.width * self.height * 4) as usize];
                let back_bitmap = Bitmap::new(
                    back_buffer.as_mut_ptr() as *mut c_void,
                    self.width,
                    self.height,
                );

                self._front_buffer = Some(front_buffer);
                self._back_buffer = Some(back_buffer);
                self.front_bitmap = Some(front_bitmap);
                self.back_bitmap = Some(back_bitmap);
            }
        }
    }

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
        self.front_bitmap.unwrap()
    }

    fn back_bitmap(&self) -> Bitmap {
        self.back_bitmap.unwrap()
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
                    self.master.clone()
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
            let data_provider = CGDataProvider::from_slice(self.front_bitmap().get_pixels());
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

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> WithIpcMaster<T, M> for PlatformMacos<T, M> {
    fn proc_ipc_master(&mut self, master: tipc::ipc_master::IpcMaster<T, M>) {
        self.master = Some(Arc::new(master))
    }
}
