use self::win_builder::WindowBuilder;
use crate::{application_window::ApplicationWindow, platform::win_config::WindowConfig};
use std::{
    fmt::Debug,
    sync::atomic::{AtomicUsize, Ordering},
};
pub mod win_builder;

static WINDOW_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct Window {
    win_cfg: Option<WindowConfig>,
    on_activate: Option<Box<dyn Fn(&mut ApplicationWindow) + Send + Sync>>,
    index: usize,
}

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
            win_cfg: None,
            on_activate: None,
            index: WINDOW_COUNTER.fetch_add(1, Ordering::Acquire),
        }
    }
}

impl Window {
    #[inline]
    pub fn builder() -> WindowBuilder {
        WindowBuilder::new()
    }

    #[inline]
    pub fn take_config(&mut self) -> WindowConfig {
        self.win_cfg.take().unwrap()
    }

    #[inline]
    pub fn take_on_activate(
        &mut self,
    ) -> Option<Box<dyn Fn(&mut ApplicationWindow) + Send + Sync>> {
        self.on_activate.take()
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }
}
