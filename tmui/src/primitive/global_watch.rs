use crate::{application_window::ApplicationWindow, widget::WidgetImpl};
use tlib::{
    events::{KeyEvent, MouseEvent},
    prelude::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobalWatchEvent {
    MousePressed,
    MouseReleased,
    MouseMove,
    MouseWhell,
    KeyPressed,
    KeyReleased,
}

#[reflect_trait]
pub trait GlobalWatch: WidgetImpl + GlobalWatchImpl {
    #[inline]
    fn register_global_watch(&self) {
        let id = self.id();
        let window = ApplicationWindow::window_of(self.window_id());

        for ty in self.watch_list() {
            window.register_global_watch(id, ty)
        }
    }

    fn watch_list(&self) -> Vec<GlobalWatchEvent>;
}

#[allow(unused_variables)]
pub trait GlobalWatchImpl: WidgetImpl {
    /// Handle global mouse pressed event.
    ///
    /// The coordinate of postion in `MouseEvent` was `World`.
    ///
    /// @return prevent event propagate or not.
    #[inline]
    fn on_global_mouse_pressed(&mut self, evt: &MouseEvent) -> bool {
        false
    }

    /// Handle global mouse released event.
    ///
    /// The coordinate of postion in `MouseEvent` was `World`.
    ///
    /// @return prevent event propagate or not.
    #[inline]
    fn on_global_mouse_released(&mut self, evt: &MouseEvent) -> bool {
        false
    }

    /// Handle global mouse move event.
    ///
    /// The coordinate of postion in `MouseEvent` was `World`.
    ///
    /// @return prevent event propagate or not.
    ///
    /// ## Notice
    /// if prevent the global mouse event, will not trigger widget's `MouseEnter`, `MouseLeave`... events.
    #[inline]
    fn on_global_mouse_move(&mut self, evt: &MouseEvent) -> bool {
        false
    }

    /// Handle global mouse whell event.
    ///
    /// The coordinate of postion in `MouseEvent` was `World`.
    ///
    /// @return prevent event propagate or not.
    #[inline]
    fn on_global_mouse_whell(&mut self, evt: &MouseEvent) -> bool {
        false
    }

    /// Handle global key pressed event.
    ///
    /// @return prevent event propagate or not.
    #[inline]
    fn on_global_key_pressed(&mut self, evt: &KeyEvent) -> bool {
        false
    }

    /// Handle global key released event.
    ///
    /// @return prevent event propagate or not.
    #[inline]
    fn on_global_key_released(&mut self, evt: &KeyEvent) -> bool {
        false
    }
}
