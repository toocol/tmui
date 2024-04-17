pub mod win_builder;
pub mod win_config;

use self::{win_builder::WindowBuilder, win_config::WindowConfig};
use crate::application::FnActivate;
use std::{
    fmt::Debug,
    sync::atomic::{AtomicUsize, Ordering},
};

static WINDOW_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub struct Window {
    index: usize,
    win_cfg: Option<WindowConfig>,
    on_activate: Option<FnActivate>,
}

unsafe impl Send for Window {}

impl Debug for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("index", &self.index)
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
            win_cfg: None,
            on_activate: None,
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
    ) -> Option<FnActivate> {
        self.on_activate.take()
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }
}
