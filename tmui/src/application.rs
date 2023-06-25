#[cfg(target_os = "linux")]
use crate::platform::PlatformLinux;
#[cfg(target_os = "macos")]
use crate::platform::PlatformMacos;
#[cfg(target_os = "windows")]
use crate::platform::PlatformWin32;
use crate::{
    application_window::ApplicationWindow,
    backend::{opengl_backend::OpenGLBackend, raster_backend::RasterBackend, Backend, BackendType},
    event_hints::event_hints,
    graphics::{board::Board, cpu_balance::CpuBalance},
    platform::{
        shared_channel::SharedChannel,
        window_context::{OutputSender, WindowContext},
        Message, PlatformContext, PlatformIpc, PlatformType,
    },
    widget::WidgetImpl,
};
use lazy_static::lazy_static;
use log::debug;
use std::{
    any::Any,
    cell::RefCell,
    marker::PhantomData,
    ptr::null_mut,
    sync::{
        atomic::{AtomicBool, AtomicPtr, Ordering},
        mpsc::{channel, Receiver},
        Once,
    },
    thread,
    time::Instant,
};
use tipc::{WithIpcMaster, WithIpcSlave};
use tlib::{
    actions::{ActionHub, ACTIVATE},
    object::ObjectImpl,
    prelude::tokio_runtime,
    timer::TimerHub,
};

lazy_static! {
    pub(crate) static ref PLATFORM_CONTEXT: AtomicPtr<Box<dyn PlatformContext>> =
        AtomicPtr::new(null_mut());
}
thread_local! {
    static IS_UI_MAIN_THREAD: RefCell<bool> = RefCell::new(false);
    static SHARED_CHANNEL: RefCell<Option<Box<dyn Any>>> = RefCell::new(None);
}

pub const FRAME_INTERVAL: u128 = 16000;

const INVALID_GENERIC_PARAM_ERROR: &'static str =
    "Invalid generic parameters, please use generic parameter defined on Application.";
static APP_STOPPED: AtomicBool = AtomicBool::new(false);
static IS_SHARED: AtomicBool = AtomicBool::new(false);
static ONCE: Once = Once::new();

/// ### The main application of tmui. <br>
///
/// For common single process gui application, use [`Application::builder`] <br>
/// Multile process gui application, use [`Application::shared_builder`] instead to define ipc shared memory UserEvent and Request type. <br>
///
/// ### These generic types were used for ipc shared memory application. <br>
/// `T`: Generic type for [`IpcEvent::UserEvent`](tipc::ipc_event::IpcEvent::UserEvent). <br>
/// `M`: Generic type for blocked request with response in ipc communication.
pub struct Application<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    width: u32,
    height: u32,
    title: String,

    platform_type: PlatformType,
    backend_type: BackendType,

    platform_context: RefCell<Option<Box<dyn PlatformContext>>>,

    on_activate: RefCell<Option<Box<dyn Fn(&mut ApplicationWindow) + Send + Sync>>>,
    on_user_event_receive: RefCell<Option<Box<dyn Fn(&mut ApplicationWindow, T) + Send + Sync>>>,
    on_request_receive:
        RefCell<Option<Box<dyn Fn(&mut ApplicationWindow, M) -> Option<M> + Send + Sync>>>,

    shared_mem_name: Option<&'static str>,
    shared_widget_id: Option<&'static str>,
    shared_channel: RefCell<Option<SharedChannel<T, M>>>,
}

impl Application<(), ()> {
    /// Get the default builder [`ApplicationBuilder`] of `Application`.
    pub fn builder() -> ApplicationBuilder<(), ()> {
        ONCE.call_once(|| {});
        ApplicationBuilder::<(), ()>::new(None)
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> Application<T, M> {
    /// Get the shared memory application builder [`ApplicationBuilder`], enable ipc function for application. <br>
    /// T: Generic type for [`IpcEvent::UserEvent`](tipc::ipc_event::IpcEvent::UserEvent). <br>
    /// M: Generic type for blocked request with response in ipc communication.
    #[inline]
    pub fn shared_builder(shared_mem_name: &'static str) -> ApplicationBuilder<T, M> {
        ONCE.call_once(|| {});
        IS_SHARED.store(true, Ordering::SeqCst);
        ApplicationBuilder::<T, M>::new(Some(shared_mem_name))
    }

    /// Start to run this application.
    pub fn run(&self) {
        let (input_sender, input_receiver) = channel::<Message>();

        let mut platform_mut = self.platform_context.borrow_mut();
        let platform_context = platform_mut.as_mut().unwrap();
        platform_context.set_input_sender(input_sender);

        let backend_type = self.backend_type;
        let on_activate = self.on_activate.borrow_mut().take();
        *self.on_activate.borrow_mut() = None;

        let mut window_context = platform_context.create_window();
        let output_sender = match window_context {
            WindowContext::Ipc(.., ref mut output) => output.take().unwrap(),
            WindowContext::Default(.., ref mut output) => output.take().unwrap(),
        };

        let shared_channel = self.shared_channel.borrow_mut().take();

        // Ipc shared user events and request process function.
        let on_user_event_receive = self.on_user_event_receive.borrow_mut().take();
        let on_request_receive = self.on_request_receive.borrow_mut().take();

        let platform_type = self.platform_type;

        // Create the `UI` main thread.
        let join = thread::Builder::new()
            .name("tmui-main".to_string())
            .spawn(move || {
                Self::ui_main(
                    platform_type,
                    backend_type,
                    output_sender,
                    input_receiver,
                    shared_channel,
                    on_activate,
                    on_user_event_receive,
                    on_request_receive,
                )
            })
            .unwrap();

        platform_context.platform_main(window_context);
        APP_STOPPED.store(true, Ordering::SeqCst);
        join.join().unwrap();

        PLATFORM_CONTEXT.store(null_mut(), Ordering::SeqCst);
        if self.platform_type == PlatformType::Ipc {
            platform_context.signal();
        }
    }

    /// The method will be activate when the ui thread was created, and activated in the ui thread. <br>
    /// UI components should create in here.
    pub fn connect_activate<F>(&self, f: F)
    where
        F: Fn(&mut ApplicationWindow) + Send + Sync + 'static,
    {
        *self.on_activate.borrow_mut() = Some(Box::new(f));
    }

    /// The method will be invoked when ipc user events received.
    pub fn connect_user_events_receive<F>(&self, f: F)
    where
        F: Fn(&mut ApplicationWindow, T) + Send + Sync + 'static,
    {
        *self.on_user_event_receive.borrow_mut() = Some(Box::new(f));
    }

    /// The method will be invoked when ipc request received.
    pub fn connect_request_receive<F>(&self, f: F)
    where
        F: Fn(&mut ApplicationWindow, M) -> Option<M> + Send + Sync + 'static,
    {
        *self.on_request_receive.borrow_mut() = Some(Box::new(f));
    }

    fn startup_initialize(&mut self) {
        let width = self.width;
        let height = self.height;
        let title = &self.title;
        let mut shared_channel = None;
        // Create the [`PlatformContext`] based on the platform type specified by the user.
        let platform_context = match self.platform_type {
            PlatformType::Ipc => {
                let mut platform_context = PlatformIpc::<T, M>::new(&title, width, height);
                let shared_mem_name = self.shared_mem_name.expect(
                    "`PlatformType::Ipc` need build by function `Application::shared_builder()`",
                );
                platform_context.with_ipc_slave(shared_mem_name);
                platform_context.initialize();
                shared_channel = Some(platform_context.shared_channel());

                // Ipc slave app's size was determined by the main program:
                self.width = platform_context.width();
                self.height = platform_context.height();

                platform_context.wrap()
            }
            #[cfg(target_os = "windows")]
            PlatformType::Win32 => {
                let mut platform_context = PlatformWin32::<T, M>::new(&title, width, height);
                if let Some(shared_mem_name) = self.shared_mem_name {
                    platform_context.with_ipc_master(shared_mem_name, self.width, self.height);
                    shared_channel = Some(platform_context.shared_channel());
                }
                platform_context.initialize();
                platform_context.wrap()
            }
            #[cfg(target_os = "linux")]
            PlatformType::Linux => {
                let mut platform_context = PlatformMacos::<T, M>::new(&title, width, height);
                if let Some(shared_mem_name) = self.shared_mem_name {
                    platform_context.with_ipc_master(shared_mem_name, width, height);
                    shared_channel = Some(platform_context.shared_channel());
                }
                platform_context.initialize();
                platform_context.wrap()
            }
            #[cfg(target_os = "macos")]
            PlatformType::Macos => {
                let mut platform_context = PlatformMacos::<T, M>::new(&title, width, height);
                if let Some(shared_mem_name) = self.shared_mem_name {
                    platform_context.with_ipc_master(shared_mem_name, width, height);
                    shared_channel = Some(platform_context.shared_channel());
                }
                platform_context.initialize();
                platform_context.wrap()
            }
        };
        self.platform_context = RefCell::new(Some(platform_context));
        self.shared_channel = RefCell::new(shared_channel);
        let ptr =
            self.platform_context.borrow_mut().as_mut().unwrap() as *mut Box<dyn PlatformContext>;
        PLATFORM_CONTEXT.store(ptr, Ordering::SeqCst);
    }

    fn ui_main(
        platform_type: PlatformType,
        backend_type: BackendType,
        output_sender: OutputSender,
        input_receiver: Receiver<Message>,
        shared_channel: Option<SharedChannel<T, M>>,
        on_activate: Option<Box<dyn Fn(&mut ApplicationWindow) + Send + Sync>>,
        on_user_event_receive: Option<Box<dyn Fn(&mut ApplicationWindow, T) + Send + Sync>>,
        on_request_receive: Option<
            Box<dyn Fn(&mut ApplicationWindow, M) -> Option<M> + Send + Sync>,
        >,
    ) {
        // Set up the ipc shared channel.
        if let Some(shared_channel) = shared_channel {
            SHARED_CHANNEL.with(|s| *s.borrow_mut() = Some(Box::new(shared_channel)));
        }

        // Set the UI thread to the `Main` thread.
        IS_UI_MAIN_THREAD.with(|is_main| *is_main.borrow_mut() = true);

        // Setup the async runtime
        let _guard = tokio_runtime().enter();

        let platform = unsafe { PLATFORM_CONTEXT.load(Ordering::SeqCst).as_ref().unwrap() };

        // Create and initialize the `ActionHub`.
        let mut action_hub = ActionHub::new();
        action_hub.initialize();

        // Create and initialize the `TimerHub`.
        let mut timer_hub = TimerHub::new();
        timer_hub.initialize();

        // Create the [`Backend`] based on the backend type specified by the user.
        let backend: Box<dyn Backend>;
        match backend_type {
            BackendType::Raster => backend = RasterBackend::new(platform.bitmap()),
            BackendType::OpenGL => backend = OpenGLBackend::new(platform.bitmap()),
        }

        // Prepare ApplicationWindow env: Create the `Board`.
        let mut board = Box::new(Board::new(backend.surface()));

        let mut window = ApplicationWindow::new(
            platform_type,
            backend.width() as i32,
            backend.height() as i32,
        );
        window.set_board(board.as_mut());

        if let Some(on_activate) = on_activate {
            on_activate(&mut window);
            drop(on_activate);
        }
        ACTIVATE.store(true, Ordering::SeqCst);

        board.add_element(window.as_mut());
        window.register_window(output_sender);
        window.initialize();
        window.activate();
        window.run_after();

        let mut cpu_balance = CpuBalance::new();
        let mut last_frame = Instant::now();
        let mut update = true;
        let mut frame_cnt = 0;
        let (mut time_17, mut time_17_20, mut time_20_25, mut time_25) = (0, 0, 0, 0);
        let mut log_instant = Instant::now();
        loop {
            if APP_STOPPED.load(Ordering::Relaxed) {
                break;
            }
            cpu_balance.loop_start();
            let elapsed = last_frame.elapsed();

            update = if elapsed.as_micros() >= FRAME_INTERVAL {
                last_frame = Instant::now();
                let frame_time = elapsed.as_micros() as f32 / 1000.;
                frame_cnt += 1;
                match frame_time as i32 {
                    0..=16 => time_17 += 1,
                    17..=19 => time_17_20 += 1,
                    20..=24 => time_20_25 += 1,
                    _ => time_25 += 1,
                }
                if log_instant.elapsed().as_secs() >= 1 {
                    debug!(
                    "frame time distribution rate: [<17ms: {}%, 17-20ms: {}%, 20-25ms: {}%, >=25ms: {}%], frame time: {}ms",
                    time_17 as f32 / frame_cnt as f32 * 100., time_17_20 as f32 / frame_cnt as f32 * 100., time_20_25 as f32 / frame_cnt as f32 * 100., time_25 as f32 / frame_cnt as f32 * 100., frame_time
                    );
                    log_instant = Instant::now();
                }
                let update = board.invalidate_visual();
                if update {
                    window.send_message(Message::VSync(Instant::now()));
                    cpu_balance.add_payload();
                }
                update
            } else {
                update
            };

            timer_hub.check_timers();
            action_hub.process_multi_thread_actions();
            tlib::r#async::async_callbacks();
            if let Ok(Message::Event(evt)) = input_receiver.try_recv() {
                window.dispatch_event(evt);
                cpu_balance.add_payload();
            }

            if let Some(ref on_user_event_receive) = on_user_event_receive {
                Self::process_user_events(&mut window, &mut cpu_balance, on_user_event_receive);
            }
            if let Some(ref on_rqst_receive) = on_request_receive {
                Self::process_request(&mut window, &mut cpu_balance, on_rqst_receive);
            } else {
                Self::process_request_ignored()
            }

            cpu_balance.payload_check();
            cpu_balance.sleep(update);
        }
    }

    #[inline]
    pub fn send_user_event(evt: T) {
        SHARED_CHANNEL.with(|s| {
            if let Some(channel) = s.borrow().as_ref() {
                let sender = &channel
                    .downcast_ref::<SharedChannel<T, M>>()
                    .expect(INVALID_GENERIC_PARAM_ERROR)
                    .0;
                sender.send_user_event(evt)
            }
        });
    }

    #[inline]
    pub fn send_request(rqst: M) -> Option<M> {
        SHARED_CHANNEL.with(|s| {
            if let Some(channel) = s.borrow().as_ref() {
                let sender = &channel
                    .downcast_ref::<SharedChannel<T, M>>()
                    .expect(INVALID_GENERIC_PARAM_ERROR)
                    .0;
                sender.send_request(rqst)
            } else {
                None
            }
        })
    }

    #[inline]
    pub fn resp_request(resp: Option<M>) {
        SHARED_CHANNEL.with(|s| {
            if let Some(channel) = s.borrow().as_ref() {
                let sender = &channel
                    .downcast_ref::<SharedChannel<T, M>>()
                    .expect(INVALID_GENERIC_PARAM_ERROR)
                    .0;
                sender.resp_request(resp)
            }
        })
    }

    #[inline]
    fn process_user_events(
        window: &mut ApplicationWindow,
        cpu_balance: &mut CpuBalance,
        on_user_event_receive: &Box<dyn Fn(&mut ApplicationWindow, T) + Sync + Send>,
    ) {
        SHARED_CHANNEL.with(|s| {
            if let Some(channel) = s.borrow().as_ref() {
                let receiver = &channel
                    .downcast_ref::<SharedChannel<T, M>>()
                    .expect(INVALID_GENERIC_PARAM_ERROR)
                    .1;
                for evt in receiver.receive_user_event_vec() {
                    cpu_balance.add_payload();
                    on_user_event_receive(window, evt);
                }
            }
        });
    }

    #[inline]
    fn process_request(
        window: &mut ApplicationWindow,
        cpu_balance: &mut CpuBalance,
        on_request_receive: &Box<dyn Fn(&mut ApplicationWindow, M) -> Option<M> + Sync + Send>,
    ) {
        SHARED_CHANNEL.with(|s| {
            if let Some(channel) = s.borrow().as_ref() {
                let (sender, receiver) = &channel
                    .downcast_ref::<SharedChannel<T, M>>()
                    .expect(INVALID_GENERIC_PARAM_ERROR);
                if let Some(rqst) = receiver.receive_request() {
                    cpu_balance.add_payload();
                    sender.resp_request(on_request_receive(window, rqst));
                }
            }
        });
    }

    #[inline]
    fn process_request_ignored() {
        SHARED_CHANNEL.with(|s| {
            if let Some(channel) = s.borrow().as_ref() {
                let (sender, receiver) = &channel
                    .downcast_ref::<SharedChannel<T, M>>()
                    .expect(INVALID_GENERIC_PARAM_ERROR);
                if let Some(_) = receiver.receive_request() {
                    sender.resp_request(None);
                }
            }
        });
    }
}

/// Determine whether the current thread is the UI main thread.
#[inline]
pub fn is_ui_thread() -> bool {
    IS_UI_MAIN_THREAD.with(|is_main| *is_main.borrow())
}

/// Is shared memory application or not.
#[inline]
pub(crate) fn is_shared() -> bool {
    IS_SHARED.load(Ordering::SeqCst)
}

/// The builder to create the [`Application`] <br>
pub struct ApplicationBuilder<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    platform: Option<PlatformType>,
    backend: Option<BackendType>,
    title: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    shared_mem_name: Option<&'static str>,
    shared_widget_id: Option<&'static str>,
    _user_event: PhantomData<T>,
    _request: PhantomData<M>,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> ApplicationBuilder<T, M> {
    // Private constructor.
    fn new(shared_mem_name: Option<&'static str>) -> Self {
        Self {
            platform: None,
            backend: None,
            title: None,
            width: None,
            height: None,
            shared_mem_name,
            shared_widget_id: None,
            _user_event: PhantomData::default(),
            _request: PhantomData::default(),
        }
    }

    /// Build the [`Application`]
    pub fn build(self) -> Application<T, M> {
        let mut app = Application {
            width: Default::default(),
            height: Default::default(),
            title: Default::default(),
            platform_type: Default::default(),
            backend_type: Default::default(),
            platform_context: Default::default(),
            on_activate: Default::default(),
            shared_mem_name: Default::default(),
            shared_widget_id: Default::default(),
            shared_channel: Default::default(),
            on_user_event_receive: Default::default(),
            on_request_receive: Default::default(),
        };

        if let Some(ref title) = self.title {
            app.title = title.to_string()
        }
        if let Some(width) = self.width {
            app.width = width
        }
        if let Some(height) = self.height {
            app.height = height
        }
        if let Some(shared_mem_name) = self.shared_mem_name {
            app.shared_mem_name = Some(shared_mem_name)
        }
        if let Some(shared_widget_id) = self.shared_widget_id {
            app.shared_widget_id = Some(shared_widget_id)
        }

        if let Some(platform) = self.platform {
            app.platform_type = platform
        }
        if let Some(backend) = self.backend {
            app.backend_type = backend
        }

        if app.platform_type == PlatformType::Ipc && app.shared_widget_id.is_none() {
            panic!("Shared application with `PlatformIpc` should specified the `shared_widget_id`.")
        }

        app.startup_initialize();
        app
    }

    pub fn platform(mut self, platform: PlatformType) -> Self {
        self.platform = Some(platform);
        self
    }

    pub fn backend(mut self, backend: BackendType) -> Self {
        self.backend = Some(backend);
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn shared_widget_id(mut self, id: &'static str) -> Self {
        self.shared_widget_id = Some(id);
        self
    }

    /// Set the cpu payload threshold (`payloads/per sec`).<br>
    /// When the program payload exceeds the threshold,
    /// the program will increase CPU usage to generate frame data more accurately in time. <br>
    ///
    /// `payload`: Including component rendering, user input, and ipc events, all count as a payload. <br>
    /// The default threshold was `40`.
    pub fn cpu_payload_threshold(self, threshold: usize) -> Self {
        CpuBalance::set_payload_threshold(threshold);
        self
    }
}

///////////////////////////////////////////////////////////////////////
// Events hints
///////////////////////////////////////////////////////////////////////
#[inline]
pub fn double_click_interval() -> i32 {
    event_hints().double_click_interval()
}
#[inline]
pub fn set_double_click_interval(interval: i32) {
    event_hints().set_double_click_interval(interval)
}

#[inline]
pub fn wheel_scroll_lines() -> i32 {
    event_hints().wheel_scroll_lines()
}
#[inline]
pub fn set_wheel_scroll_lines(scroll_lines: i32) {
    event_hints().set_wheel_scroll_lines(scroll_lines)
}
