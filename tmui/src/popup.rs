use crate::{
   prelude::*,
   tlib::object::{ObjectImpl, ObjectSubclass},
   widget::WidgetImpl,
};

#[extends(Widget)]
pub struct Popup {}

impl ObjectSubclass for Popup {
   const NAME: &'static str = "Popup";
}

impl ObjectImpl for Popup {}

impl WidgetImpl for Popup {}

#[reflect_trait]
pub trait PopupImpl: WidgetImpl {}