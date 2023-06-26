pub(crate) mod message;
pub(crate) mod platform_ipc;
pub(crate) mod platform_macos;
pub(crate) mod platform_win32;
pub(crate) mod shared_channel;
pub(crate) mod window_context;
pub(crate) mod window_process;

use std::sync::mpsc::Sender;

use crate::graphics::bitmap::Bitmap;
pub use message::*;
pub(crate) use platform_ipc::*;
#[cfg(target_os = "macos")]
pub(crate) use platform_macos::*;
#[cfg(target_os = "windows")]
pub(crate) use platform_win32::*;
use tlib::figure::Rect;

use self::window_context::WindowContext;

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
pub(crate) trait PlatformContext: 'static {
    /// Initialize the PlatformContext.
    fn initialize(&mut self);

    /// Get the title of platfom.
    fn title(&self) -> &str;

    /// Get the width of the platform.
    fn width(&self) -> u32;

    /// Get the height of the platform.
    fn height(&self) -> u32;

    /// Get the region of the platform.
    fn region(&self) -> Rect;

    /// Resize the platform by specific width and height.
    fn resize(&mut self, width: u32, height: u32);

    /// Get current effective `Bitmap` of platform context.
    fn bitmap(&self) -> Bitmap;

    /// Set the `input_sender` to transfer user input.
    fn set_input_sender(&mut self, input_sender: Sender<Message>);

    /// Get the `input_sender` to transfer user input.
    fn input_sender(&self) -> &Sender<Message>;

    /// Create the window and event loop of the specific platform.
    fn create_window(&mut self) -> WindowContext;

    /// The platform main function.
    fn platform_main(&mut self, window_context: WindowContext);

    /// Redraw the window.
    fn redraw(&mut self);

    /// Only avalid on shared_memory was opened.
    ///
    /// wait until another process was invoke [`PlatformContext::signal`]
    fn wait(&self);

    /// Only avalid on shared_memory was opened.
    ///
    /// sginal the process which invoke [`PlatformContext::wait`] to carry on.
    fn signal(&self);

    /// For shared-memory application, add shared region rect
    fn add_shared_region(&self, id: &'static str, rect: Rect);
}
