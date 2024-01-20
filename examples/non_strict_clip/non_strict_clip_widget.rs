use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
pub struct NonStrictClipWidget {}

impl ObjectSubclass for NonStrictClipWidget {
    const NAME: &'static str = "NonStrictClipWidget";
}

impl ObjectImpl for NonStrictClipWidget {
    fn initialize(&mut self) {
        self.set_strict_clip_widget(false);
    }
}

impl WidgetImpl for NonStrictClipWidget {}

impl NonStrictClipWidget {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}