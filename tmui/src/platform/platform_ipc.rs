use super::{
    window_context::{OutputSender, WindowContext},
    Message, PlatformContext,
};
use crate::graphics::bitmap::Bitmap;
use std::{
    ptr::null_mut,
    sync::mpsc::{channel, Sender},
};

pub(crate) struct PlatformIpc {
    title: String,
    width: u32,
    height: u32,

    front_bitmap: Bitmap,
    back_bitmap: Bitmap,
}

impl PlatformIpc {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            front_bitmap: Bitmap::new(null_mut(), width, height),
            back_bitmap: Bitmap::new(null_mut(), width, height),
        }
    }

    // Wrap trait `PlatfomContext` with [`Box`].
    pub fn wrap(self) -> Box<dyn PlatformContext> {
        Box::new(self)
    }
}

impl PlatformContext for PlatformIpc {
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

    fn set_input_sender(&mut self, _input_sender: Sender<super::Message>) {
        todo!()
    }

    fn input_sender(&self) -> &Sender<super::Message> {
        todo!()
    }

    fn create_window(&mut self) -> WindowContext {
        let (output_sender, output_receiver) = channel::<Message>();
        WindowContext::Ipc(output_receiver, Some(OutputSender::Sender(output_sender)))
    }

    fn platform_main(&self, _window_context: WindowContext) {
        todo!()
    }

    fn redraw(&mut self) {
        todo!()
    }
}
