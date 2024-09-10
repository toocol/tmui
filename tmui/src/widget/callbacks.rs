use super::WidgetImpl;
use tlib::events::{KeyEvent, MouseEvent};

pub type MouseEnterFn = Box<dyn Fn(&mut dyn WidgetImpl)>;
pub type MouseLeaveFn = Box<dyn Fn(&mut dyn WidgetImpl)>;
pub type MousePressedFn = Box<dyn Fn(&mut dyn WidgetImpl, &MouseEvent)>;
pub type MouseReleasedFn = Box<dyn Fn(&mut dyn WidgetImpl, &MouseEvent)>;
pub type MouseMoveFn = Box<dyn Fn(&mut dyn WidgetImpl, &MouseEvent)>;
pub type MouseWheelFn = Box<dyn Fn(&mut dyn WidgetImpl, &MouseEvent)>;
pub type KeyPressedFn = Box<dyn Fn(&mut dyn WidgetImpl, &KeyEvent)>;
pub type KeyReleasedFn = Box<dyn Fn(&mut dyn WidgetImpl, &KeyEvent)>;
pub type WindowMinimizedFn = Box<dyn Fn(&mut dyn WidgetImpl)>;
pub type WindowMaximizedFn = Box<dyn Fn(&mut dyn WidgetImpl)>;
pub type WindowRestoredFn = Box<dyn Fn(&mut dyn WidgetImpl)>;
pub type VisibilityChangedFn = Box<dyn Fn(&mut dyn WidgetImpl, bool)>;

#[derive(Default)]
pub struct Callbacks {
    pub(crate) mouse_enter: Option<MouseEnterFn>,
    pub(crate) mouse_leave: Option<MouseLeaveFn>,
    pub(crate) mouse_pressed: Option<MousePressedFn>,
    pub(crate) mouse_released: Option<MouseReleasedFn>,
    pub(crate) mouse_move: Option<MouseMoveFn>,
    pub(crate) mouse_wheel: Option<MouseMoveFn>,
    pub(crate) key_pressed: Option<KeyPressedFn>,
    pub(crate) key_released: Option<KeyReleasedFn>,
    pub(crate) window_minimized: Option<WindowMinimizedFn>,
    pub(crate) window_maximized: Option<WindowMaximizedFn>,
    pub(crate) window_restored: Option<WindowRestoredFn>,
    pub(crate) visibility_changed: Option<VisibilityChangedFn>,
}

pub trait CallbacksRegister: WidgetImpl {
    #[inline]
    fn register_mouse_enter<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl) + 'static,
    {
        self.callbacks_mut().mouse_enter = Some(Box::new(f));
    }

    #[inline]
    fn register_mouse_leave<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl) + 'static,
    {
        self.callbacks_mut().mouse_leave = Some(Box::new(f));
    }

    #[inline]
    fn register_mouse_pressed<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl, &MouseEvent) + 'static,
    {
        self.callbacks_mut().mouse_pressed = Some(Box::new(f))
    }

    #[inline]
    fn register_mouse_released<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl, &MouseEvent) + 'static,
    {
        self.callbacks_mut().mouse_released = Some(Box::new(f))
    }

    #[inline]
    fn register_mouse_move<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl, &MouseEvent) + 'static,
    {
        self.callbacks_mut().mouse_move = Some(Box::new(f))
    }

    #[inline]
    fn register_mouse_wheel<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl, &MouseEvent) + 'static,
    {
        self.callbacks_mut().mouse_wheel = Some(Box::new(f))
    }

    #[inline]
    fn register_key_pressed<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl, &KeyEvent) + 'static,
    {
        self.callbacks_mut().key_pressed = Some(Box::new(f))
    }

    #[inline]
    fn register_key_released<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl, &KeyEvent) + 'static,
    {
        self.callbacks_mut().key_released = Some(Box::new(f))
    }

    #[inline]
    fn register_window_minimized<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl) + 'static,
    {
        self.callbacks_mut().window_minimized = Some(Box::new(f))
    }

    #[inline]
    fn register_window_maximized<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl) + 'static,
    {
        self.callbacks_mut().window_maximized = Some(Box::new(f))
    }

    #[inline]
    fn register_window_restored<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl) + 'static,
    {
        self.callbacks_mut().window_restored = Some(Box::new(f))
    }

    #[inline]
    fn register_visibility_changed<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl, bool) + 'static,
    {
        self.callbacks_mut().visibility_changed = Some(Box::new(f))
    }
}
impl<T: WidgetImpl> CallbacksRegister for T {}
