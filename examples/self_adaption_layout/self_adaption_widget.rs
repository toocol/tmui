use tmui::{
   prelude::*,
   tlib::object::{ObjectImpl, ObjectSubclass},
   widget::WidgetImpl,
};

#[extends(Widget)]
pub struct SelfAdaptionWidget {}

impl ObjectSubclass for SelfAdaptionWidget {
   const NAME: &'static str = "SelfAdaptionWidget";
}

impl ObjectImpl for SelfAdaptionWidget {}

impl WidgetImpl for SelfAdaptionWidget {}

impl SelfAdaptionWidget {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}