#![cfg(target_os = "macos")]
use super::{Message, PlatformContext};
use crate::{
    application::PLATFORM_CONTEXT,
    primitive::{
        bitmap::Bitmap,
        shared_channel::{self, SharedChannel},
    },
    runtime::{
        window_context::{OutputSender, WindowContext},
        window_process::WindowProcess,
    },
};
use cocoa::{
    appkit::{
        NSApp, NSApplication, NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
        NSImage, NSImageView, NSView, NSWindow,
    },
    base::{id, nil},
    foundation::{NSAutoreleasePool, NSSize},
};
use core_graphics::{
    base::{kCGImageAlphaLast, kCGRenderingIntentDefault},
    color_space::{kCGColorSpaceSRGB, CGColorSpace},
    data_provider::CGDataProvider,
    image::CGImage,
};
use objc::*;
use std::sync::{
    atomic::Ordering,
    mpsc::{channel, Sender},
    Arc
};
use tipc::{ipc_master::IpcMaster, IpcNode, WithIpcMaster, RwLock};
use tlib::winit::{
    dpi::{PhysicalSize, Size},
    event_loop::EventLoopBuilder,
    platform::macos::WindowExtMacOS,
    window::WindowBuilder,
};

pub(crate) struct PlatformMacos<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    title: String,
    width: u32,
    height: u32,
    resized: bool,

    bitmap: Option<Arc<RwLock<Bitmap>>>,
    input_sender: Option<Sender<Message>>,

    ns_window: Option<id>,
    ns_image_view: Option<id>,
    color_space: CGColorSpace,

    // Ipc shared memory context.
    master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
    user_ipc_event_sender: Option<Sender<Vec<T>>>,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformMacos<T, M> {
    #[inline]
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            resized: false,
            bitmap: None,
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
    pub fn shared_channel(&mut self) -> SharedChannel<T, M> {
        let (sender, receiver) = channel();
        self.user_ipc_event_sender = Some(sender);
        shared_channel::master_channel(self.master.as_ref().unwrap().clone(), receiver)
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformContext
    for PlatformMacos<T, M>
{
    fn initialize(&mut self) {
        match self.master {
            Some(ref master) => {
                let master = master.read();
                self.bitmap = Some(Arc::new(RwLock::new(Bitmap::from_raw_pointer(
                    master.buffer_raw_pointer(),
                    self.width,
                    self.height,
                    master.buffer_lock(),
                    master.name(),
                    master.ty(),
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

    fn region(&self) -> tlib::figure::Rect {
        unreachable!()
    }

    #[inline]
    fn resize(&mut self, width: u32, height: u32) {
        let mut bitmap_guard = self.bitmap.as_ref().unwrap().write().unwrap();
        self.width = width;
        self.height = height;
        self.resized = true;

        match self.master {
            Some(ref master) => {
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
    fn set_input_sender(&mut self, input_sender: std::sync::mpsc::Sender<super::Message>) {
        self.input_sender = Some(input_sender)
    }

    #[inline]
    fn input_sender(&self) -> &std::sync::mpsc::Sender<super::Message> {
        self.input_sender.as_ref().unwrap()
    }

    fn create_window(&mut self) -> WindowContext {
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

    fn platform_main(&mut self, window_context: WindowContext) {
        unsafe {
            let platform = PLATFORM_CONTEXT
                .load(Ordering::SeqCst)
                .as_mut()
                .expect("`PLATFORM_WIN32` is None.");

            if let WindowContext::Default(window, event_loop, _) = window_context {
                WindowProcess::new().event_handle(
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
    fn request_redraw(&mut self, window: &tlib::winit::window::Window) {
        window.request_redraw();
    }

    fn redraw(&mut self) {
        if self.width == 0 || self.height == 0 {
            return;
        }

        let bitmap_guard = self.bitmap.as_ref().unwrap().read().unwrap();
        if !bitmap_guard.is_prepared() {
            return;
        }

        unsafe {
            // Create NSImage by CGImage
            let ns_window = self.ns_window.unwrap();

            let content_view = ns_window.contentView();
            let rect = content_view.bounds();

            let (pixels, _guard) = bitmap_guard.get_pixels();

            // Create the CGImage from memory pixels buffer.
            let data_provider = CGDataProvider::from_slice(pixels);
            let cg_image = CGImage::new(
                self.width as usize,
                self.height as usize,
                8,
                32,
                self.width as usize * 4,
                &self.color_space,
                kCGImageAlphaLast,
                &data_provider,
                false,
                kCGRenderingIntentDefault,
            );
            let cg_img_ref = cg_image.as_ref();

            let image_size = NSSize::new(rect.size.width, rect.size.height);
            let ns_image = NSImage::alloc(nil);
            let ns_image: id = msg_send![ns_image, initWithCGImage:cg_img_ref size:image_size];

            // Set NSImage to NSImageView
            if self.ns_image_view.is_none() || self.resized {
                let ns_image_view =
                    NSImageView::initWithFrame_(NSImageView::alloc(nil), rect).autorelease();

                let old_ns_img = self.ns_image_view.replace(ns_image_view);
                if let Some(old_ns_img) = old_ns_img {
                    old_ns_img.removeFromSuperview()
                }

                content_view.addSubview_(ns_image_view);

                self.resized = false;
            }
            self.ns_image_view.as_mut().unwrap().setImage_(ns_image);

            let _: id = msg_send![ns_image, release];
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
    fn add_shared_region(&self, id: &'static str, rect: tlib::figure::Rect) {
        if let Some(ref master) = self.master {
            master.add_rect(id, rect)
        }
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> WithIpcMaster<T, M>
    for PlatformMacos<T, M>
{
    fn proc_ipc_master(&mut self, master: tipc::ipc_master::IpcMaster<T, M>) {
        self.master = Some(Arc::new(master))
    }
}
