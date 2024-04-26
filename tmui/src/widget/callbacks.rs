use super::WidgetImpl;

pub type HoverInFn = Box<dyn Fn(&mut dyn WidgetImpl)>;
pub type HoverOutFn = Box<dyn Fn(&mut dyn WidgetImpl)>;

#[derive(Default)]
pub struct Callbacks {
    pub(crate) hover_in: Option<HoverInFn>,
    pub(crate) hover_out: Option<HoverOutFn>,
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
}
impl<T: WidgetImpl> CallbacksRegister for T {}
