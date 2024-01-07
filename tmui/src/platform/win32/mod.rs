#![cfg(windows_platform)]
pub(crate) mod win32_window;

use self::win32_window::Win32Window;
use super::{
    ipc_inner_agent::InnerAgent,
    logic_window::LogicWindow,
    physical_window::PhysicalWindow,
    win_config::{self, WindowConfig},
    PlatformContext,
};
use crate::{
    primitive::Message,
    runtime::window_context::{
        InputReceiver, InputSender, LogicWindowContext, OutputReceiver, PhysicalWindowContext,
    },
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
use tlib::winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::HWND;

pub(crate) struct PlatformWin32<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    /// Shared memory ipc
    master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformWin32<T, M> {
    #[inline]
    pub fn new() -> Self {
        Self {
            master: None,
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
    ) -> (LogicWindow<T, M>, PhysicalWindow<T, M>) {
        let inner_agent = if self.master.is_some() {
            let master = self.master.as_ref().unwrap().clone();
            Some(InnerAgent::master(master))
        } else {
            None
        };

        let (width, height) = win_config.size();
        let bitmap = Arc::new(RwLock::new(Bitmap::new(width, height, inner_agent)));

        let event_loop = EventLoopBuilder::<Message>::with_user_event()
            .build()
            .unwrap();

        let window = win_config::build_window(win_config, &event_loop).unwrap();

        let window_id = window.id();
        let window_handle = window.window_handle().unwrap().as_raw();
        let hwnd = match window_handle {
            RawWindowHandle::Win32(hwnd) => HWND(hwnd.hwnd.into()),
            _ => unreachable!(),
        };
        let event_loop_proxy = event_loop.create_proxy();

        let (input_sender, input_receiver) = channel::<Message>();

        // Create the shared channel.
        let (shared_channel, user_ipc_event_sender) = if self.master.is_some() {
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
                hwnd,
                bitmap,
                self.master.clone(),
                PhysicalWindowContext::Default(
                    window,
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
