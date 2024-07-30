pub mod win_builder;
pub mod win_config;

use tlib::{winit::window::WindowId, Value};

use self::{win_builder::WindowBuilder, win_config::WindowConfig};
use crate::application::FnActivate;
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::atomic::{AtomicUsize, Ordering},
};

static WINDOW_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub struct Window {
    index: usize,
    modal: bool,
    win_cfg: Option<WindowConfig>,
    on_activate: Option<FnActivate>,
    parent: Option<WindowId>,
    params: Option<HashMap<String, Value>>,
}

unsafe impl Send for Window {}

impl Debug for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("index", &self.index)
            .field("modal", &self.modal)
            .field("win_cfg", &self.win_cfg)
            .field("on_activate", &self.on_activate.is_some())
            .field("parent", &self.parent)
            .finish()
    }
}

impl Window {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            index: WINDOW_COUNTER.fetch_add(1, Ordering::Acquire),
            modal: false,
            win_cfg: None,
            on_activate: None,
            parent: None,
            params: None,
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
    pub(crate) fn take_on_activate(&mut self) -> Option<FnActivate> {
        self.on_activate.take()
    }

    #[inline]
    pub(crate) fn set_parent(&mut self, parent: WindowId) {
        self.parent = Some(parent);
    }

    #[inline]
    pub(crate) fn get_parent(&self) -> WindowId {
        self.parent.unwrap()
    }

    #[inline]
    pub(crate) fn take_params(&mut self) -> Option<HashMap<String, Value>> {
        self.params
            .take()
    }
}

impl Window {
    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    #[inline]
    pub fn is_modal(&self) -> bool {
        self.modal
    }
}
