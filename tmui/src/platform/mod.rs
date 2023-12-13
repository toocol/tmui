pub(crate) mod win32;
pub(crate) mod macos;
pub(crate) mod ipc;
pub(crate) mod linux;
pub(crate) mod logic_window;
pub(crate) mod physical_window;

#[cfg(all(not(x11_platform), not(wayland_platform), free_unix))]
compile_error!("Please select a feature to build for unix: `x11`, `wayland`");

use std::sync::Arc;

use crate::primitive::bitmap::Bitmap;
use tipc::RwLock;

pub(crate) use ipc::*;
#[cfg(macos_platform)]
pub(crate) use macos::*;
#[cfg(wayland_platform)]
pub(crate) use linux::wayland::*;
#[cfg(x11_platform)]
pub(crate) use linux::x11::*;
#[cfg(windows_platform)]
pub(crate) use win32::*;

use self::logic_window::LogicWindow;
use self::physical_window::PhysicalWindow;

#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum PlatformType {
    #[cfg(windows_platform)]
    #[default]
    Win32,
    #[cfg(x11_platform)]
    #[default]
    LinuxX11,
    #[cfg(wayland_platform)]
    #[default]
    LinuxWayland,
    #[cfg(macos_platform)]
    #[default]
    Macos,
    Ipc,
}

/// PlatformContext holding the bitmap of specific memory area to renderering image.
/// The raw pointer and memory was created by specific platfom, such as win32([`PlatformWin32`]), ipc channel([`PlatformIpc`])
pub(crate) trait PlatformContext<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send>: 'static {
    /// Initialize the PlatformContext.
    fn initialize(&mut self);

    /// Get the title of platfom.
    fn title(&self) -> &str;

    /// Get the width of the platform.
    fn width(&self) -> u32;

    /// Get the height of the platform.
    fn height(&self) -> u32;

    /// Get current effective `Bitmap` of platform context.
    fn bitmap(&self) -> Arc<RwLock<Bitmap>>;

    /// Create the window and event loop of the specific platform.
    fn create_window(&mut self) -> (LogicWindow<T, M>, PhysicalWindow<T, M>);
}
