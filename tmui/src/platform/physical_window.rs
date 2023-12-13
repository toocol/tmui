use super::ipc_window::IpcWindow;
#[cfg(wayland_platform)]
use super::linux::wayland_window::WaylandWindow;
#[cfg(x11_platform)]
use super::linux::x11_window::X11Window;
#[cfg(windows_platform)]
use super::win32_window::Win32Window;

#[cfg(windows_platform)]
pub(crate) type PhysWindow<T, M> = Win32Window<T, M>;

#[cfg(wayland_platform)]
pub(crate) type PhysWindow<T, M> = WaylandWindow<T, M>;

#[cfg(x11_platform)]
pub(crate) type PhysWindow<T, M> = X11Window<T, M>;

pub(crate) enum PhysicalWindow<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    Ipc(IpcWindow<T, M>),

    #[cfg(windows_platform)]
    Win32(PhysWindow<T, M>),

    #[cfg(macos_platform)]
    Macos,

    #[cfg(wayland_platform)]
    Wayland(PhysWindow<T, M>),

    #[cfg(x11_platform)]
    X11(PhysWindow<T, M>),
}
