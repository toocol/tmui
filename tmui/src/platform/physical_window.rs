use super::ipc_window::IpcWindow;
#[cfg(windows_platform)]
use super::win32_window::Win32Window;
#[cfg(macos_platform)]
use super::macos_window::MacosWindow;
#[cfg(wayland_platform)]
use super::linux::wayland_window::WaylandWindow;
#[cfg(x11_platform)]
use super::linux::x11_window::X11Window;

#[cfg(windows_platform)]
pub(crate) type PhysWindow<T, M> = Win32Window<T, M>;

#[cfg(macos_platform)]
pub(crate) type PhysWindow<T, M> = MacosWindow<T, M>;

#[cfg(wayland_platform)]
pub(crate) type PhysWindow<T, M> = WaylandWindow<T, M>;

#[cfg(x11_platform)]
pub(crate) type PhysWindow<T, M> = X11Window<T, M>;

pub(crate) enum PhysicalWindow<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    Ipc(IpcWindow<T, M>),

    #[cfg(windows_platform)]
    Win32(PhysWindow<T, M>),

    #[cfg(macos_platform)]
    Macos(PhysWindow<T, M>),

    #[cfg(wayland_platform)]
    Wayland(PhysWindow<T, M>),

    #[cfg(x11_platform)]
    X11(PhysWindow<T, M>),
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PhysicalWindow<T, M> {
    #[inline]
    pub(crate) fn into_phys_window(self) -> PhysWindow<T, M> {
        match self {
            Self::Ipc(_) => unreachable!(),
            #[cfg(windows_platform)]
            Self::Win32(win) => win,
            #[cfg(macos_platform)]
            Self::Macos(win) => win,
            #[cfg(wayland_platform)]
            Self::Wayland(win) => win,
            #[cfg(x11_platform)]
            Self::X11(win) => win,
        }
    }
}