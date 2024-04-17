pub(crate) mod gl_bootstrap;
pub(crate) mod ipc;
pub(crate) mod linux;
pub(crate) mod logic_window;
pub(crate) mod macos;
pub(crate) mod physical_window;
pub(crate) mod win32;

#[cfg(all(not(x11_platform), not(wayland_platform), free_unix))]
compile_error!("Please select a feature to build for unix: `x11`, `wayland`");

use std::sync::Arc;

pub(crate) use ipc::*;
#[cfg(wayland_platform)]
pub(crate) use linux::wayland::*;
#[cfg(x11_platform)]
pub(crate) use linux::x11::*;
#[cfg(macos_platform)]
pub(crate) use macos::*;
use tlib::typedef::WinitWindow;
use tlib::winit::event_loop::{EventLoopProxy, EventLoopWindowTarget};
#[cfg(windows_platform)]
pub(crate) use win32::*;

use crate::backend::BackendType;
use crate::primitive::Message;
use crate::window::win_config::{self, WindowConfig};

use self::gl_bootstrap::GlEnv;
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
    /// Create the window and event loop of the specific platform.
    fn create_window(
        &self,
        win_config: WindowConfig,
        target: Option<&EventLoopWindowTarget<Message>>,
        proxy: Option<EventLoopProxy<Message>>,
    ) -> (LogicWindow<T, M>, PhysicalWindow<T, M>);
}

pub(crate) fn make_window(
    win_config: WindowConfig,
    target: &EventLoopWindowTarget<Message>,
    backend_type: BackendType,
) -> (WinitWindow, Option<Arc<GlEnv>>) {
    if backend_type == BackendType::OpenGL {
        let (win, gl_env) =
            gl_bootstrap::bootstrap_gl_window(target, win_config.create_window_builder())
                .expect("bootstrap gl window failed.");

        (win, Some(gl_env))
    } else {
        let window = win_config::build_window(win_config, target).expect("build_window failed.");
        (window, None)
    }
}
