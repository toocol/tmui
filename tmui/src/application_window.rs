use crate::{
    graphics::{
        figure::{Color, Size},
        painter::Painter,
    },
    prelude::*,
    widget::WidgetImpl,
};
use log::debug;
use skia_safe::Font;
use tlib::object::{ObjectImpl, ObjectSubclass};

#[extends_widget]
#[derive(Default)]
pub struct ApplicationWindow {}

impl ObjectSubclass for ApplicationWindow {
    const NAME: &'static str = "ApplicationWindow";

    type Type = ApplicationWindow;

    type ParentType = Object;
}

impl ObjectImpl for ApplicationWindow {
    fn construct(&mut self) {
        self.parent_construct();
        debug!(
            "`ApplicationWindow` construct: static_type: {}",
            Self::static_type().name()
        )
    }
}

impl WidgetImpl for ApplicationWindow {
    fn paint(&mut self, mut _painter: Painter) {}
}

impl ApplicationWindow {
    pub fn new(width: i32, height: i32) -> ApplicationWindow {
        Object::new(&[("width", &width), ("height", &height)])
    }

    pub fn parent_setting(&self) {
        let child = self.get_raw_child();
        child_parent_setting(self, child)
    }

    pub fn size_probe(&self) {
        let child = self.get_raw_child();
        if let Some(child) = child {
            child_width_probe(self.size(), self.size(), child);
        }
    }

    pub fn position_probe(&self) {
        let child = self.get_raw_child();
        child_position_probe(self, child)
    }
}

fn child_parent_setting(
    mut parent: *const dyn WidgetImpl,
    mut child: Option<*const dyn WidgetImpl>,
) {
    while let Some(child_ptr) = child {
        let child_ref = unsafe { child_ptr.as_ref().unwrap() };
        child_ref.set_parent(parent);
        parent = child_ptr;
        child = child_ref.get_raw_child();
    }
}

#[inline]
fn child_width_probe(window_size: Size, parent_size: Size, widget: *const dyn WidgetImpl) -> Size {
    let widget_ref = unsafe { widget.as_ref().unwrap() };
    if widget_ref.get_raw_child().is_none() {
        let size = widget_ref.size();
        if parent_size.width() != 0 && parent_size.height() != 0 {
            if size.width() == 0 {
                widget_ref.width_request(parent_size.width());
            }
            if size.height() == 0 {
                widget_ref.height_request(parent_size.height());
            }
        } else {
            if size.width() == 0 {
                widget_ref.width_request(window_size.width());
            }
            if size.height() == 0 {
                widget_ref.height_request(window_size.height());
            }
        }
        return size;
    } else {
        let size = widget_ref.size();
        let child_size = child_width_probe(window_size, size, widget_ref.get_raw_child().unwrap());
        if size.width() == 0 {
            widget_ref.width_request(child_size.width());
        }
        if size.height() == 0 {
            widget_ref.height_request(child_size.height());
        }
        return widget_ref.size();
    }
}

#[inline]
fn child_position_probe(
    mut parent: *const dyn WidgetImpl,
    mut child: Option<*const dyn WidgetImpl>,
) {
    while let Some(child_ptr) = child {
        let child_ref = unsafe { child_ptr.as_ref().unwrap() };
        let child_rect = child_ref.rect();
        let parent_rect = unsafe { parent.as_ref().unwrap().rect() };

        let halign = child_ref.get_property("halign").unwrap().get::<Align>();
        let valign = child_ref.get_property("valign").unwrap().get::<Align>();

        match halign {
            Align::Start => child_ref.set_fixed_x(parent_rect.x() + child_ref.margin_left()),
            Align::Center => {
                let offset =
                    (parent_rect.width() - child_ref.rect().width()) / 2 + child_ref.margin_left();
                child_ref.set_fixed_x(parent_rect.x() + offset)
            }
            Align::End => {
                let offset =
                    parent_rect.width() - child_ref.rect().width() + child_ref.margin_left();
                child_ref.set_fixed_x(parent_rect.x() + offset)
            }
        }

        match valign {
            Align::Start => {
                child_ref.set_fixed_y(parent_rect.y() + child_rect.y() + child_ref.margin_top())
            }
            Align::Center => {
                let offset =
                    (parent_rect.height() - child_ref.rect().height()) / 2 + child_ref.margin_top();
                child_ref.set_fixed_y(parent_rect.y() + offset)
            }
            Align::End => {
                let offset =
                    parent_rect.height() - child_ref.rect().height() + child_ref.margin_top();
                child_ref.set_fixed_y(parent_rect.y() + offset)
            }
        }

        parent = child_ptr;
        child = child_ref.get_raw_child();
    }
}
