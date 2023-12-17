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

pub trait PopupExt {
    fn as_widget_impl(&self) -> &dyn WidgetImpl;

    fn as_widget_impl_mut(&mut self) -> &mut dyn WidgetImpl;
}

#[reflect_trait]
pub trait PopupImpl: WidgetImpl + PopupExt {}

#[reflect_trait]
pub trait Popupable: WidgetImpl {
    /// Add the popup to the widget.
    /// 
    /// Only one popup can exist at the same time.
    fn add_popup(&mut self, popup: Box<dyn PopupImpl>);

    /// Change the popup's visibility to true, show the popup.
    fn show_popup(&mut self);

    /// Change the popup's visibility to false, show the popup.
    fn hide_popup(&mut self);

    fn get_popup_ref(&self) -> Option<&dyn PopupImpl>;

    fn get_popup_mut(&mut self) -> Option<&mut dyn PopupImpl>;
}