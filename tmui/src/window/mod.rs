pub mod win_builder;
pub mod win_config;

use self::win_config::WindowConfig;
use crate::{application::FnActivate, prelude::RawWindowHandle6};
use ahash::AHashMap;
use std::{
    fmt::Debug,
    sync::atomic::{AtomicUsize, Ordering},
};
use tlib::{object::ObjectId, winit::window::WindowId, Value};

static WINDOW_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub(crate) struct Window {
    index: usize,
    modal: bool,
    inner_window: bool,
    win_widget_id: Option<ObjectId>,
    win_cfg: Option<WindowConfig>,
    on_activate: Option<FnActivate>,
    parent: Option<WindowId>,
    params: Option<AHashMap<String, Value>>,
}

unsafe impl Send for Window {}

impl Debug for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("index", &self.index)
            .field("modal", &self.modal)
            .field("inner_window", &self.inner_window)
            .field("win_widget_id", &self.win_widget_id)
            .field("win_cfg", &self.win_cfg)
            .field("on_activate", &self.on_activate.is_some())
            .field("parent", &self.parent)
            .field("params", &self.params)
            .finish()
    }
}

impl Window {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            index: WINDOW_COUNTER.fetch_add(1, Ordering::Acquire),
            modal: false,
            inner_window: false,
            win_widget_id: None,
            win_cfg: None,
            on_activate: None,
            parent: None,
            params: None,
        }
    }
}

impl Window {
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
    pub(crate) fn set_parent_window_rwh(&mut self, rwh: RawWindowHandle6) {
        self.win_cfg.as_mut().unwrap().set_parent_window_rwh(rwh)
    }

    #[inline]
    pub(crate) fn get_parent(&self) -> WindowId {
        self.parent.unwrap()
    }

    #[inline]
    pub(crate) fn take_params(&mut self) -> Option<AHashMap<String, Value>> {
        self.params.take()
    }

    #[inline]
    pub(crate) fn is_modal(&self) -> bool {
        self.modal
    }

    #[inline]
    pub(crate) fn is_inner_window(&self) -> bool {
        self.inner_window
    }

    #[inline]
    pub(crate) fn index(&self) -> usize {
        self.index
    }

    #[inline]
    pub(crate) fn win_widget_id(&self) -> Option<ObjectId> {
        self.win_widget_id
    }
}
