pub(crate) mod platform_ipc;
pub(crate) mod platform_macos;
pub(crate) mod platform_wayland;
pub(crate) mod platform_win32;
pub(crate) mod platform_x11;

#[cfg(all(not(x11_platform), not(wayland_platform), free_unix))]
compile_error!("Please select a feature to build for unix: `x11`, `wayland`");

use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;

use crate::primitive::{bitmap::Bitmap, Message};
use crate::runtime::window_context::WindowContext;
use tlib::{figure::Rect, winit::window::Window};

pub(crate) use platform_ipc::*;
#[cfg(macos_platform)]
pub(crate) use platform_macos::*;
#[cfg(wayland_platform)]
pub(crate) use platform_wayland::*;
#[cfg(x11_platform)]
pub(crate) use platform_x11::*;
#[cfg(windows_platform)]
pub(crate) use platform_win32::*;

#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum PlatformType {
    #[cfg(windows_platform)]
    #[default]
    Win32,
    #[cfg(free_unix)]
    #[default]
    LinuxX11,
    #[cfg(free_unix)]
    LinuxWayland,
    #[cfg(macos_platform)]
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
    fn bitmap(&self) -> Arc<RwLock<Bitmap>>;

    /// Set the `input_sender` to transfer user input.
    fn set_input_sender(&mut self, input_sender: Sender<Message>);

    /// Get the `input_sender` to transfer user input.
    fn input_sender(&self) -> &Sender<Message>;

    /// Create the window and event loop of the specific platform.
    fn create_window(&mut self) -> WindowContext;

    /// The platform main function.
    fn platform_main(&mut self, window_context: WindowContext);

    /// Request to redraw the window.
    fn request_redraw(&mut self, window: &Window);

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
