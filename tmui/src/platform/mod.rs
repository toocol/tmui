pub(crate) mod ipc;
pub(crate) mod linux;
pub(crate) mod logic_window;
pub(crate) mod macos;
pub(crate) mod physical_window;
pub(crate) mod win32;

#[cfg(all(not(x11_platform), not(wayland_platform), free_unix))]
compile_error!("Please select a feature to build for unix: `x11`, `wayland`");

pub(crate) use ipc::*;
#[cfg(wayland_platform)]
pub(crate) use linux::wayland::*;
#[cfg(x11_platform)]
pub(crate) use linux::x11::*;
#[cfg(macos_platform)]
pub(crate) use macos::*;
use tlib::winit::event_loop::{EventLoopWindowTarget, EventLoopProxy};
use tlib::winit::raw_window_handle::RawWindowHandle;
#[cfg(windows_platform)]
pub(crate) use win32::*;

use crate::primitive::Message;
use crate::window::win_config::WindowConfig;

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
pub(crate) trait PlatformContext<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send>:
    'static
{
    /// Initialize the PlatformContext.
    fn initialize(&mut self);

    /// Create the window and event loop of the specific platform.
    fn create_window(
        &self,
        win_config: WindowConfig,
        parent: Option<RawWindowHandle>,
        target: Option<&EventLoopWindowTarget<Message>>,
        proxy: Option<EventLoopProxy<Message>>,
    ) -> (LogicWindow<T, M>, PhysicalWindow<T, M>);
}
