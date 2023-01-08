use std::ptr::null_mut;
use super::PlatformContext;
use crate::graphics::bitmap::Bitmap;
use skia_safe::{AlphaType, ColorSpace, ColorType, ImageInfo};

pub struct PlatformIpc {
    title: String,
    width: i32,
    height: i32,
    bitmap: Bitmap,
    image_info: ImageInfo,
}

impl PlatformContext for PlatformIpc {
    type Type = PlatformIpc;

    fn new(title: &str, width: i32, height: i32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            bitmap: Bitmap::new(null_mut(), width, height),
            image_info: ImageInfo::new(
                (width, height),
                ColorType::BGRA8888,
                AlphaType::Premul,
                ColorSpace::new_srgb(),
            ),
        }
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        todo!()
    }

    fn close(&self) {
        todo!()
    }

    fn context_bitmap(&self) -> &Bitmap {
        &self.bitmap
    }

    fn image_info(&self) -> &ImageInfo {
        &self.image_info
    }

    fn handle_platform_event(&self) {
        todo!()
    }
}
