pub(crate) mod ipc_bridge;
pub(crate) mod ipc_inner_agent;
pub(crate) mod ipc_window;

use self::ipc_window::IpcWindow;

use super::{logic_window::LogicWindow, physical_window::PhysicalWindow, PlatformContext, PlatformType};
use crate::{
    platform::ipc_inner_agent::InnerAgent,
    primitive::{
        bitmap::Bitmap,
        shared_channel::{self},
        Message,
    },
    runtime::window_context::{
        InputReceiver, InputSender, LogicWindowContext, OutputReceiver, OutputSender,
        PhysicalWindowContext,
    },
    window::win_config::WindowConfig, backend::BackendType,
};
use std::sync::{mpsc::channel, Arc};
use tipc::{ipc_slave::IpcSlave, IpcNode, parking_lot::RwLock, WithIpcSlave};
use tlib::winit::event_loop::{EventLoopWindowTarget, EventLoopProxy};

pub(crate) struct PlatformIpc<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    /// Shared memory ipc slave
    slave: Option<Arc<RwLock<IpcSlave<T, M>>>>,
    shared_widget_id: Option<&'static str>,

    platform_type: PlatformType,
    backend_type: BackendType,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformIpc<T, M> {
    #[inline]
    pub fn new(platform_type: PlatformType, backend_type: BackendType) -> Self {
        Self {
            slave: None,
            shared_widget_id: None,
            platform_type,
            backend_type,
        }
    }

    // Wrap trait `PlatfomContext` with [`Box`].
    #[inline]
    pub fn wrap(self) -> Box<dyn PlatformContext<T, M>> {
        Box::new(self)
    }

    #[inline]
    pub fn set_shared_widget_id(&mut self, id: &'static str) {
        self.shared_widget_id = Some(id)
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformContext<T, M>
    for PlatformIpc<T, M>
{
    #[inline]
    fn create_window(
        &self,
        _: WindowConfig,
        _: Option<&EventLoopWindowTarget<Message>>,
        _: Option<EventLoopProxy<Message>>,
    ) -> (LogicWindow<T, M>, PhysicalWindow<T, M>) {
        let slave_clone = self.slave.as_ref().unwrap().clone();
        let inner_agent = InnerAgent::slave(slave_clone);

        let slave = self.slave.as_ref().unwrap().read();
        let bitmap = Arc::new(RwLock::new(Bitmap::from_raw_pointer(
            slave.buffer_raw_pointer(),
            slave.width(),
            slave.height(),
            slave.buffer_lock(),
            inner_agent,
        )));

        let (input_sender, input_receiver) = channel::<Message>();
        let (output_sender, output_receiver) = channel::<Message>();

        // Create the shared channel
        let (user_ipc_event_sender, user_ipc_event_receiver) = channel();
        let shared_channel = shared_channel::slave_channel(
            self.slave.as_ref().unwrap().clone(),
            user_ipc_event_receiver,
        );

        let (mut logic_window, physical_window) = (
            LogicWindow::slave(
                bitmap,
                self.shared_widget_id.unwrap(),
                self.slave.as_ref().unwrap().clone(),
                Some(shared_channel),
                LogicWindowContext {
                    output_sender: OutputSender::Sender(output_sender),
                    input_receiver: InputReceiver(input_receiver),
                },
            ),
            PhysicalWindow::Ipc(IpcWindow::new(
                self.slave.as_ref().unwrap().clone(),
                PhysicalWindowContext(
                    OutputReceiver::Receiver(output_receiver),
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

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> WithIpcSlave<T, M>
    for PlatformIpc<T, M>
{
    fn proc_ipc_slave(&mut self, slave: tipc::ipc_slave::IpcSlave<T, M>) {
        self.slave = Some(Arc::new(RwLock::new(slave)))
    }
}
