use super::{Window, win_config::WindowConfig};
use crate::{application::FnActivate, application_window::ApplicationWindow};

#[derive(Default)]
pub struct WindowBuilder {
    win_cfg: Option<WindowConfig>,
    on_activate: Option<FnActivate>,
}

impl WindowBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn config(mut self, config: WindowConfig) -> Self {
        self.win_cfg = Some(config);
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

        window
    }
}
