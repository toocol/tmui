#![cfg(windows_platform)]
pub(crate) mod win32_window;

use self::win32_window::Win32Window;

use super::{logic_window::LogicWindow, physical_window::PhysicalWindow, PlatformContext};
use crate::{
    primitive::Message,
    runtime::window_context::{
        InputReceiver, InputSender, LogicWindowContext, OutputReceiver, PhysicalWindowContext,
    },
    winit::{
        dpi::{PhysicalSize, Size},
        event_loop::EventLoopBuilder,
        window::WindowBuilder,
    },
};
use crate::{
    primitive::{
        bitmap::Bitmap,
        shared_channel::{self, SharedChannel},
    },
    runtime::window_context::OutputSender,
};
use std::sync::{
        mpsc::{channel, Sender},
        Arc,
    };
use tipc::{
    ipc_master::IpcMaster, IpcNode, RwLock, WithIpcMaster,
};
use tlib::winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::HWND;

pub(crate) struct PlatformWin32<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    title: String,
    width: u32,
    height: u32,

    bitmap: Option<Arc<RwLock<Bitmap>>>,

    /// The fileds associated with win32
    hwnd: Option<HWND>,

    /// Shared memory ipc
    master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
    user_ipc_event_sender: Option<Sender<Vec<T>>>,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformWin32<T, M> {
    #[inline]
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            bitmap: None,
            hwnd: None,
            master: None,
            user_ipc_event_sender: None,
        }
    }

    // Wrap trait `PlatfomContext` with [`Box`].
    #[inline]
    pub fn wrap(self) -> Box<dyn PlatformContext<T, M>> {
        Box::new(self)
    }

    #[inline]
    pub fn shared_channel(&mut self) -> SharedChannel<T, M> {
        let (sender, receiver) = channel();
        self.user_ipc_event_sender = Some(sender);
        shared_channel::master_channel(self.master.as_ref().unwrap().clone(), receiver)
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformContext<T, M>
    for PlatformWin32<T, M>
{
    fn initialize(&mut self) {
        match self.master {
            Some(ref master) => {
                let guard = master.read();
                self.bitmap = Some(Arc::new(RwLock::new(Bitmap::from_raw_pointer(
                    guard.buffer_raw_pointer(),
                    self.width,
                    self.height,
                    guard.buffer_lock(),
                    guard.name(),
                    guard.ty(),
                ))));
            }
            None => {
                self.bitmap = Some(Arc::new(RwLock::new(Bitmap::new(self.width, self.height))));
            }
        }
    }

    #[inline]
    fn title(&self) -> &str {
        &self.title
    }

    #[inline]
    fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    fn bitmap(&self) -> Arc<RwLock<Bitmap>> {
        self.bitmap.as_ref().unwrap().clone()
    }

    fn create_window(&mut self) -> (LogicWindow<T, M>, PhysicalWindow<T, M>) {
        let event_loop = EventLoopBuilder::<Message>::with_user_event()
            .build()
            .unwrap();

        let window = WindowBuilder::new()
            .with_title(&self.title)
            .with_inner_size(Size::Physical(PhysicalSize::new(self.width, self.height)))
            .build(&event_loop)
            .unwrap();

        let window_handle = window.window_handle().unwrap().as_raw();
        match window_handle {
            RawWindowHandle::Win32(hwnd) => self.hwnd = Some(HWND(hwnd.hwnd.into())),
            _ => {}
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
                self.bitmap(),
                self.master.clone(),
                shared_channel,
                LogicWindowContext {
                    output_sender: OutputSender::EventLoopProxy(event_loop_proxy),
                    input_receiver: InputReceiver(input_receiver),
                },
            ),
            PhysicalWindow::Win32(Win32Window::new(
                self.hwnd.unwrap(),
                self.bitmap(),
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
