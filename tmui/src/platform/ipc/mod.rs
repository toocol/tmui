pub(crate) mod ipc_bridge;
pub(crate) mod ipc_window;

use self::ipc_window::IpcWindow;

use super::{logic_window::LogicWindow, physical_window::PhysicalWindow, PlatformContext};
use crate::{
    primitive::{
        bitmap::Bitmap,
        shared_channel::{self},
        Message,
    },
    runtime::window_context::{
            InputReceiver, InputSender, LogicWindowContext, OutputReceiver, OutputSender,
            PhysicalWindowContext,
        },
};
use std::sync::{
    mpsc::channel,
    Arc,
};
use tipc::{
    ipc_slave::IpcSlave, IpcNode, RwLock, WithIpcSlave,
};
use tlib::figure::Rect;

pub(crate) struct PlatformIpc<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    title: String,
    region: Rect,

    bitmap: Option<Arc<RwLock<Bitmap>>>,

    /// Shared memory ipc slave
    slave: Option<Arc<RwLock<IpcSlave<T, M>>>>,

    shared_widget_id: Option<&'static str>,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformIpc<T, M> {
    #[inline]
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let region = Rect::new(0, 0, width as i32, height as i32);
        Self {
            title: title.to_string(),
            region,
            bitmap: None,
            slave: None,
            shared_widget_id: None,
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
    fn initialize(&mut self) {
        let slave = self.slave.as_ref().unwrap().read();
        let bitmap = Bitmap::from_raw_pointer(
            slave.buffer_raw_pointer(),
            slave.width(),
            slave.height(),
            slave.buffer_lock(),
            slave.name(),
            slave.ty(),
        );

        self.region = slave
            .region(self.shared_widget_id.unwrap())
            .expect("The `SharedWidget` with id `{}` was not exist.");

        self.bitmap = Some(Arc::new(RwLock::new(bitmap)));
    }

    #[inline]
    fn title(&self) -> &str {
        &self.title
    }

    #[inline]
    fn width(&self) -> u32 {
        self.region.width() as u32
    }

    #[inline]
    fn height(&self) -> u32 {
        self.region.height() as u32
    }

    #[inline]
    fn bitmap(&self) -> Arc<RwLock<Bitmap>> {
        self.bitmap.as_ref().unwrap().clone()
    }

    #[inline]
    fn create_window(&mut self) -> (LogicWindow<T, M>, PhysicalWindow<T, M>) {
        let (input_sender, input_receiver) = channel::<Message>();
        let (output_sender, output_receiver) = channel::<Message>();

        // Create the shared channel
        let (user_ipc_event_sender, user_ipc_event_receiver) = channel();
        let shared_channel = shared_channel::slave_channel(
            self.slave.as_ref().unwrap().clone(),
            user_ipc_event_receiver,
        );

        (
            LogicWindow::slave(
                self.bitmap(),
                self.shared_widget_id.unwrap(),
                self.slave.clone(),
                Some(shared_channel),
                LogicWindowContext {
                    output_sender: OutputSender::Sender(output_sender),
                    input_receiver: InputReceiver(input_receiver),
                },
            ),
            PhysicalWindow::Ipc(IpcWindow::new(
                self.slave.as_ref().unwrap().clone(),
                PhysicalWindowContext::Ipc(
                    OutputReceiver::Receiver(output_receiver),
                    InputSender(input_sender),
                ),
                user_ipc_event_sender,
            )),
        )
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> WithIpcSlave<T, M>
    for PlatformIpc<T, M>
{
    fn proc_ipc_slave(&mut self, slave: tipc::ipc_slave::IpcSlave<T, M>) {
        self.slave = Some(Arc::new(RwLock::new(slave)))
    }
}
