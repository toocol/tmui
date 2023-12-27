#[cfg(macos_platform)]
use crate::platform::PlatformMacos;
#[cfg(wayland_platform)]
use crate::platform::PlatformWayland;
#[cfg(windows_platform)]
use crate::platform::PlatformWin32;
#[cfg(x11_platform)]
use crate::platform::PlatformX11;
use crate::{
    application_window::ApplicationWindow,
    backend::BackendType,
    event_hints::event_hints,
    platform::{PlatformContext, PlatformIpc, PlatformType},
    primitive::{cpu_balance::CpuBalance, shared_channel::SharedChannel},
    runtime::{ui_runtime, windows_process::WindowsProcess},
};
use std::{
    any::Any,
    cell::RefCell,
    marker::PhantomData,
    sync::{
        atomic::{AtomicBool, Ordering},
        Once,
    },
    thread,
};
use tipc::{WithIpcMaster, WithIpcSlave};
use tlib::events::Event;

thread_local! {
    pub(crate) static IS_UI_MAIN_THREAD: RefCell<bool> = RefCell::new(false);
    pub(crate) static SHARED_CHANNEL: RefCell<Option<Box<dyn Any>>> = RefCell::new(None);
}

const INVALID_GENERIC_PARAM_ERROR: &'static str =
    "Invalid generic parameters, please use generic parameter defined on Application.";
pub(crate) static APP_STARTED: AtomicBool = AtomicBool::new(false);
pub(crate) static APP_STOPPED: AtomicBool = AtomicBool::new(false);
pub(crate) static IS_SHARED: AtomicBool = AtomicBool::new(false);
pub(crate) static HIGH_LOAD: AtomicBool = AtomicBool::new(false);
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
    ui_stack_size: usize,

    platform_type: PlatformType,
    backend_type: BackendType,

    platform_context: RefCell<Option<Box<dyn PlatformContext<T, M>>>>,

    on_activate: RefCell<Option<Box<dyn Fn(&mut ApplicationWindow) + Send + Sync>>>,
    on_user_event_receive: RefCell<Option<Box<dyn Fn(&mut ApplicationWindow, T) + Send + Sync>>>,
    on_request_receive:
        RefCell<Option<Box<dyn Fn(&mut ApplicationWindow, M) -> Option<M> + Send + Sync>>>,

    shared_mem_name: Option<&'static str>,
    shared_widget_id: Option<&'static str>,
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
        let mut platform_mut = self.platform_context.borrow_mut();
        let platform_context = platform_mut.as_mut().unwrap();

        // Create the window
        let (mut logic_window, physical_window) = platform_context.create_window();

        // Get the customize event handle functions.
        let on_activate = self.on_activate.borrow_mut().take();
        let on_user_event_receive = self.on_user_event_receive.borrow_mut().take();
        let on_request_receive = self.on_request_receive.borrow_mut().take();

        // Set the fields of logic windows.
        logic_window.platform_type = self.platform_type;
        logic_window.backend_type = self.backend_type;

        logic_window.on_activate = on_activate;
        logic_window.on_user_event_receive = on_user_event_receive;
        logic_window.on_request_receive = on_request_receive;

        // Create the `UI` main thread.
        let join = thread::Builder::new()
            .name("tmui-main".to_string())
            .stack_size(self.ui_stack_size)
            .spawn(move || ui_runtime::<T, M>(logic_window))
            .unwrap();

        WindowsProcess::<T, M>::new().process(physical_window);

        join.join().unwrap();
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
        // Create the [`PlatformContext`] based on the platform type specified by the user.
        let platform_context = match self.platform_type {
            PlatformType::Ipc => {
                let mut platform_context = PlatformIpc::<T, M>::new(&title, width, height);
                let shared_mem_name = self.shared_mem_name.expect(
                    "`PlatformType::Ipc` need build by function `Application::shared_builder()`",
                );
                let shared_widget_id = self
                    .shared_widget_id
                    .expect("`PlatformType::Ipc` require non-None.`");
                platform_context.set_shared_widget_id(shared_widget_id);
                platform_context.with_ipc_slave(shared_mem_name);
                platform_context.initialize();

                // Ipc slave app's size was determined by the main program:
                self.width = platform_context.width();
                self.height = platform_context.height();

                platform_context.wrap()
            }
            #[cfg(windows_platform)]
            PlatformType::Win32 => {
                let mut platform_context = PlatformWin32::<T, M>::new(&title, width, height);
                if let Some(shared_mem_name) = self.shared_mem_name {
                    platform_context.with_ipc_master(shared_mem_name);
                }
                platform_context.initialize();
                platform_context.wrap()
            }
            #[cfg(x11_platform)]
            PlatformType::LinuxX11 => {
                let mut platform_context = PlatformX11::<T, M>::new(&title, width, height);
                if let Some(shared_mem_name) = self.shared_mem_name {
                    platform_context.with_ipc_master(shared_mem_name);
                }
                platform_context.initialize();
                platform_context.wrap()
            }
            #[cfg(wayland_platform)]
            PlatformType::LinuxWayland => {
                let mut platform_context = PlatformWayland::<T, M>::new(&title, width, height);
                if let Some(shared_mem_name) = self.shared_mem_name {
                    platform_context.with_ipc_master(shared_mem_name);
                }
                platform_context.initialize();
                platform_context.wrap()
            }
            #[cfg(macos_platform)]
            PlatformType::Macos => {
                let mut platform_context = PlatformMacos::<T, M>::new(&title, width, height);
                if let Some(shared_mem_name) = self.shared_mem_name {
                    platform_context.with_ipc_master(shared_mem_name, width, height);
                }
                platform_context.initialize();
                platform_context.wrap()
            }
        };
        self.platform_context = RefCell::new(Some(platform_context));
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
    pub(crate) fn send_event_ipc(evt: &Event) {
        SHARED_CHANNEL.with(|s| {
            if let Some(channel) = s.borrow().as_ref() {
                let sender = &channel
                    .downcast_ref::<SharedChannel<T, M>>()
                    .expect(INVALID_GENERIC_PARAM_ERROR)
                    .0;
                sender.send_event_ipc(evt)
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
    pub(crate) fn process_user_events(
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
                    cpu_balance.add_payload(1.);
                    on_user_event_receive(window, evt);
                }
            }
        });
    }

    #[inline]
    pub(crate) fn process_request(
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
                    cpu_balance.add_payload(1.);
                    sender.resp_request(on_request_receive(window, rqst));
                }
            }
        });
    }

    #[inline]
    pub(crate) fn process_request_ignored() {
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

    #[inline]
    pub(crate) fn set_app_started() {
        APP_STARTED.store(true, Ordering::Release);
    }

    #[inline]
    pub(crate) fn is_app_started() -> bool {
        APP_STARTED.load(Ordering::Acquire)
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
    ui_stack_size: usize,
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
            ui_stack_size: 8 * 1024 * 1024,
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
            ui_stack_size: self.ui_stack_size,
            platform_type: Default::default(),
            backend_type: Default::default(),
            platform_context: Default::default(),
            on_activate: Default::default(),
            shared_mem_name: Default::default(),
            shared_widget_id: Default::default(),
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

    pub fn ui_stack_size(mut self, size: usize) -> Self {
        self.ui_stack_size = size;
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

#[inline]
pub fn request_high_load(high_load: bool) {
    HIGH_LOAD.store(high_load, Ordering::Release)
}

#[inline]
pub fn is_high_load() -> bool {
    HIGH_LOAD.load(Ordering::Acquire)
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

#[inline]
pub fn cursor_blinking_time() -> u32 {
    event_hints().cursor_blinking_time()
}
#[inline]
pub fn set_cursor_blinking_time(blinking_time_ms: u32) {
    event_hints().set_cursor_blinking_time(blinking_time_ms)
}

#[inline]
pub fn start_drag_distance() -> i32 {
    event_hints().start_drag_distance()
}
#[inline]
pub fn set_start_drag_disntance(distance: i32) {
    event_hints().set_start_drag_distance(distance)
}
