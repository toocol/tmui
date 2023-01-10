pub mod message;
pub mod platform_ipc;
pub mod platform_win32;

use std::{cell::RefCell, sync::mpsc::Sender};

pub use message::*;
pub use platform_ipc::*;
pub use platform_win32::*;

use crate::graphics::bitmap::Bitmap;

#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum PlatformType {
    #[cfg(target_os = "windows")]
    #[default]
    Win32,
    #[cfg(target_os = "linux")]
    #[default]
    Linux,
    #[cfg(target_os = "macos")]
    #[default]
    Macos,
    Ipc,
}

/// PlatformContext holding the bitmap of specific memory area to renderering image.
/// The raw pointer and memory was created by specific platfom, such as win32([`PlatformWin32`]), ipc channel([`PlatformIpc`])
pub trait PlatformContext: Sized + 'static {
    type Type: PlatformContext;

    /// The constructer to build the `PlatformContext` by specific width and height.
    fn new(title: &str, width: i32, height: i32) -> Self;

    /// Wrap trait `PlatfomContext` it self to dyn trait [`PlatfromContextWrapper`].
    fn wrap(self) -> Box<dyn PlatformContextWrapper> {
        Box::new(RefCell::new(self))
    }

    /// Get the title of platfom.
    fn title(&self) -> &str;

    /// Get the width of the platform.
    fn width(&self) -> i32;

    /// Get the height of the platform.
    fn height(&self) -> i32;

    /// Resize the platform by specific width and height.
    fn resize(&mut self, width: i32, height: i32);

    /// Close the `PlatformContext`
    fn close(&self);

    /// Get the front `Bitmap` of platform context.
    fn front_bitmap(&self) -> Bitmap;

    /// Get the front `Bitmap` of platform context.
    fn back_bitmap(&self) -> Bitmap;

    /// Handle the event corresponding to specific platform.
    fn handle_platform_event(&self);

    /// Send [`Message`] to the platform context.
    fn send_message(&self, message: Message);

    /// Set the `input_sender` to transfer user input.
    fn set_input_sender(&mut self, input_sender: Sender<Message>);
}

pub trait PlatformContextWrapper {
    fn title(&self) -> String;

    fn width(&self) -> i32;

    fn height(&self) -> i32;

    fn resize(&self, width: i32, height: i32);

    fn close(&self);

    fn front_bitmap(&self) -> Bitmap;

    fn back_bitmap(&self) -> Bitmap;

    fn handle_platform_event(&self);

    fn send_message(&self, message: Message);

    fn set_input_sender(&self, input_sender: Sender<Message>);
}

impl<T: PlatformContext> PlatformContextWrapper for RefCell<T> {
    fn title(&self) -> String {
        self.borrow().title().to_string()
    }

    fn width(&self) -> i32 {
        self.borrow().width()
    }

    fn height(&self) -> i32 {
        self.borrow().height()
    }

    fn resize(&self, width: i32, height: i32) {
        self.borrow_mut().resize(width, height)
    }

    fn close(&self) {
        self.borrow().close()
    }

    fn front_bitmap(&self) -> Bitmap {
        self.borrow().front_bitmap()
    }

    fn back_bitmap(&self) -> Bitmap {
        self.borrow().back_bitmap()
    }

    fn handle_platform_event(&self) {
        self.borrow().handle_platform_event()
    }

    fn send_message(&self, message: Message) {
        self.borrow().send_message(message)
    }

    fn set_input_sender(&self, input_sender: Sender<Message>) {
        self.borrow_mut().set_input_sender(input_sender)
    }
}
