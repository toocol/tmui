#[cfg(target_os = "linux")]
use crate::platform::PlatformLinux;
#[cfg(target_os = "macos")]
use crate::platform::PlatformMacos;
#[cfg(target_os = "windows")]
use crate::platform::PlatformWin32;
use crate::{
    application_window::ApplicationWindow,
    backend::{opengl_backend::OpenGLBackend, raster_backend::RasterBackend, Backend, BackendType},
    graphics::board::Board,
    platform::{Message, PlatformContext, PlatformContextWrapper, PlatformIpc, PlatformType},
    widget::store_board,
};
use lazy_static::lazy_static;
use std::{
    cell::RefCell,
    ptr::null_mut,
    sync::{
        atomic::{AtomicPtr, Ordering},
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread,
    time::Duration,
};
use tlib::{actions::ActionHub, utils::TimeStamp, Object};

lazy_static! {
    pub static ref PLATFORM_CONTEXT: AtomicPtr<Box<dyn PlatformContextWrapper>> =
        AtomicPtr::new(null_mut());
    pub static ref APPLICATION_WINDOW: AtomicPtr<ApplicationWindow> = AtomicPtr::new(null_mut());
}
thread_local! { static IS_UI_MAIN_THREAD: RefCell<bool> = RefCell::new(false) }

pub const FRAME_INTERVAL: u128 = 16000;

#[derive(Default)]
pub struct Application {
    width: i32,
    height: i32,
    title: String,

    platform_type: PlatformType,
    backend_type: BackendType,

    platform_context: Option<Box<dyn PlatformContextWrapper>>,

    on_activate: RefCell<Option<Arc<dyn Fn(&ApplicationWindow) + Send + Sync>>>,
}

impl Application {
    /// Get the builder [`ApplicationBuilder`] of `Application`.
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder::default()
    }

    /// Get the main application window([`ApplicationWindow`]).
    /// There was only one application window in tmui.
    pub fn application_window<'a>() -> &'a ApplicationWindow {
        if !Self::is_ui_thread() {
            panic!("`Application::application_window()` should only call in the UI `main` thread.");
        }

        unsafe { APPLICATION_WINDOW.load(Ordering::SeqCst).as_mut().unwrap() }
    }

    /// Determine whether the current thread is the UI main thread.
    pub fn is_ui_thread() -> bool {
        IS_UI_MAIN_THREAD.with(|is_main| *is_main.borrow())
    }

    /// Start to run this application.
    pub fn run(&self) {
        let (output_sender, output_receiver) = channel::<Message>();
        let (input_sender, input_receiver) = channel::<Message>();

        let platform_context = self.platform_context.as_ref().unwrap();
        platform_context.set_input_sender(input_sender);

        // Create the `UI` main thread.
        let backend_type = self.backend_type;
        let on_activate = self.on_activate.borrow().clone();
        *self.on_activate.borrow_mut() = None;
        thread::spawn(move || {
            Self::ui_main(backend_type, output_sender, input_receiver, on_activate)
        });

        loop {
            if let Ok(msg) = output_receiver.try_recv() {
                platform_context.send_message(msg);
            }
            platform_context.handle_platform_event();
            thread::sleep(Duration::from_nanos(1));
        }
    }

    /// The method will be activate when the ui thread was created, and activated in the ui thread. <br>
    /// UI components should create in here.
    pub fn connect_activate<F>(&self, f: F)
    where
        F: Fn(&ApplicationWindow) + Send + Sync + 'static,
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
        self.platform_context = Some(platform_context);
        PLATFORM_CONTEXT.store(
            self.platform_context.as_mut().unwrap() as *mut Box<dyn PlatformContextWrapper>,
            Ordering::SeqCst,
        );
    }

    fn ui_main(
        backend_type: BackendType,
        output_sender: Sender<Message>,
        _input_receiver: Receiver<Message>,
        on_activate: Option<Arc<dyn Fn(&ApplicationWindow) + Send + Sync>>,
    ) {
        // Set the UI thread to the `Main` thread.
        IS_UI_MAIN_THREAD.with(|is_main| *is_main.borrow_mut() = true);

        let platform = unsafe { PLATFORM_CONTEXT.load(Ordering::SeqCst).as_ref().unwrap() };
        // Create and initialize the `ActionHub`.
        let mut action_hub = ActionHub::new();
        action_hub.initialize();

        // Create the [`Backend`] based on the backend type specified by the user.
        let backend;
        match backend_type {
            BackendType::Raster => {
                backend = RasterBackend::new(platform.front_bitmap(), platform.back_bitmap()).wrap()
            }
            BackendType::OpenGL => {
                backend = OpenGLBackend::new(platform.front_bitmap(), platform.back_bitmap()).wrap()
            }
        }

        // Create the `Board`.
        let mut board = Board::new(backend.surface());
        store_board(&mut board);

        let mut window: ApplicationWindow = Object::new(&[]);
        APPLICATION_WINDOW.store(&mut window as *mut ApplicationWindow, Ordering::SeqCst);

        if let Some(on_activate) = on_activate {
            on_activate(&window);
            drop(on_activate);
        }
        board.add_element(&mut window);

        let mut last_frame = 0u128;
        let mut update;
        loop {
            update = board.invalidate_visual();

            let now = TimeStamp::timestamp_micros();
            if now - last_frame >= FRAME_INTERVAL && update {
                last_frame = now;
                output_sender.send(Message::MESSAGE_VSNYC).unwrap();
            }

            action_hub.process_multi_thread_actions();
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
    width: Option<i32>,
    height: Option<i32>,
}

impl ApplicationBuilder {
    /// Build the [`Application`]
    pub fn build(self) -> Application {
        let mut app = Application::default();

        if let Some(ref title) = self.title {
            app.title = title.to_string()
        }
        if let Some(ref width) = self.width {
            app.width = *width
        }
        if let Some(ref height) = self.height {
            app.height = *height
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

    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: i32) -> Self {
        self.height = Some(height);
        self
    }
}
