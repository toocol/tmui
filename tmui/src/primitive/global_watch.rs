use tlib::{events::{KeyEvent, MouseEvent}, prelude::*};
use crate::{application_window::ApplicationWindow, widget::WidgetImpl};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobalWatchEvent {
    MousePress,
    MouseRelease,
    MouseMove,
    MouseWhell,
    KeyPress,
    KeyRelease,
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
    #[inline]
    fn on_global_mouse_pressed(&mut self, evt: &MouseEvent) {}

    /// Handle global mouse released event.
    /// 
    /// The coordinate of postion in `MouseEvent` was `World`.
    #[inline]
    fn on_global_mouse_released(&mut self, evt: &MouseEvent) {}

    /// Handle global mouse move event.
    /// 
    /// The coordinate of postion in `MouseEvent` was `World`.
    #[inline]
    fn on_global_mouse_move(&mut self, evt: &MouseEvent) {}

    /// Handle global mouse whell event.
    /// 
    /// The coordinate of postion in `MouseEvent` was `World`.
    #[inline]
    fn on_global_mouse_whell(&mut self, evt: &MouseEvent) {}

    /// Handle global key pressed event.
    #[inline]
    fn on_global_key_pressed(&mut self, evt: &KeyEvent) {}

    /// Handle global key released event.
    #[inline]
    fn on_global_key_released(&mut self, evt: &KeyEvent) {}
}