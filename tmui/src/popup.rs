use std::ptr::NonNull;
use tlib::{events::MouseEvent, nonnull_mut, nonnull_ref};
use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
pub struct Popup {
    supervisor: WidgetHnd,
    offsets: (i32, i32),
}

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

    #[inline]
    fn set_supervisor(&mut self, widget: &mut dyn WidgetImpl) {
        self.supervisor = NonNull::new(widget);
    }

    #[inline]
    fn supervisor(&self) -> &dyn WidgetImpl {
        nonnull_ref!(self.supervisor)
    }

    #[inline]
    fn supervisor_mut(&mut self) -> &mut dyn WidgetImpl {
        nonnull_mut!(self.supervisor)
    }

    #[inline]
    fn calc_relative_position(&mut self) {
        let supervisor_rect = self.supervisor().rect();
        let rect = self.rect();
        self.offsets = (
            rect.x() - supervisor_rect.x(),
            rect.y() - supervisor_rect.y(),
        );
    }

    #[inline]
    fn layout_relative_position(&mut self) {
        let supervisor_rect = self.supervisor().rect();
        self.set_fixed_x(supervisor_rect.x() + self.offsets.0);
        self.set_fixed_y(supervisor_rect.y() + self.offsets.1);
    }
}

impl PopupImpl for Popup {}

impl Overlaid for Popup {}

pub trait PopupExt {
    fn as_widget_impl(&self) -> &dyn WidgetImpl;

    fn as_widget_impl_mut(&mut self) -> &mut dyn WidgetImpl;

    fn set_supervisor(&mut self, widget: &mut dyn WidgetImpl);

    fn supervisor(&self) -> &dyn WidgetImpl;

    fn supervisor_mut(&mut self) -> &mut dyn WidgetImpl;

    fn calc_relative_position(&mut self);

    fn layout_relative_position(&mut self);
}

#[reflect_trait]
pub trait PopupImpl: WidgetImpl + PopupExt + Overlaid {
    /// Calculate the position of the popup widget when it becomes visible.
    ///
    /// @param: `base_rect` the rectangle of base widget.<br>
    /// @param: `point` the hitting point.
    fn calculate_position(&self, base_rect: Rect, mut point: Point) -> Point {
        let size = self.size();
        point.set_y(base_rect.y() - size.height() - 3);
        point.set_x(point.x() - size.width() / 2);
        point
    }

    /// If true, the popup widget will be a modal widget.
    /// 
    /// Default value is [`false`]
    #[inline]
    fn is_modal(&self) -> bool {
        false
    }

    /// If true, popup will hide when clicking the area outside the component.
    /// 
    /// Default value is [`true`]
    #[inline]
    fn hide_on_click(&self) -> bool {
        true
    }

    /// If true, popup will move postion by mouse dragging.
    /// 
    /// Default value is [`false`]
    #[inline]
    fn move_capable(&self) -> bool {
        false
    }

    #[inline]
    fn handle_global_mouse_pressed(&mut self, evt: &MouseEvent) -> bool {
        if !self.visible() {
            return false;
        }
        let pos: Point = evt.position().into();
        if !self.rect().contains(&pos) {
            self.on_mouse_click_hide();
            self.hide();
            true
        } else {
            false
        }
    }

    #[inline]
    fn on_mouse_click_hide(&mut self) {}
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
            popup.calc_relative_position();

            popup.show();

            let window = ApplicationWindow::window_of(popup.window_id());
            window.layout_change(popup.as_widget_impl_mut());
        }
    }

    /// Change the popup's visibility to false, hide the popup.
    #[inline]
    fn hide_popup(&mut self) {
        if let Some(popup) = self.get_popup_mut() {
            popup.hide();
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
