#[cfg(target_os = "linux")]
use crate::platform::PlatformLinux;
#[cfg(target_os = "macos")]
use crate::platform::PlatformMacos;
#[cfg(target_os = "windows")]
use crate::platform::PlatformWin32;
use crate::{
    application_window::{store_board, ApplicationWindow},
    backend::{opengl_backend::OpenGLBackend, raster_backend::RasterBackend, Backend, BackendType},
    graphics::board::Board,
    platform::{
        window_context::{OutputSender, WindowContext},
        Message, PlatformContext, PlatformIpc, PlatformType,
    },
};
use lazy_static::lazy_static;
use std::{
    cell::RefCell,
    ptr::null_mut,
    sync::{
        atomic::{AtomicPtr, Ordering},
        mpsc::{channel, Receiver},
        Arc,
    },
    thread,
    time::Duration,
};
use tlib::{
    actions::{ActionHub, ACTIVATE},
    object::ObjectImpl,
    prelude::tokio_runtime,
    timer::TimerHub,
    utils::TimeStamp,
};

lazy_static! {
    pub(crate) static ref PLATFORM_CONTEXT: AtomicPtr<Box<dyn PlatformContext>> =
        AtomicPtr::new(null_mut());
    pub(crate) static ref APPLICATION_WINDOW: AtomicPtr<ApplicationWindow> =
        AtomicPtr::new(null_mut());
}
thread_local! { static IS_UI_MAIN_THREAD: RefCell<bool> = RefCell::new(false) }

pub const FRAME_INTERVAL: u128 = 16000;

#[derive(Default)]
pub struct Application {
    width: u32,
    height: u32,
    title: String,

    platform_type: PlatformType,
    backend_type: BackendType,

    platform_context: RefCell<Option<Box<dyn PlatformContext>>>,

    on_activate: RefCell<Option<Arc<dyn Fn(&mut ApplicationWindow) + Send + Sync>>>,
}

impl Application {
    /// Get the builder [`ApplicationBuilder`] of `Application`.
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder::default()
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

        thread::Builder::new()
            .name("tmui-main".to_string())
            .spawn(move || Self::ui_main(backend_type, output_sender, input_receiver, on_activate))
            .unwrap();

        platform_context.platform_main(window_context);
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
        // Create the [`PlatformContext`] based on the platform type specified by the user.
        let platform_context;
        match self.platform_type {
            PlatformType::Ipc => platform_context = PlatformIpc::new(&title, width, height).wrap(),
            #[cfg(target_os = "windows")]
            PlatformType::Win32 => {
                platform_context = PlatformWin32::new(&title, width, height).wrap()
            }
            #[cfg(target_os = "linux")]
            PlatformType::Linux => {
                platform_context = PlatformLinux::new(&title, width, height).wrap()
            }
            #[cfg(target_os = "macos")]
            PlatformType::Macos => {
                platform_context = PlatformMacos::new(&title, width, height).wrap()
            }
        }
        self.platform_context = RefCell::new(Some(platform_context));
        PLATFORM_CONTEXT.store(
            self.platform_context.borrow_mut().as_mut().unwrap() as *mut Box<dyn PlatformContext>,
            Ordering::SeqCst,
        );
    }

    fn ui_main(
        backend_type: BackendType,
        output_sender: OutputSender,
        _input_receiver: Receiver<Message>,
        on_activate: Option<Arc<dyn Fn(&mut ApplicationWindow) + Send + Sync>>,
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
        APPLICATION_WINDOW.store(window.as_mut() as *mut ApplicationWindow, Ordering::SeqCst);

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

        let mut last_frame = 0u128;
        let mut update;
        loop {
            update = board.invalidate_visual();

            let now = TimeStamp::timestamp_micros();
            if now - last_frame >= FRAME_INTERVAL && update {
                last_frame = now;
                window.send_message(Message::VSync);
            }

            timer_hub.check_timers();
            action_hub.process_multi_thread_actions();
            tlib::r#async::async_callbacks();
            thread::sleep(Duration::from_nanos(1));
        }
    }
}

/// The builder to create the [`Application`]
#[derive(Default)]
pub struct ApplicationBuilder {
    platform: Option<PlatformType>,
    backend: Option<BackendType>,
    title: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
}

impl ApplicationBuilder {
    /// Build the [`Application`]
    pub fn build(self) -> Application {
        let mut app = Application::default();

        if let Some(ref title) = self.title {
            app.title = title.to_string()
        }
        if let Some(width) = self.width {
            app.width = width
        }
        if let Some(height) = self.height {
            app.height = height
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
}
