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
pub trait PopupImpl: WidgetImpl + PopupExt {
    /// Calculate the position of the component when it becomes visible.
    /// 
    /// @param: `base_rect` the rectangle of base widget.<br>
    /// @param: `point` the hitting point.
    fn calculate_position(&self, base_rect: Rect, mut point: Point) -> Point {
        let size = self.size();
        point.set_y(base_rect.y() - size.height() - 3);
        point.set_x(point.x() - size.width() / 2);
        point
    }
}

#[reflect_trait]
pub trait Popupable: WidgetImpl {
    /// Add the popup to the widget.
    /// 
    /// Only one popup can exist at the same time.
    fn add_popup(&mut self, popup: Box<dyn PopupImpl>);

    /// Change the popup's visibility to true, show the popup.
    /// 
    /// basic_point: `global coordinate` point needed.
    fn show_popup(&mut self, basic_point: Point);

    /// Change the popup's visibility to false, hide the popup.
    fn hide_popup(&mut self);

    /// Get the refrence of popup.
    fn get_popup_ref(&self) -> Option<&dyn PopupImpl>;

    /// Get the mutable refrence of popup.
    fn get_popup_mut(&mut self) -> Option<&mut dyn PopupImpl>;
}