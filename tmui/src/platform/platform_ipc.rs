use std::ptr::null_mut;
use super::PlatformContext;
use crate::graphics::bitmap::Bitmap;

pub struct PlatformIpc {
    title: String,
    width: i32,
    height: i32,
    bitmap: Bitmap,
}

impl PlatformContext for PlatformIpc {
    type Type = PlatformIpc;

    fn new(title: &str, width: i32, height: i32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            bitmap: Bitmap::new(null_mut(), width, height),
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

    fn handle_platform_event(&self) {
        todo!()
    }

    fn send_message(&self, _message: super::Message) {
        todo!()
    }
}
