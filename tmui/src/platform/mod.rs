pub mod platform_ipc;
pub mod platform_win32;

pub use platform_ipc::*;
pub use platform_win32::*;

use crate::graphics::bitmap::Bitmap;
use skia_safe::ImageInfo;

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
        Box::new(Some(self))
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

    /// Get the `Bitmap` of platform context.
    fn context_bitmap(&self) -> &Bitmap;

    /// Get the `ImageInfo` of platform context.
    fn image_info(&self) -> &ImageInfo;
}

pub trait PlatformContextWrapper {
    fn title(&self) -> &str;

    fn width(&self) -> i32;

    fn height(&self) -> i32;

    fn resize(&mut self, width: i32, height: i32);

    fn close(&self);

    fn context_bitmap(&self) -> &Bitmap;

    fn image_info(&self) -> &ImageInfo;
}

impl<T: PlatformContext> PlatformContextWrapper for Option<T> {
    fn title(&self) -> &str {
        self.as_ref().unwrap().title()
    }

    fn width(&self) -> i32 {
        self.as_ref().unwrap().width()
    }

    fn height(&self) -> i32 {
        self.as_ref().unwrap().height()
    }

    fn resize(&mut self, width: i32, height: i32) {
        self.as_mut().unwrap().resize(width, height)
    }

    fn close(&self) {
        self.as_ref().unwrap().close()
    }

    fn context_bitmap(&self) -> &Bitmap {
        self.as_ref().unwrap().context_bitmap()
    }

    fn image_info(&self) -> &ImageInfo {
        self.as_ref().unwrap().image_info()
    }
}
