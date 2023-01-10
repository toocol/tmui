use super::PlatformContext;
use crate::graphics::bitmap::Bitmap;
use std::{ptr::null_mut, sync::mpsc::Sender};

pub struct PlatformIpc {
    title: String,
    width: i32,
    height: i32,

    front_bitmap: Bitmap,
    back_bitmap: Bitmap,
}

impl PlatformContext for PlatformIpc {
    type Type = PlatformIpc;

    fn new(title: &str, width: i32, height: i32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            front_bitmap: Bitmap::new(null_mut(), width, height),
            back_bitmap: Bitmap::new(null_mut(), width, height),
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

    fn front_bitmap(&self) -> Bitmap {
        self.front_bitmap
    }

    fn back_bitmap(&self) -> Bitmap {
        self.back_bitmap
    }

    fn handle_platform_event(&self) {
        todo!()
    }

    fn send_message(&self, _message: super::Message) {
        todo!()
    }

    fn set_input_sender(&mut self, _input_sender: Sender<super::Message>) {
        todo!()
    }
}
