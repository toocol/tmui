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

impl ObjectImpl for Popup {
    #[inline]
    fn type_register(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<Self, ReflectPopupImpl>();
        type_registry.register::<Self, ReflectOverlaid>();
    }
}

impl WidgetImpl for Popup {}

impl PopupExt for Popup {
    #[inline]
    fn as_widget_impl(&self) -> &dyn WidgetImpl {
        self
    }

    #[inline]
    fn as_widget_impl_mut(&mut self) -> &mut dyn WidgetImpl {
        self
    }
}

impl PopupImpl for Popup {}

impl Overlaid for Popup {}

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
    #[inline]
    fn add_popup(&mut self, mut popup: Box<dyn PopupImpl>) {
        ApplicationWindow::initialize_dynamic_component(popup.as_widget_impl_mut());

        self.set_popup(popup);
    }

    /// Change the popup's visibility to true, show the popup.
    ///
    /// basic_point: `global coordinate` point needed.
    fn show_popup(&mut self, basic_point: Point) {
        let rect = self.rect();
        if let Some(popup) = self.get_popup_mut() {
            if popup.visible() {
                return;
            }

            let pos = popup.calculate_position(rect, basic_point);
            popup.set_fixed_x(pos.x());
            popup.set_fixed_y(pos.y());

            let window = ApplicationWindow::window_of(popup.window_id());
            if window.initialized() {
                window.layout_change(popup.as_widget_impl_mut());
            }
            
            popup.show();
        }
    }

    /// Change the popup's visibility to false, hide the popup.
    #[inline]
    fn hide_popup(&mut self) {
        if let Some(popup) = self.get_popup_mut() {
            popup.hide()
        }
    }

    /// Set the popup to widget.
    ///
    /// use [`add_popup()`](Popupable::add_popup) instead, do not use this function directly.
    fn set_popup(&mut self, popup: Box<dyn PopupImpl>);

    /// Get the refrence of popup.
    fn get_popup_ref(&self) -> Option<&dyn PopupImpl>;

    /// Get the mutable refrence of popup.
    fn get_popup_mut(&mut self) -> Option<&mut dyn PopupImpl>;

    /// Cast to dyn WidgetImpl
    fn as_widget(&self) -> &dyn WidgetImpl;

    /// Cast to mutable dyn WidgetImpl
    fn as_widget_mut(&mut self) -> &mut dyn WidgetImpl;
}
