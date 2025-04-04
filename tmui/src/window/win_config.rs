use crate::{graphics::icon::Icon, prelude::RawWindowHandle6, primitive::Message};
use derivative::Derivative;
use tlib::{
    figure::{Point, Size},
    typedef::{WinitIcon, WinitPosition, WinitWindowBuilder},
    winit::{
        dpi::{PhysicalPosition, PhysicalSize}, error::OsError, event_loop::EventLoopWindowTarget, window::{Window, WindowButtons, WindowLevel}
    },
};
#[cfg(windows_platform)]
use tlib::winit::platform::windows::WindowBuilderExtWindows;

type WinitSize = tlib::winit::dpi::Size;

pub(crate) fn build_window(
    win_config: WindowConfig,
    target: &EventLoopWindowTarget<Message>,
) -> Result<Window, OsError> {
    win_config.create_window_builder().build(target)
}

#[derive(Debug, Clone)]
pub struct WindowConfig {
    title: String,
    /// The width of window.
    width: u32,
    /// The height of window.
    height: u32,
    /// The maxmium size of window.
    max_size: Option<Size>,
    /// The minimum size of window.
    min_size: Option<Size>,
    /// The icon of window.
    win_icon: Option<WinitIcon>,
    /// Whether the window should have a border, a title bar, etc.
    decoration: bool,
    /// Whether the window will support transparency.
    transparent: bool,
    /// Whether the background of the window should be blurred by the system.
    blur: bool,
    /// Whether the window will be initially visible or hidden.
    visible: bool,
    /// whether the window is resizable or not.
    resizable: bool,
    /// Request that the window is maximized upon creation.
    maximized: bool,
    /// Whether the window will be initially focused or not.
    active: bool,
    /// The enabled window buttons.
    enable_buttons: WindowButtons,
    /// The initial position of new window.
    position: Option<Point>,
    /// The `RawWindowHandle` of system level parent window.
    parent_window: Option<RawWindowHandle6>,
    /// Window level of new window
    win_level: WindowLevel,
    /// Window defer display or not.
    defer_display: bool,
    /// Skip the taskbar setting or not.
    skip_taskbar: bool,
}

impl WindowConfig {
    #[inline]
    fn new() -> Self {
        Self {
            title: Default::default(),
            width: Default::default(),
            height: Default::default(),
            max_size: Default::default(),
            min_size: Default::default(),
            win_icon: Default::default(),
            decoration: Default::default(),
            transparent: Default::default(),
            blur: Default::default(),
            visible: Default::default(),
            resizable: Default::default(),
            maximized: Default::default(),
            active: Default::default(),
            enable_buttons: WindowButtons::all(),
            position: Default::default(),
            parent_window: Default::default(),
            win_level: Default::default(),
            defer_display: Default::default(),
            skip_taskbar: Default::default(),
        }
    }

    #[inline]
    pub fn builder() -> WindowConfigBuilder {
        WindowConfigBuilder::default()
    }

    #[inline]
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn max_size(&self) -> Option<Size> {
        self.max_size
    }

    #[inline]
    pub fn min_size(&self) -> Option<Size> {
        self.min_size
    }

    #[inline]
    pub fn win_icon(&self) -> Option<WinitIcon> {
        self.win_icon.clone()
    }

    #[inline]
    pub fn decoration(&self) -> bool {
        self.decoration
    }

    #[inline]
    pub fn transparent(&self) -> bool {
        self.transparent
    }

    #[inline]
    pub fn blur(&self) -> bool {
        self.blur
    }

    #[inline]
    pub fn visible(&self) -> bool {
        self.visible
    }

    #[inline]
    pub fn resizable(&self) -> bool {
        self.resizable
    }

    #[inline]
    pub fn maximized(&self) -> bool {
        self.maximized
    }

    #[inline]
    pub fn active(&self) -> bool {
        self.active
    }

    #[inline]
    pub fn position(&self) -> Option<Point> {
        self.position
    }

    #[inline]
    pub fn win_level(&self) -> WindowLevel {
        self.win_level
    }

    #[inline]
    pub fn defer_display(&self) -> bool {
        self.defer_display
    }

    #[inline]
    pub fn skip_taskbar(&self) -> bool {
        self.skip_taskbar
    }

    #[inline]
    pub(crate) fn set_parent_window_rwh(&mut self, rwh: RawWindowHandle6) {
        self.parent_window = Some(rwh)
    }

    pub(crate) fn create_window_builder(self) -> WinitWindowBuilder {
        let (width, height) = self.size();

        let mut window_bld = WinitWindowBuilder::new()
            .with_title(&self.title)
            .with_inner_size(WinitSize::Physical(PhysicalSize::new(width, height)))
            .with_decorations(self.decoration)
            .with_blur(self.blur)
            .with_visible(self.visible)
            .with_resizable(self.resizable)
            .with_maximized(self.maximized)
            .with_active(self.active)
            .with_enabled_buttons(self.enable_buttons)
            .with_window_level(self.win_level);

        #[cfg(windows_platform)]
        {
            window_bld = window_bld.with_skip_taskbar(self.skip_taskbar);
        }

        if let Some(max_size) = self.max_size {
            window_bld = window_bld.with_max_inner_size(WinitSize::Physical(PhysicalSize::new(
                max_size.width() as u32,
                max_size.height() as u32,
            )))
        }

        if let Some(min_size) = self.min_size {
            window_bld = window_bld.with_min_inner_size(WinitSize::Physical(PhysicalSize::new(
                min_size.width() as u32,
                min_size.height() as u32,
            )))
        }

        if let Some(icon) = self.win_icon {
            window_bld = window_bld.with_window_icon(Some(icon))
        }

        if let Some(pos) = self.position {
            let position = WinitPosition::Physical(PhysicalPosition::new(pos.x(), pos.y()));
            window_bld = window_bld.with_position(position);
        }

        if let Some(rwh) = self.parent_window {
            window_bld = unsafe { window_bld.with_parent_window(Some(rwh)) };
        } else {
            window_bld = window_bld.with_transparent(self.transparent);
        }

        window_bld
    }
}

#[derive(Derivative, Debug)]
#[derivative(Default)]
pub struct WindowConfigBuilder {
    #[derivative(Default(value = "\"Tmui Window\".to_string()"))]
    title: String,
    width: Option<u32>,
    height: Option<u32>,
    max_size: Option<Size>,
    min_size: Option<Size>,
    win_icon: Option<Icon>,
    #[derivative(Default(value = "true"))]
    decoration: bool,
    transparent: bool,
    blur: bool,
    #[derivative(Default(value = "true"))]
    visible: bool,
    #[derivative(Default(value = "true"))]
    resizable: bool,
    maximized: bool,
    #[derivative(Default(value = "true"))]
    active: bool,
    #[derivative(Default(value = "WindowButtons::all()"))]
    enable_buttons: WindowButtons,
    position: Option<Point>,
    win_level: WindowLevel,
    defer_display: bool,
    skip_taskbar: bool,
}

impl WindowConfigBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the title of window.
    ///
    /// The default value was "Tmui Window".
    #[inline]
    pub fn title(mut self, title: impl ToString) -> Self {
        self.title = title.to_string();
        self
    }

    /// Set the width of window.
    #[inline]
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the height of window.
    #[inline]
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set the maxmium size of window.
    #[inline]
    pub fn max_size<T: Into<Size>>(mut self, max_size: T) -> Self {
        self.max_size = Some(max_size.into());
        self
    }

    /// Set the minimum size of window.
    #[inline]
    pub fn min_size<T: Into<Size>>(mut self, min_size: T) -> Self {
        self.min_size = Some(min_size.into());
        self
    }

    /// Set the icon of window.
    #[inline]
    pub fn win_icon(mut self, icon: Icon) -> Self {
        self.win_icon = Some(icon);
        self
    }

    /// Set whether the window should have a border, a title bar, etc.
    ///
    /// The default value was `true`.
    #[inline]
    pub fn decoration(mut self, decoration: bool) -> Self {
        self.decoration = decoration;
        self
    }

    /// Set whether the window will support transparency.
    ///
    /// The default value was `false`.
    ///
    /// [` No need set for child window, it will follow parent window's transparency strategy. `]
    #[inline]
    pub fn transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }

    /// Set whether the background of the window should be blurred by the system.
    ///
    /// The default value was `false`.
    #[inline]
    pub fn blur(mut self, blur: bool) -> Self {
        self.blur = blur;
        self
    }

    /// Set whether the window will be initially visible or hidden
    ///
    /// The default value was `true`.
    #[inline]
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Set whether the window is resizable or not.
    ///
    /// The default value was `true`.
    #[inline]
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Request that the window is maximized upon creation.
    ///
    /// The default value was `false`.
    #[inline]
    pub fn maximized(mut self, maximized: bool) -> Self {
        self.maximized = maximized;
        self
    }

    /// Set whether the window will be initially focused or not.
    ///
    /// The default value was `true`.
    #[inline]
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Sets the enabled window buttons.
    ///
    /// The default is [`WindowButtons::all()`]
    #[inline]
    pub fn enable_buttons(mut self, enable_buttons: WindowButtons) -> Self {
        self.enable_buttons = enable_buttons;
        self
    }

    /// Sets the initial position of window.
    ///
    /// The default is [`None`]
    #[inline]
    pub fn position<T: Into<Point>>(mut self, position: T) -> Self {
        self.position = Some(position.into());
        self
    }

    /// Set the window level of new window.
    ///
    /// The default value was [WindowLevel::Normal]
    #[inline]
    pub fn win_level(mut self, win_level: WindowLevel) -> Self {
        self.win_level = win_level;
        self
    }

    /// Set the window defer display or not.
    /// 
    /// The default value was [`false`]
    #[inline]
    pub fn defer_display(mut self, defer_display: bool) -> Self {
        self.defer_display = defer_display;
        self
    }

    /// Set the window register on the taskbar or not.
    /// 
    /// The default value was [`false`]
    #[inline]
    pub fn skip_taskbar(mut self, skip_taskbar: bool) -> Self {
        self.skip_taskbar = skip_taskbar;
        self
    }

    #[inline]
    pub fn build(self) -> WindowConfig {
        let mut cfg = WindowConfig::new();

        cfg.title = self.title;
        cfg.width = self.width.expect("`WindowConfig` must specify the width");
        cfg.height = self.height.expect("`WindowConfig` must specify the height");
        cfg.max_size = self.max_size;
        cfg.min_size = self.min_size;
        if let Some(icon) = self.win_icon {
            cfg.win_icon = Some(icon.into());
        }
        cfg.decoration = self.decoration;
        cfg.transparent = self.transparent;
        cfg.blur = self.blur;
        cfg.visible = self.visible;
        cfg.resizable = self.resizable;
        cfg.maximized = self.maximized;
        cfg.active = self.active;
        cfg.enable_buttons = self.enable_buttons;
        cfg.position = self.position;
        cfg.win_level = self.win_level;
        cfg.defer_display = self.defer_display;
        cfg.skip_taskbar = self.skip_taskbar;

        cfg
    }
}
