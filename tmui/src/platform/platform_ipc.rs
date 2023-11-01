use super::{
    shared_channel::{self, SharedChannel},
    PlatformContext,
};
use crate::{application::PLATFORM_CONTEXT, primitive::{bitmap::Bitmap, Message}, runtime::{window_process, window_context::{OutputSender, WindowContext}}};
use std::sync::{
    atomic::Ordering,
    mpsc::{channel, Sender},
    Arc,
};
use tipc::{ipc_slave::IpcSlave, IpcNode, WithIpcSlave};
use tlib::figure::Rect;

pub(crate) struct PlatformIpc<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    title: String,
    region: Rect,

    bitmap: Option<Bitmap>,

    input_sender: Option<Sender<Message>>,

    /// Shared memory ipc slave
    slave: Option<Arc<IpcSlave<T, M>>>,
    user_ipc_event_sender: Option<Sender<Vec<T>>>,

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
            input_sender: None,
            slave: None,
            user_ipc_event_sender: None,
            shared_widget_id: None,
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

    #[inline]
    pub fn set_shared_widget_id(&mut self, id: &'static str) {
        self.shared_widget_id = Some(id)
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformContext
    for PlatformIpc<T, M>
{
    fn initialize(&mut self) {
        let slave = self.slave.as_ref().unwrap();
        let front_bitmap = Bitmap::new(slave.buffer_raw_pointer(), slave.width(), slave.height());

        self.region = slave
            .region(self.shared_widget_id.unwrap())
            .expect("The `SharedWidget` with id `{}` was not exist.");

        self.bitmap = Some(front_bitmap);
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
    fn region(&self) -> Rect {
        self.region
    }

    #[inline]
    fn resize(&mut self, width: u32, height: u32) {
        self.region.set_width(width as i32);
        self.region.set_height(height as i32);
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

    #[inline]
    fn request_redraw(&mut self, _window: &tlib::winit::window::Window) {}

    #[inline]
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

    #[inline]
    fn add_shared_region(&self, _: &'static str, _: Rect) {}
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> WithIpcSlave<T, M>
    for PlatformIpc<T, M>
{
    fn proc_ipc_slave(&mut self, slave: tipc::ipc_slave::IpcSlave<T, M>) {
        self.slave = Some(Arc::new(slave))
    }
}
