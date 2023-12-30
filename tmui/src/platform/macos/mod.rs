#![cfg(target_os = "macos")]
pub(crate) mod macos_window;

use crate::{
    primitive::Message,
    primitive::{bitmap::Bitmap, shared_channel},
    runtime::window_context::{
        InputReceiver, InputSender, LogicWindowContext, OutputReceiver, OutputSender,
        PhysicalWindowContext,
    },
};
use cocoa::{
    appkit::{
        NSApp, NSApplication, NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
    },
    base::id,
};
use objc::runtime::Object;
use std::sync::{mpsc::channel, Arc};
use tipc::{ipc_master::IpcMaster, RwLock, WithIpcMaster};
use tlib::winit::{
    event_loop::EventLoopBuilder,
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
};

use self::macos_window::MacosWindow;

use super::{logic_window::LogicWindow, physical_window::PhysicalWindow, PlatformContext, win_config::{WindowConfig, self}, ipc_inner_agent::InnerAgent};

pub(crate) struct PlatformMacos<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    // Ipc shared memory context.
    master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformMacos<T, M> {
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
    for PlatformMacos<T, M>
{
    fn initialize(&mut self) {}

    fn create_window(&mut self, win_config: WindowConfig) -> (LogicWindow<T, M>, PhysicalWindow<T, M>) {
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

        unsafe {
            let ns_app = NSApp();
            ns_app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
        }

        let window = win_config::build_window(win_config, &event_loop).unwrap();

        let window_handle = window.window_handle().unwrap().as_raw();
        let ns_view: id = match window_handle {
            RawWindowHandle::AppKit(id) => id.ns_view.as_ptr() as *mut Object,
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
                bitmap.clone(),
                self.master.clone(),
                shared_channel,
                LogicWindowContext {
                    output_sender: OutputSender::EventLoopProxy(event_loop_proxy),
                    input_receiver: InputReceiver(input_receiver),
                },
            ),
            PhysicalWindow::Macos(MacosWindow::new(
                ns_view,
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
    for PlatformMacos<T, M>
{
    fn proc_ipc_master(&mut self, master: tipc::ipc_master::IpcMaster<T, M>) {
        self.master = Some(Arc::new(RwLock::new(master)))
    }
}
