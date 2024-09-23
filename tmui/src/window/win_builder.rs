use super::{win_config::WindowConfig, Window};
use crate::{application::FnActivate, application_window::ApplicationWindow};
use std::collections::HashMap;
use tlib::{object::ObjectId, values::ToValue, Value};

#[derive(Default)]
pub struct WindowBuilder {
    win_cfg: Option<WindowConfig>,
    /// Os level child window or not.
    child_window: bool,
    win_widget_id: ObjectId,
    modal: bool,
    on_activate: Option<FnActivate>,
    params: HashMap<String, Value>,
}

impl WindowBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the configuration of new window.
    #[inline]
    pub fn config(mut self, config: WindowConfig) -> Self {
        self.win_cfg = Some(config);
        self
    }

    /// Set the new window is modal or not.
    ///
    /// The default value was [`false`]
    #[inline]
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    /// Set the new window is os level child window of current window or not.
    ///
    /// The default value was [`false`]
    #[inline]
    pub fn child_window(mut self, is_child_window: bool) -> Self {
        self.child_window = is_child_window;
        self
    }

    #[inline]
    pub fn on_activate<F: 'static + Fn(&mut ApplicationWindow) + Send + Sync>(
        mut self,
        on_activate: F,
    ) -> Self {
        self.on_activate = Some(Box::new(on_activate));
        self
    }

    #[inline]
    pub fn param(mut self, key: impl ToString, val: impl ToValue) -> Self {
        self.params.insert(key.to_string(), val.to_value());
        self
    }

    #[inline]
    pub(crate) fn win_widget_id(mut self, id: ObjectId) -> Self {
        self.win_widget_id = id;
        self
    }

    #[inline]
    pub(crate) fn build(self) -> Window {
        let mut window = Window::new();

        window.win_cfg = Some(
            self.win_cfg
                .expect("build `Window` must specify the window config."),
        );
        window.on_activate = self.on_activate;
        window.modal = self.modal;
        window.child_window = self.child_window;
        window.win_widget_id = self.win_widget_id;
        window.params = Some(self.params);

        window
    }
}
