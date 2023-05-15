#[cfg(target_os = "linux")]
use crate::platform::PlatformLinux;
#[cfg(target_os = "macos")]
use crate::platform::PlatformMacos;
#[cfg(target_os = "windows")]
use crate::platform::PlatformWin32;
use crate::{
    application_window::{store_board, ApplicationWindow},
    backend::{opengl_backend::OpenGLBackend, raster_backend::RasterBackend, Backend, BackendType},
    graphics::{board::Board, cpu_balance::CpuBalance},
    platform::{
        shared_channel::SharedChannel,
        window_context::{OutputSender, WindowContext},
        Message, PlatformContext, PlatformIpc, PlatformType,
    },
};
use lazy_static::lazy_static;
use log::debug;
use std::{
    cell::RefCell,
    marker::PhantomData,
    ptr::null_mut,
    sync::{
        atomic::{AtomicBool, AtomicPtr, Ordering},
        mpsc::{channel, Receiver},
        Arc,
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
thread_local! { static IS_UI_MAIN_THREAD: RefCell<bool> = RefCell::new(false) }

pub const FRAME_INTERVAL: u128 = 16000;
static APP_STOPPED: AtomicBool = AtomicBool::new(false);

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

    on_activate: RefCell<Option<Arc<dyn Fn(&mut ApplicationWindow) + Send + Sync>>>,

    shared_mem_name: Option<&'static str>,
    shared_channel: RefCell<Option<SharedChannel<T, M>>>,
}

impl Application<(), ()> {
    /// Get the default builder [`ApplicationBuilder`] of `Application`.
    pub fn builder() -> ApplicationBuilder<(), ()> {
        ApplicationBuilder::<(), ()>::new(None)
    }
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> Application<T, M> {
    /// Get the shared memory application builder [`ApplicationBuilder`], enable ipc function for application. <br>
    /// T: Generic type for [`IpcEvent::UserEvent`](tipc::ipc_event::IpcEvent::UserEvent). <br>
    /// M: Generic type for blocked request with response in ipc communication.
    #[inline]
    pub fn shared_builder(shared_mem_name: &'static str) -> ApplicationBuilder<T, M> {
        ApplicationBuilder::<T, M>::new(Some(shared_mem_name))
    }

    /// Determine whether the current thread is the UI main thread.
    #[inline]
    pub fn is_ui_thread() -> bool {
        IS_UI_MAIN_THREAD.with(|is_main| *is_main.borrow())
    }

    /// Start to run this application.
    pub fn run(&self) {
        let (input_sender, input_receiver) = channel::<Message>();

        let mut platform_mut = self.platform_context.borrow_mut();
        let platform_context = platform_mut.as_mut().unwrap();
        platform_context.set_input_sender(input_sender);

        // Create the `UI` main thread.
        let backend_type = self.backend_type;
        let on_activate = self.on_activate.borrow().clone();
        *self.on_activate.borrow_mut() = None;

        let mut window_context = platform_context.create_window();
        let output_sender = match window_context {
            WindowContext::Ipc(.., ref mut output) => output.take().unwrap(),
            WindowContext::Default(.., ref mut output) => output.take().unwrap(),
        };

        let shared_channel = self.shared_channel.borrow_mut().take();

        let join = thread::Builder::new()
            .name("tmui-main".to_string())
            .spawn(move || {
                Self::ui_main(
                    backend_type,
                    output_sender,
                    input_receiver,
                    on_activate,
                    shared_channel,
                )
            })
            .unwrap();

        platform_context.platform_main(window_context);
        APP_STOPPED.store(true, Ordering::SeqCst);
        join.join().unwrap();
    }

    /// The method will be activate when the ui thread was created, and activated in the ui thread. <br>
    /// UI components should create in here.
    pub fn connect_activate<F>(&self, f: F)
    where
        F: Fn(&mut ApplicationWindow) + Send + Sync + 'static,
    {
        *self.on_activate.borrow_mut() = Some(Arc::new(f));
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
        backend_type: BackendType,
        output_sender: OutputSender,
        _input_receiver: Receiver<Message>,
        on_activate: Option<Arc<dyn Fn(&mut ApplicationWindow) + Send + Sync>>,
        _shared_channel: Option<SharedChannel<T, M>>,
    ) {
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
            BackendType::Raster => {
                backend = RasterBackend::new(platform.front_bitmap(), platform.back_bitmap())
            }
            BackendType::OpenGL => {
                backend = OpenGLBackend::new(platform.front_bitmap(), platform.back_bitmap())
            }
        }

        // Prepare ApplicationWindow env: Create the `Board`, windows layouts
        let mut board = Board::new(backend.surface());
        store_board(&mut board);

        let mut window = ApplicationWindow::new(backend.width() as i32, backend.height() as i32);

        if let Some(on_activate) = on_activate {
            on_activate(&mut window);
            drop(on_activate);
        }
        ACTIVATE.store(true, Ordering::SeqCst);

        board.add_element(window.as_mut());
        window.register_window(output_sender);
        window.initialize();
        window.window_layout_change();
        window.activate();

        let mut cpu_balance = CpuBalance::new();
        let mut last_frame = Instant::now();
        let mut update = true;
        let mut frame_cnt = 0;
        let (mut time_17, mut time_17_20, mut time_20_25, mut time_25) = (0, 0, 0, 0);
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
                debug!(
                    "frame time distribution rate: [<17ms: {}%, 17-20ms: {}%, 20-25ms: {}%, >=25ms: {}%], frame time: {}ms",
                    time_17 as f32 / frame_cnt as f32 * 100., time_17_20 as f32 / frame_cnt as f32 * 100., time_20_25 as f32 / frame_cnt as f32 * 100., time_25 as f32 / frame_cnt as f32 * 100., frame_time
                );
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
            cpu_balance.payload_check();
            cpu_balance.sleep(update);
        }
    }
}

/// The builder to create the [`Application`] <br>
pub struct ApplicationBuilder<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    platform: Option<PlatformType>,
    backend: Option<BackendType>,
    title: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    shared_mem_name: Option<&'static str>,
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
            shared_channel: Default::default(),
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

        if let Some(platform) = self.platform {
            app.platform_type = platform
        }
        if let Some(backend) = self.backend {
            app.backend_type = backend
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
