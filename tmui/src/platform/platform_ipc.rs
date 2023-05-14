use super::{
    window_context::{OutputSender, WindowContext},
    window_process, Message, PlatformContext,
};
use crate::{application::PLATFORM_CONTEXT, graphics::bitmap::Bitmap};
use std::sync::{
    atomic::Ordering,
    mpsc::{channel, Sender, Receiver}, Arc,
};
use tipc::{ipc_slave::IpcSlave, IpcNode, WithIpcSlave};

pub(crate) struct PlatformIpc<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    title: String,
    width: u32,
    height: u32,

    front_bitmap: Option<Bitmap>,
    back_bitmap: Option<Bitmap>,

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
            front_bitmap: None,
            back_bitmap: None,
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
    pub fn gen_user_ipc_event_channel(&mut self) -> Receiver<Vec<T>> {
        let (sender, receiver) = channel();
        self.user_ipc_event_sender = Some(sender);
        receiver
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> PlatformContext for PlatformIpc<T, M> {
    fn initialize(&mut self) {
        let slave = self.slave.as_ref().unwrap();
        let front_bitmap = Bitmap::new(slave.primary_buffer_raw_pointer(), self.width, self.height);

        let back_bitmap = Bitmap::new(
            slave.secondary_buffer_raw_pointer(),
            self.width,
            self.height,
        );

        self.front_bitmap = Some(front_bitmap);
        self.back_bitmap = Some(back_bitmap);
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        todo!()
    }

    fn front_bitmap(&self) -> Bitmap {
        self.front_bitmap.unwrap()
    }

    fn back_bitmap(&self) -> Bitmap {
        self.back_bitmap.unwrap()
    }

    fn set_input_sender(&mut self, input_sender: Sender<Message>) {
        self.input_sender = Some(input_sender);
    }

    fn input_sender(&self) -> &Sender<Message> {
        self.input_sender.as_ref().unwrap()
    }

    fn create_window(&mut self) -> WindowContext {
        let (output_sender, output_receiver) = channel::<Message>();
        WindowContext::Ipc(output_receiver, Some(OutputSender::Sender(output_sender)))
    }

    fn platform_main(&self, window_context: WindowContext) {
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
            )
        } else {
            panic!("Invalid window context.")
        }
    }

    fn redraw(&mut self) {}
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> WithIpcSlave<T, M> for PlatformIpc<T, M> {
    fn proc_ipc_slave(&mut self, slave: tipc::ipc_slave::IpcSlave<T, M>) {
        self.slave = Some(Arc::new(slave))
    }
}
