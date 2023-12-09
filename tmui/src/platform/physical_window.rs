use super::{ipc_window::IpcWindow, win32_window::Win32Window};

#[cfg(windows_platform)]
pub(crate) type PhysWindow<T, M> = Win32Window<T, M>;

pub(crate) enum PhysicalWindow<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    Ipc(IpcWindow<T, M>),

    #[cfg(windows_platform)]
    Win32(PhysWindow<T, M>),

    #[cfg(macos_platform)]
    Macos,

    #[cfg(wayland_platform)]
    Wayland,

    #[cfg(x11_platform)]
    X11,
}
