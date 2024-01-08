#![cfg(windows_platform)]
pub(crate) mod win32_window;

use self::win32_window::Win32Window;
use super::{
    ipc_inner_agent::InnerAgent, logic_window::LogicWindow, physical_window::PhysicalWindow,
    PlatformContext,
};
use crate::{
    primitive::Message,
    runtime::window_context::{
        InputReceiver, InputSender, LogicWindowContext, OutputReceiver, PhysicalWindowContext,
    },
    window::win_config::{self, WindowConfig},
    winit::event_loop::EventLoopBuilder,
};
use crate::{
    primitive::{
        bitmap::Bitmap,
        shared_channel::{self},
    },
    runtime::window_context::OutputSender,
};
use std::sync::{mpsc::channel, Arc};
use tipc::{ipc_master::IpcMaster, RwLock, WithIpcMaster};
use tlib::winit::{
    event_loop::{EventLoopProxy, EventLoopWindowTarget},
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
};
use windows::Win32::Foundation::HWND;

pub(crate) struct PlatformWin32<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    /// Shared memory ipc
    master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
    main_win_create: bool,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformWin32<T, M> {
    #[inline]
    pub fn new() -> Self {
        Self {
            master: None,
            main_win_create: true,
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
    fn initialize(&mut self) {}

    fn create_window(
        &self,
        win_config: WindowConfig,
        parent: Option<RawWindowHandle>,
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
        let bitmap = Arc::new(RwLock::new(Bitmap::new(width, height, inner_agent)));

        let (window, event_loop) = if let Some(target) = target {
            let window = win_config::build_window(win_config, target, parent).unwrap();

            (window, None)
        } else {
            let event_loop = EventLoopBuilder::<Message>::with_user_event()
                .build()
                .unwrap();
            let window = win_config::build_window(win_config, &event_loop, parent).unwrap();

            (window, Some(event_loop))
        };

        let window_id = window.id();
        let window_handle = window.window_handle().unwrap().as_raw();
        let hwnd = match window_handle {
            RawWindowHandle::Win32(hwnd) => HWND(hwnd.hwnd.into()),
            _ => unreachable!(),
        };
        let event_loop_proxy = if let Some(proxy) = proxy {
            proxy
        } else {
            event_loop.as_ref().unwrap().create_proxy()
        };

        let (input_sender, input_receiver) = channel::<Message>();

        // Create the shared channel.
        let (shared_channel, user_ipc_event_sender) =
            if self.master.is_some() && self.main_win_create {
                let (user_ipc_event_sender, user_ipc_event_receiver) = channel();
                let shared_channel = shared_channel::master_channel(
                    self.master.as_ref().unwrap().clone(),
                    user_ipc_event_receiver,
                );
                (Some(shared_channel), Some(user_ipc_event_sender))
            } else {
                (None, None)
            };

        (
            LogicWindow::master(
                window_id,
                window_handle,
                bitmap.clone(),
                self.master.clone(),
                shared_channel,
                LogicWindowContext {
                    output_sender: OutputSender::EventLoopProxy(event_loop_proxy),
                    input_receiver: InputReceiver(input_receiver),
                },
            ),
            PhysicalWindow::Win32(Win32Window::new(
                window_id,
                window,
                hwnd,
                bitmap,
                self.master.clone(),
                PhysicalWindowContext(
                    OutputReceiver::EventLoop(event_loop),
                    InputSender(input_sender),
                ),
                user_ipc_event_sender,
            )),
        )
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> WithIpcMaster<T, M>
    for PlatformWin32<T, M>
{
    fn proc_ipc_master(&mut self, master: tipc::ipc_master::IpcMaster<T, M>) {
        self.master = Some(Arc::new(RwLock::new(master)))
    }
}
