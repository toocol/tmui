#![cfg(windows_platform)]
pub(crate) mod win32_window;

use self::win32_window::Win32Window;
use super::{
    ipc_inner_agent::InnerAgent, logic_window::LogicWindow, physical_window::PhysicalWindow,
    platform_win_op::HwndGetter, PlatformContext, PlatformType,
};
use crate::{
    backend::BackendType,
    primitive::Message,
    runtime::window_context::{
        InputReceiver, InputSender, LogicWindowContext, OutputReceiver, PhysicalWindowContext,
    },
    window::win_config::WindowConfig,
    winit::event_loop::EventLoopBuilder,
};
use crate::{
    primitive::{
        bitmap::Bitmap,
        shared_channel::{self},
    },
    runtime::window_context::OutputSender,
};
use raw_window_handle::HasRawWindowHandle;
use std::{
    cell::Cell,
    sync::{mpsc::channel, Arc},
};
use tipc::{ipc_master::IpcMaster, parking_lot::RwLock, WithIpcMaster};
use tlib::{
    figure::Point,
    winit::{
        event_loop::{EventLoopProxy, EventLoopWindowTarget},
        raw_window_handle::HasWindowHandle,
    },
};

pub(crate) struct PlatformWin32<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    /// Shared memory ipc
    master: Option<Arc<RwLock<IpcMaster<T, M>>>>,

    main_win_create: Cell<bool>,
    platform_type: PlatformType,
    backend_type: BackendType,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformWin32<T, M> {
    #[inline]
    pub fn new(platform_type: PlatformType, backend_type: BackendType) -> Self {
        Self {
            master: None,
            main_win_create: Cell::new(true),
            platform_type,
            backend_type,
        }
    }

    // Wrap trait `PlatfomContext` with [`Box`].
    #[inline]
    pub fn wrap(self) -> Box<dyn PlatformContext<T, M>> {
        Box::new(self)
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformContext<T, M>
    for PlatformWin32<T, M>
{
    fn create_window(
        &self,
        win_config: WindowConfig,
        target: Option<&EventLoopWindowTarget<Message>>,
        proxy: Option<EventLoopProxy<Message>>,
    ) -> (LogicWindow<T, M>, PhysicalWindow<T, M>) {
        let inner_agent = if self.master.is_some() {
            let master = self.master.as_ref().unwrap().clone();
            Some(InnerAgent::master(master))
        } else {
            None
        };

        let (width, height) = win_config.size();
        let bitmap = Arc::new(RwLock::new(if self.backend_type == BackendType::OpenGL {
            Bitmap::empty(width, height)
        } else {
            Bitmap::new(width, height, inner_agent)
        }));

        let (window, event_loop, gl_env) = if let Some(target) = target {
            let (window, gl_env) = super::make_window(win_config, target, self.backend_type);

            (window, None, gl_env)
        } else {
            let event_loop = EventLoopBuilder::<Message>::with_user_event()
                .build()
                .unwrap();
            let (window, gl_env) = super::make_window(win_config, &event_loop, self.backend_type);

            (window, Some(event_loop), gl_env)
        };

        let window_id = window.id();
        let hwnd = window.raw_window_handle().hwnd();
        let event_loop_proxy = if let Some(proxy) = proxy {
            proxy
        } else {
            event_loop.as_ref().unwrap().create_proxy()
        };

        let (input_sender, input_receiver) = channel::<Message>();

        // Create the shared channel.
        let (shared_channel, user_ipc_event_sender) =
            if self.master.is_some() && self.main_win_create.get() {
                let (user_ipc_event_sender, user_ipc_event_receiver) = channel();
                let shared_channel = shared_channel::master_channel(
                    self.master.as_ref().unwrap().clone(),
                    user_ipc_event_receiver,
                );
                (Some(shared_channel), Some(user_ipc_event_sender))
            } else {
                (None, None)
            };

        self.main_win_create.set(false);

        let init_outer = if let Ok(pos) = window.outer_position() {
            Point::new(pos.x, pos.y)
        } else {
            Point::new(0, 0)
        };
        let init_inner = if let Ok(pos) = window.inner_position() {
            Point::new(pos.x, pos.y)
        } else {
            Point::new(0, 0)
        };
        let (mut logic_window, physical_window) = (
            LogicWindow::master(
                window
                    .window_handle()
                    .expect("Get window handle failed,")
                    .as_raw(),
                window_id,
                gl_env.clone(),
                bitmap.clone(),
                self.master.clone(),
                shared_channel,
                LogicWindowContext {
                    output_sender: OutputSender::EventLoopProxy(event_loop_proxy),
                    input_receiver: InputReceiver(input_receiver),
                },
                (init_outer, init_inner),
            ),
            PhysicalWindow::Win32(Win32Window::new(
                window_id,
                window,
                hwnd,
                bitmap,
                gl_env,
                self.master.clone(),
                PhysicalWindowContext(
                    OutputReceiver::EventLoop(event_loop),
                    InputSender(input_sender),
                ),
                user_ipc_event_sender,
            )),
        );

        logic_window.platform_type = self.platform_type;
        logic_window.backend_type = self.backend_type;

        (logic_window, physical_window)
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> WithIpcMaster<T, M>
    for PlatformWin32<T, M>
{
    fn proc_ipc_master(&mut self, master: tipc::ipc_master::IpcMaster<T, M>) {
        self.master = Some(Arc::new(RwLock::new(master)))
    }
}
