use super::WidgetImpl;
use tlib::events::{KeyEvent, MouseEvent};

pub type HoverInFn = Box<dyn Fn(&mut dyn WidgetImpl)>;
pub type HoverOutFn = Box<dyn Fn(&mut dyn WidgetImpl)>;
pub type MousePressedFn = Box<dyn Fn(&mut dyn WidgetImpl, &MouseEvent)>;
pub type MouseReleasedFn = Box<dyn Fn(&mut dyn WidgetImpl, &MouseEvent)>;
pub type MouseMoveFn = Box<dyn Fn(&mut dyn WidgetImpl, &MouseEvent)>;
pub type MouseWheelFn = Box<dyn Fn(&mut dyn WidgetImpl, &MouseEvent)>;
pub type KeyPressedFn = Box<dyn Fn(&mut dyn WidgetImpl, &KeyEvent)>;
pub type KeyReleasedFn = Box<dyn Fn(&mut dyn WidgetImpl, &KeyEvent)>;
pub type WindowMinimizedFn = Box<dyn Fn(&mut dyn WidgetImpl)>;
pub type WindowMaximizedFn = Box<dyn Fn(&mut dyn WidgetImpl)>;
pub type WindowRestoredFn = Box<dyn Fn(&mut dyn WidgetImpl)>;

#[derive(Default)]
pub struct Callbacks {
    pub(crate) hover_in: Option<HoverInFn>,
    pub(crate) hover_out: Option<HoverOutFn>,
    pub(crate) mouse_pressed: Option<MousePressedFn>,
    pub(crate) mouse_released: Option<MouseReleasedFn>,
    pub(crate) mouse_move: Option<MouseMoveFn>,
    pub(crate) mouse_wheel: Option<MouseMoveFn>,
    pub(crate) key_pressed: Option<KeyPressedFn>,
    pub(crate) key_released: Option<KeyReleasedFn>,
    pub(crate) window_minimized: Option<WindowMinimizedFn>,
    pub(crate) window_maximized: Option<WindowMaximizedFn>,
    pub(crate) window_restored: Option<WindowRestoredFn>,
}

pub trait CallbacksRegister: WidgetImpl {
    #[inline]
    fn callback_hover_in<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl) + 'static,
    {
        self.callbacks_mut().hover_in = Some(Box::new(f));
    }

    #[inline]
    fn callback_hover_out<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl) + 'static,
    {
        self.callbacks_mut().hover_out = Some(Box::new(f));
    }

    #[inline]
    fn callback_mouse_pressed<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl, &MouseEvent) + 'static,
    {
        self.callbacks_mut().mouse_pressed = Some(Box::new(f))
    }

    #[inline]
    fn callback_mouse_released<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl, &MouseEvent) + 'static,
    {
        self.callbacks_mut().mouse_released = Some(Box::new(f))
    }

    #[inline]
    fn callback_mouse_move<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl, &MouseEvent) + 'static,
    {
        self.callbacks_mut().mouse_move = Some(Box::new(f))
    }

    #[inline]
    fn callback_mouse_wheel<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl, &MouseEvent) + 'static,
    {
        self.callbacks_mut().mouse_wheel = Some(Box::new(f))
    }

    #[inline]
    fn callback_key_pressed<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl, &KeyEvent) + 'static,
    {
        self.callbacks_mut().key_pressed = Some(Box::new(f))
    }

    #[inline]
    fn callback_key_released<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl, &KeyEvent) + 'static,
    {
        self.callbacks_mut().key_released = Some(Box::new(f))
    }

    #[inline]
    fn callback_window_minimized<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl) + 'static,
    {
        self.callbacks_mut().window_minimized = Some(Box::new(f))
    }

    #[inline]
    fn callback_window_maximized<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl) + 'static,
    {
        self.callbacks_mut().window_maximized = Some(Box::new(f))
    }

    #[inline]
    fn callback_window_restored<F>(&mut self, f: F)
    where
        F: Fn(&mut dyn WidgetImpl) + 'static,
    {
        self.callbacks_mut().window_restored = Some(Box::new(f))
    }
}
impl<T: WidgetImpl> CallbacksRegister for T {}
