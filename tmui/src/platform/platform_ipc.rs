use super::{
    shared_channel::{self, SharedChannel},
    window_context::{OutputSender, WindowContext},
    window_process, Message, PlatformContext,
};
use crate::{application::PLATFORM_CONTEXT, graphics::bitmap::Bitmap};
use std::sync::{
    atomic::Ordering,
    mpsc::{channel, Sender},
    Arc,
};
use tipc::{ipc_slave::IpcSlave, IpcNode, WithIpcSlave};
use tlib::{figure::Rect, prelude::*, reflect_trait};

pub(crate) struct PlatformIpc<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    title: String,
    width: u32,
    height: u32,

    bitmap: Option<Bitmap>,

    input_sender: Option<Sender<Message>>,

    /// Shared memory ipc slave
    slave: Option<Arc<IpcSlave<T, M>>>,
    user_ipc_event_sender: Option<Sender<Vec<T>>>,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformIpc<T, M> {
    #[inline]
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            bitmap: None,
            input_sender: None,
            slave: None,
            user_ipc_event_sender: None,
        }
    }

    // Wrap trait `PlatfomContext` with [`Box`].
    #[inline]
    pub fn wrap(self) -> Box<dyn PlatformContext> {
        Box::new(self)
    }

    #[inline]
    pub fn shared_channel(&mut self) -> SharedChannel<T, M> {
        let (sender, receiver) = channel();
        self.user_ipc_event_sender = Some(sender);
        shared_channel::slave_channel(self.slave.as_ref().unwrap().clone(), receiver)
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformContext
    for PlatformIpc<T, M>
{
    fn initialize(&mut self) {
        let slave = self.slave.as_ref().unwrap();
        let region = slave.region();
        let front_bitmap = Bitmap::new(
            slave.buffer_raw_pointer(),
            region.width() as u32,
            region.height() as u32,
        );

        self.bitmap = Some(front_bitmap);
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
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        todo!()
    }

    #[inline]
    fn bitmap(&self) -> Bitmap {
        self.bitmap.unwrap()
    }

    #[inline]
    fn set_input_sender(&mut self, input_sender: Sender<Message>) {
        self.input_sender = Some(input_sender);
    }

    #[inline]
    fn input_sender(&self) -> &Sender<Message> {
        self.input_sender.as_ref().unwrap()
    }

    #[inline]
    fn create_window(&mut self) -> WindowContext {
        let (output_sender, output_receiver) = channel::<Message>();
        WindowContext::Ipc(output_receiver, Some(OutputSender::Sender(output_sender)))
    }

    fn platform_main(&mut self, window_context: WindowContext) {
        let platform = unsafe {
            PLATFORM_CONTEXT
                .load(Ordering::SeqCst)
                .as_mut()
                .expect("`PLATFORM_WIN32` is None.")
        };

        if let WindowContext::Ipc(output_receiver, _) = window_context {
            window_process::WindowProcess::new().event_handle_ipc::<T, M>(
                platform.as_mut(),
                output_receiver,
                self.slave.as_ref().unwrap().clone(),
                self.user_ipc_event_sender.take(),
            )
        } else {
            panic!("Invalid window context.")
        }
    }

    fn redraw(&mut self) {}

    #[inline]
    fn wait(&self) {
        if let Some(ref slave) = self.slave {
            slave.wait()
        }
    }

    #[inline]
    fn signal(&self) {
        if let Some(ref slave) = self.slave {
            slave.signal()
        }
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> WithIpcSlave<T, M>
    for PlatformIpc<T, M>
{
    fn proc_ipc_slave(&mut self, slave: tipc::ipc_slave::IpcSlave<T, M>) {
        self.slave = Some(Arc::new(slave))
    }
}

#[reflect_trait]
pub trait IpcContext {
    fn add_region(&mut self, rect: Rect);
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> AsAny for PlatformIpc<T, M> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any_boxed(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> Reflect
    for PlatformIpc<T, M>
{
    fn as_reflect(&self) -> &dyn Reflect {
        self
    }

    fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
        self
    }

    fn as_reflect_boxed(self: Box<Self>) -> Box<dyn Reflect> {
        self
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> IpcContext
    for PlatformIpc<T, M>
{
    fn add_region(&mut self, rect: Rect) {
        todo!()
    }
}
