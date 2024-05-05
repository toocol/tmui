use super::{win_config::WindowConfig, Window};
use crate::{application::FnActivate, application_window::ApplicationWindow};

#[derive(Default)]
pub struct WindowBuilder {
    win_cfg: Option<WindowConfig>,
    modal: bool,
    on_activate: Option<FnActivate>,
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

    #[inline]
    pub fn on_activate<F: 'static + Fn(&mut ApplicationWindow) + Send + Sync>(
        mut self,
        on_activate: F,
    ) -> Self {
        self.on_activate = Some(Box::new(on_activate));
        self
    }

    #[inline]
    pub fn build(self) -> Window {
        let mut window = Window::new();

        window.win_cfg = Some(
            self.win_cfg
                .expect("build `Window` must specify the window config."),
        );
        window.on_activate = self.on_activate;
        window.modal = self.modal;

        window
    }
}
