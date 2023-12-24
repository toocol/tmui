#![cfg(target_os = "macos")]
use std::sync::{mpsc::Sender, Arc};
use cocoa::{
    appkit::{NSImage, NSImageView, NSView, NSWindow, NSEvent},
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
use tipc::{ipc_master::IpcMaster, RwLock};
use crate::{primitive::bitmap::Bitmap, runtime::window_context::PhysicalWindowContext};

pub(crate) struct MacosWindow<T: 'static + Copy + Send + Sync, M: 'static + Copy + Send + Sync> {
    ns_view: id,
    ns_image_view: Option<id>,
    color_space: CGColorSpace,

    bitmap: Arc<RwLock<Bitmap>>,

    pub master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
    pub context: Option<PhysicalWindowContext>,
    pub user_ipc_event_sender: Option<Sender<Vec<T>>>,
}

impl<T: 'static + Copy + Send + Sync, M: 'static + Copy + Send + Sync> MacosWindow<T, M> {
    #[inline]
    pub fn new(
        ns_view: id,
        bitmap: Arc<RwLock<Bitmap>>,
        master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
        context: PhysicalWindowContext,
        user_ipc_event_sender: Option<Sender<Vec<T>>>,
    ) -> Self {
        Self {
            ns_view,
            ns_image_view: None,
            color_space: unsafe { CGColorSpace::create_with_name(kCGColorSpaceSRGB).unwrap() },
            bitmap,
            master,
            context: Some(context),
            user_ipc_event_sender,
        }
    }

    #[inline]
    pub fn request_redraw(&mut self, window: &tlib::winit::window::Window) {
        window.request_redraw();
    }

    pub fn redraw(&mut self) {
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