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

pub trait Popupable {
    /// Add the popup to the widget.
    /// 
    /// Only one popup can exist at the same time.
    fn add_popup(&mut self, popup: Box<dyn PopupImpl>);

    /// Change the popup's visibility to true, show the popup.
    fn show_popup(&mut self);

    /// Change the popup's visibility to false, show the popup.
    fn hide_popup(&mut self);
}