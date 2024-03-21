#![cfg(target_os = "macos")]
use crate::{
    platform::gl_bootstrap::GlEnv, primitive::{bitmap::Bitmap, Message}, runtime::window_context::{OutputReceiver, PhysicalWindowContext}
};
use cocoa::{
    appkit::{NSEvent, NSImage, NSImageView, NSView, NSWindow},
    base::{id, nil},
    foundation::{NSAutoreleasePool, NSSize},
};
use core_graphics::{
    base::{kCGImageAlphaLast, kCGRenderingIntentDefault},
    color_space::{kCGColorSpaceSRGB, CGColorSpace},
    data_provider::CGDataProvider,
    image::CGImage,
};
use log::error;
use objc::*;
use std::sync::{mpsc::Sender, Arc};
use tipc::{ipc_master::IpcMaster, parking_lot::RwLock};
use tlib::{
    typedef::WinitWindow,
    winit::{event_loop::EventLoop, window::WindowId},
};

pub(crate) struct MacosWindow<T: 'static + Copy + Send + Sync, M: 'static + Copy + Send + Sync> {
    window_id: WindowId,
    winit_window: Option<WinitWindow>,

    ns_view: id,
    ns_image_view: Option<id>,
    color_space: CGColorSpace,

    bitmap: Arc<RwLock<Bitmap>>,

    gl_env: Option<Arc<GlEnv>>,

    pub master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
    pub context: PhysicalWindowContext,
    pub user_ipc_event_sender: Option<Sender<Vec<T>>>,
}

impl<T: 'static + Copy + Send + Sync, M: 'static + Copy + Send + Sync> MacosWindow<T, M> {
    #[inline]
    pub fn new(
        window_id: WindowId,
        winit_window: WinitWindow,
        ns_view: id,
        bitmap: Arc<RwLock<Bitmap>>,
        gl_env: Option<Arc<GlEnv>>,
        master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
        context: PhysicalWindowContext,
        user_ipc_event_sender: Option<Sender<Vec<T>>>,
    ) -> Self {
        Self {
            window_id,
            winit_window: Some(winit_window),
            ns_view,
            ns_image_view: None,
            color_space: unsafe { CGColorSpace::create_with_name(kCGColorSpaceSRGB).unwrap() },
            bitmap,
            gl_env,
            master,
            context,
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

    #[inline]
    pub fn request_redraw(&self) {
        if let Some(ref window) = self.winit_window {
            window.request_redraw();
        }
    }

    pub fn redraw(&mut self) {
        if self.is_gl_backend() {
            return;
        }

        let bitmap_guard = self.bitmap.read();
        if !bitmap_guard.is_prepared() {
            return;
        }
        let (width, height) = (bitmap_guard.width(), bitmap_guard.height());

        unsafe {
            let content_view = self.ns_view.window().contentView();
            let rect = content_view.bounds();

            let _guard = bitmap_guard.ipc_read();

            // Create the CGImage from memory pixels buffer.
            let data_provider = CGDataProvider::from_slice(bitmap_guard.get_pixels());
            let cg_image = CGImage::new(
                width as usize,
                height as usize,
                8,
                32,
                width as usize * 4,
                &self.color_space,
                kCGImageAlphaLast,
                &data_provider,
                false,
                kCGRenderingIntentDefault,
            );
            let cg_img_ref = cg_image.as_ref();

            // Create NSImage by CGImage
            let image_size = NSSize::new(rect.size.width, rect.size.height);
            let ns_image = NSImage::alloc(nil);
            let ns_image: id = msg_send![ns_image, initWithCGImage:cg_img_ref size:image_size];

            // Set NSImage to NSImageView
            if self.ns_image_view.is_none() || bitmap_guard.is_resized() {
                let ns_image_view =
                    NSImageView::initWithFrame_(NSImageView::alloc(nil), rect).autorelease();

                let old_ns_img = self.ns_image_view.replace(ns_image_view);
                if let Some(old_ns_img) = old_ns_img {
                    old_ns_img.removeFromSuperview()
                }

                content_view.addSubview_(ns_image_view);
            }
            self.ns_image_view.as_mut().unwrap().setImage_(ns_image);

            let _: id = msg_send![ns_image, release];

            drop(_guard);
            drop(bitmap_guard);
            self.bitmap.write().reset_resized();
        }
    }
}
