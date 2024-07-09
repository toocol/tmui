#![cfg(target_os = "macos")]
pub(crate) mod macos_window;

use crate::{
    backend::BackendType,
    primitive::Message,
    primitive::{bitmap::Bitmap, shared_channel},
    runtime::window_context::{
        InputReceiver, InputSender, LogicWindowContext, OutputReceiver, OutputSender,
        PhysicalWindowContext,
    },
    window::win_config::WindowConfig,
};
use cocoa::{
    appkit::{
        NSApp, NSApplication, NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
    },
    base::id,
};
use objc::runtime::Object;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::{
    cell::Cell,
    sync::{mpsc::channel, Arc},
};
use tipc::{ipc_master::IpcMaster, parking_lot::RwLock, WithIpcMaster};
use tlib::{
    figure::Point,
    winit::event_loop::{EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget},
};

use self::macos_window::MacosWindow;

use super::{
    ipc_inner_agent::InnerAgent, logic_window::LogicWindow, physical_window::PhysicalWindow,
    PlatformContext, PlatformType,
};

pub(crate) struct PlatformMacos<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    // Ipc shared memory context.
    master: Option<Arc<RwLock<IpcMaster<T, M>>>>,

    main_win_create: Cell<bool>,
    platform_type: PlatformType,
    backend_type: BackendType,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformMacos<T, M> {
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
    for PlatformMacos<T, M>
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

        unsafe {
            let ns_app = NSApp();
            ns_app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
        }

        let window_id = window.id();
        let window_handle = window.raw_window_handle();
        let ns_view: id = match window_handle {
            RawWindowHandle::AppKit(id) => id.ns_view as *mut Object,
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

        let init_position = if let Ok(pos) = window.outer_position() {
            Point::new(pos.x, pos.y)
        } else {
            Point::new(0, 0)
        };
        let (mut logic_window, physical_window) = (
            LogicWindow::master(
                window_id,
                gl_env.clone(),
                bitmap.clone(),
                self.master.clone(),
                shared_channel,
                LogicWindowContext {
                    output_sender: OutputSender::EventLoopProxy(event_loop_proxy),
                    input_receiver: InputReceiver(input_receiver),
                },
                init_position,
            ),
            PhysicalWindow::Macos(MacosWindow::new(
                window_id,
                window,
                ns_view,
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
    for PlatformMacos<T, M>
{
    fn proc_ipc_master(&mut self, master: tipc::ipc_master::IpcMaster<T, M>) {
        self.master = Some(Arc::new(RwLock::new(master)))
    }
}
