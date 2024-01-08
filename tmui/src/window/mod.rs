pub mod win_config;
pub mod win_builder;

use tlib::winit::raw_window_handle::RawWindowHandle;
use self::{win_builder::WindowBuilder, win_config::WindowConfig};
use crate::application_window::ApplicationWindow;
use std::{
    fmt::Debug,
    sync::atomic::{AtomicUsize, Ordering},
};

static WINDOW_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct Window {
    index: usize,
    parent: Option<RawWindowHandle>,
    win_cfg: Option<WindowConfig>,
    on_activate: Option<Box<dyn Fn(&mut ApplicationWindow) + Send + Sync>>,
    child_window: bool,
}

unsafe impl Send for Window {}

impl Debug for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("win_cfg", &self.win_cfg)
            .field("on_activate", &self.on_activate.is_some())
            .finish()
    }
}

impl Window {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            index: WINDOW_COUNTER.fetch_add(1, Ordering::Acquire),
            parent: None,
            win_cfg: None,
            on_activate: None,
            child_window: false,
        }
    }
}

impl Window {
    #[inline]
    pub fn builder() -> WindowBuilder {
        WindowBuilder::new()
    }

    #[inline]
    pub(crate) fn take_config(&mut self) -> WindowConfig {
        self.win_cfg.take().unwrap()
    }

    #[inline]
    pub(crate) fn take_on_activate(
        &mut self,
    ) -> Option<Box<dyn Fn(&mut ApplicationWindow) + Send + Sync>> {
        self.on_activate.take()
    }

    #[inline]
    pub(crate) fn set_parent(&mut self, handle: RawWindowHandle) {
        self.parent = Some(handle)
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    #[inline]
    pub fn parent(&self) -> Option<RawWindowHandle> {
        self.parent
    }

    #[inline]
    pub fn is_child_window(&self) -> bool {
        self.child_window
    }
}
