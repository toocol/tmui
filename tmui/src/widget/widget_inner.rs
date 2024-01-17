use super::WidgetImpl;

pub(crate) trait WidgetInnerExt {
    fn set_fixed_width_ration(&mut self, ration: f32);

    fn set_fixed_height_ration(&mut self, ration: f32);
}

macro_rules! widget_inner_ext_impl {
    () => {
        #[inline]
        fn set_fixed_width_ration(&mut self, ration: f32) {
            self.widget_model_mut().fixed_width_ration = ration;
        }

        #[inline]
        fn set_fixed_height_ration(&mut self, ration: f32) {
            self.widget_model_mut().fixed_height_ration = ration;
        }
    };
}

impl<T: WidgetImpl> WidgetInnerExt for T {
    widget_inner_ext_impl!();
}
impl WidgetInnerExt for dyn WidgetImpl {
    widget_inner_ext_impl!();
}