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
    font::mgr::FontManager,
    graphics::icon::Icon,
    platform::{PlatformContext, PlatformIpc, PlatformType},
    prelude::CloseHandlerMgr,
    primitive::{cpu_balance::CpuBalance, shared_channel::SharedChannel},
    runtime::{start_ui_runtime, windows_process::WindowsProcess},
    window::win_config::{WindowConfig, WindowConfigBuilder},
};
use log::error;
use std::{
    any::Any,
    cell::RefCell,
    marker::PhantomData,
    sync::{
        atomic::{AtomicBool, AtomicU8, Ordering},
        Once,
    },
};
use tipc::{WithIpcMaster, WithIpcSlave};
use tlib::{events::Event, figure::Size, winit::window::WindowButtons};

thread_local! {
    pub(crate) static IS_UI_MAIN_THREAD: RefCell<bool> = const { RefCell::new(false) };
    pub(crate) static IS_UI_THREAD: RefCell<bool> = const { RefCell::new(false) };
    pub(crate) static SHARED_CHANNEL: RefCell<Option<Box<dyn Any>>> = RefCell::new(None);
}

const INVALID_GENERIC_PARAM_ERROR: &str =
    "Invalid generic parameters, please use generic parameter defined on Application.";
pub(crate) static APP_STARTED: AtomicBool = AtomicBool::new(false);
pub(crate) static APP_STOPPED: AtomicBool = AtomicBool::new(false);
pub(crate) static IS_SHARED: AtomicBool = AtomicBool::new(false);
pub(crate) static HIGH_LOAD: AtomicBool = AtomicBool::new(false);
pub(crate) static UI_THREAD_CNT: AtomicU8 = AtomicU8::new(0);
static ONCE: Once = Once::new();

pub type FnActivate = Box<dyn FnOnce(&mut ApplicationWindow) + Send>;
pub type FnRunAfter = Box<dyn FnOnce(&mut ApplicationWindow)>;
pub type FnUserEventReceive<T> = Box<dyn Fn(&mut ApplicationWindow, T) + Send + Sync>;
pub type FnRequestReceive<T> = Box<dyn Fn(&mut ApplicationWindow, T) -> Option<T> + Send + Sync>;

/// ### The main application of tmui. <br>
///
/// For common single process gui application, use [`Application::builder`] <br>
/// Multile process gui application, use [`Application::shared_builder`] instead to define ipc shared memory UserEvent and Request type. <br>
///
/// ### These generic types were used for ipc shared memory application. <br>
/// `T`: Generic type for [`IpcEvent::UserEvent`](tipc::ipc_event::IpcEvent::UserEvent). <br>
/// `M`: Generic type for blocked request with response in ipc communication.
pub struct Application<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> {
    ui_stack_size: usize,

    win_config: RefCell<Option<WindowConfig>>,

    platform_type: PlatformType,
    backend_type: BackendType,

    platform_context: RefCell<Option<Box<dyn PlatformContext<T, M>>>>,

    on_activate: RefCell<Option<FnActivate>>,
    on_user_event_receive: RefCell<Option<FnUserEventReceive<T>>>,
    on_request_receive: RefCell<Option<FnRequestReceive<M>>>,

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
        let (mut logic_window, physical_window) = platform_context.create_window(
            self.win_config.borrow_mut().take().unwrap(),
            None,
            None,
        );

        // Get the customize event handle functions.
        let on_activate = self.on_activate.borrow_mut().take();
        let on_user_event_receive = self.on_user_event_receive.borrow_mut().take();
        let on_request_receive = self.on_request_receive.borrow_mut().take();

        // Set the fields of logic windows.
        logic_window.on_activate = on_activate;
        logic_window.on_user_event_receive = on_user_event_receive;
        logic_window.on_request_receive = on_request_receive;

        // Load the fonts.
        FontManager::load_fonts();

        // Create the `UI` main thread.
        let join = start_ui_runtime(0, self.ui_stack_size, logic_window);

        WindowsProcess::<T, M>::new(self.ui_stack_size, platform_context.as_ref())
            .process(physical_window);

        crate::opti::tracker::Tracker::output_file()
            .unwrap_or_else(|err| error!("Output tracker file failed, error = {:?}", err));

        join.join().unwrap();

        CloseHandlerMgr::process()
    }

    /// The method will be activate when the ui thread was created, and activated in the ui thread. <br>
    /// UI components should create in here.
    pub fn connect_activate<F>(&self, f: F)
    where
        F: FnOnce(&mut ApplicationWindow) + Send + Sync + 'static,
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
        // Create the [`PlatformContext`] based on the platform type specified by the user.
        let platform_context = match self.platform_type {
            PlatformType::Ipc => {
                let mut platform_context =
                    PlatformIpc::<T, M>::new(self.platform_type, self.backend_type);
                let shared_mem_name = self.shared_mem_name.expect(
                    "`PlatformType::Ipc` need build by function `Application::shared_builder()`",
                );
                let shared_widget_id = self
                    .shared_widget_id
                    .expect("`PlatformType::Ipc` require non-None.`");
                platform_context.set_shared_widget_id(shared_widget_id);
                platform_context.with_ipc_slave(shared_mem_name);

                platform_context.wrap()
            }
            #[cfg(windows_platform)]
            PlatformType::Win32 => {
                let mut platform_context =
                    PlatformWin32::<T, M>::new(self.platform_type, self.backend_type);
                if let Some(shared_mem_name) = self.shared_mem_name {
                    platform_context.with_ipc_master(shared_mem_name);
                }
                platform_context.wrap()
            }
            #[cfg(x11_platform)]
            PlatformType::LinuxX11 => {
                let mut platform_context =
                    PlatformX11::<T, M>::new(self.platform_type, self.backend_type);
                if let Some(shared_mem_name) = self.shared_mem_name {
                    platform_context.with_ipc_master(shared_mem_name);
                }
                platform_context.wrap()
            }
            #[cfg(wayland_platform)]
            PlatformType::LinuxWayland => {
                let mut platform_context =
                    PlatformWayland::<T, M>::new(self.platform_type, self.backend_type);
                if let Some(shared_mem_name) = self.shared_mem_name {
                    platform_context.with_ipc_master(shared_mem_name);
                }
                platform_context.wrap()
            }
            #[cfg(macos_platform)]
            PlatformType::Macos => {
                let mut platform_context =
                    PlatformMacos::<T, M>::new(self.platform_type, self.backend_type);
                if let Some(shared_mem_name) = self.shared_mem_name {
                    platform_context.with_ipc_master(shared_mem_name);
                }
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
        on_user_event_receive: &FnUserEventReceive<T>,
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
        on_request_receive: &FnRequestReceive<M>,
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
                if receiver.receive_request().is_some() {
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

/// Determine whether the current thread is the UI thread.
#[inline]
pub fn is_ui_thread() -> bool {
    IS_UI_THREAD.with(|is_ui| *is_ui.borrow())
}

/// Determine whether the current thread is the UI main thread.
#[inline]
pub fn is_ui_main_thread() -> bool {
    IS_UI_MAIN_THREAD.with(|is_main| *is_main.borrow())
}

/// Get the count of ui threads.
#[inline]
pub fn ui_thread_count() -> u8 {
    UI_THREAD_CNT.load(Ordering::Relaxed)
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
    width: Option<u32>,
    height: Option<u32>,
    win_cfg_bld: WindowConfigBuilder,
    ui_stack_size: usize,
    shared_mem_name: Option<&'static str>,
    shared_widget_id: Option<&'static str>,
    opti_track: bool,
    _user_event: PhantomData<T>,
    _request: PhantomData<M>,
}

impl<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send> ApplicationBuilder<T, M> {
    // Private constructor.
    #[inline]
    fn new(shared_mem_name: Option<&'static str>) -> Self {
        Self {
            platform: None,
            backend: None,
            width: None,
            height: None,
            ui_stack_size: 8 * 1024 * 1024,
            win_cfg_bld: WindowConfigBuilder::default(),
            shared_mem_name,
            shared_widget_id: None,
            opti_track: false,
            _user_event: PhantomData,
            _request: PhantomData,
        }
    }

    /// Build the [`Application`]
    pub fn build(mut self) -> Application<T, M> {
        let mut app = Application {
            ui_stack_size: self.ui_stack_size,
            win_config: Default::default(),
            platform_type: Default::default(),
            backend_type: Default::default(),
            platform_context: Default::default(),
            on_activate: Default::default(),
            shared_mem_name: Default::default(),
            shared_widget_id: Default::default(),
            on_user_event_receive: Default::default(),
            on_request_receive: Default::default(),
        };

        if let Some(platform) = self.platform {
            app.platform_type = platform
        }

        if let Some(width) = self.width {
            self.win_cfg_bld = self.win_cfg_bld.width(width)
        } else if app.platform_type == PlatformType::Ipc {
            self.win_cfg_bld = self.win_cfg_bld.width(0)
        } else {
            panic!("Application window should specify the `width`.")
        }

        if let Some(height) = self.height {
            self.win_cfg_bld = self.win_cfg_bld.height(height)
        } else if app.platform_type == PlatformType::Ipc {
            self.win_cfg_bld = self.win_cfg_bld.height(0)
        } else {
            panic!("Application window should specify the `height`.")
        }

        if let Some(shared_mem_name) = self.shared_mem_name {
            app.shared_mem_name = Some(shared_mem_name)
        }
        if let Some(shared_widget_id) = self.shared_widget_id {
            app.shared_widget_id = Some(shared_widget_id)
        }

        if let Some(backend) = self.backend {
            app.backend_type = backend
        }

        if self.opti_track {
            crate::opti::tracker::set_tracked();
        }

        if app.platform_type == PlatformType::Ipc && app.shared_widget_id.is_none() {
            panic!("Shared application with `PlatformIpc` should specified the `shared_widget_id`.")
        }
        app.win_config = RefCell::new(Some(self.win_cfg_bld.build()));

        app.startup_initialize();
        app
    }

    /// Set the platform type of application.
    ///
    /// The default platform was os specified, the alternative platform was [`Ipc`](PlatformType::Ipc).
    #[inline]
    pub fn platform(mut self, platform: PlatformType) -> Self {
        self.platform = Some(platform);
        self
    }

    /// Set the render backend of application.
    #[inline]
    pub fn backend(mut self, backend: BackendType) -> Self {
        self.backend = Some(backend);
        self
    }

    /// Set the title of application main window.
    ///
    /// The default value was "Tmui Window".
    #[inline]
    pub fn title(mut self, title: &'static str) -> Self {
        self.win_cfg_bld = self.win_cfg_bld.title(title.to_string());
        self
    }

    /// Set the width of application main window.
    #[inline]
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the height of application main window.
    #[inline]
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set the maxmium size of application main window.
    #[inline]
    pub fn max_size<S: Into<Size>>(mut self, size: S) -> Self {
        self.win_cfg_bld = self.win_cfg_bld.max_size(size);
        self
    }

    /// Set the minimum size of application main window.
    #[inline]
    pub fn min_size<S: Into<Size>>(mut self, size: S) -> Self {
        self.win_cfg_bld = self.win_cfg_bld.min_size(size);
        self
    }

    /// Set the icon of application main window.
    #[inline]
    pub fn icon(mut self, icon: Icon) -> Self {
        self.win_cfg_bld = self.win_cfg_bld.win_icon(icon);
        self
    }

    /// Set whether the application main window should have a border, a title bar, etc.
    ///
    /// The default value was `true`.
    #[inline]
    pub fn decoration(mut self, decoration: bool) -> Self {
        self.win_cfg_bld = self.win_cfg_bld.decoration(decoration);
        self
    }

    /// Set whether the application main window will support transparency.
    ///
    /// The default value was `false`.
    ///
    /// The child window create from main window will follow the transparency strategy.
    #[inline]
    pub fn transparent(mut self, transparent: bool) -> Self {
        self.win_cfg_bld = self.win_cfg_bld.transparent(transparent);
        self
    }

    /// Whether the background of the application main window should be blurred by the system.
    ///
    /// The default value was `false`.
    #[inline]
    pub fn blur(mut self, blur: bool) -> Self {
        self.win_cfg_bld = self.win_cfg_bld.blur(blur);
        self
    }

    /// Set whether the application main window will be initially visible or hidden.
    ///
    /// The default value was `true`.
    #[inline]
    pub fn visible(mut self, visible: bool) -> Self {
        self.win_cfg_bld = self.win_cfg_bld.visible(visible);
        self
    }

    /// Set whether the application main window is resizable or not.
    ///
    /// The default value was `true`.
    #[inline]
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.win_cfg_bld = self.win_cfg_bld.resizable(resizable);
        self
    }

    /// Request that the application main window is maximized upon creation.
    ///
    /// The default value was `false`.
    #[inline]
    pub fn maximized(mut self, maximized: bool) -> Self {
        self.win_cfg_bld = self.win_cfg_bld.maximized(maximized);
        self
    }

    /// Set whether the application main window will be initially focused or not.
    ///
    /// The default value was `true`.
    #[inline]
    pub fn active(mut self, active: bool) -> Self {
        self.win_cfg_bld = self.win_cfg_bld.active(active);
        self
    }

    /// Sets the enabled window buttons for application main window.
    ///
    /// The default is [`WindowButtons::all()`]
    #[inline]
    pub fn enable_buttons(mut self, enable_buttons: WindowButtons) -> Self {
        self.win_cfg_bld = self.win_cfg_bld.enable_buttons(enable_buttons);
        self
    }

    /// Set the thread stack size of each ui thread.
    ///
    /// The default value was `8Mb`.
    #[inline]
    pub fn ui_stack_size(mut self, size: usize) -> Self {
        self.ui_stack_size = size;
        self
    }

    /// Set the shared widget id of shared memory(cross process render) application.
    #[inline]
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
    #[inline]
    pub fn cpu_payload_threshold(self, threshold: usize) -> Self {
        CpuBalance::set_payload_threshold(threshold);
        self
    }

    /// Allow the program to record optimization tracking information.
    ///
    /// The default value was `false`.
    #[inline]
    pub fn opti_track(mut self, track: bool) -> Self {
        self.opti_track = track;
        self
    }

    /// Set to `true` to hide the window until the UI is loaded.
    ///
    /// The default value was `false`.
    #[inline]
    pub fn defer_display(mut self, defer_display: bool) -> Self {
        self.win_cfg_bld = self.win_cfg_bld.visible(!defer_display);
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
