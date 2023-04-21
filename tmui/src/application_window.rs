use crate::{
    graphics::{
        board::Board,
        figure::{Color, Size},
        painter::Painter,
    },
    prelude::*,
    widget::WidgetImpl,
};
use lazy_static::lazy_static;
use log::debug;
use skia_safe::Font;
use std::{
    ptr::null_mut,
    sync::{
        atomic::{AtomicPtr, Ordering},
        Once,
    },
};
use tlib::object::{ObjectImpl, ObjectSubclass};

static INIT: Once = Once::new();
lazy_static! {
    static ref BOARD: AtomicPtr<Board> = AtomicPtr::new(null_mut());
}

/// Store the [`Board`] as raw ptr.
pub fn store_board(board: &mut Board) {
    INIT.call_once(move || {
        BOARD.store(board as *mut Board, Ordering::SeqCst);
    })
}

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

    fn initialize(&mut self) {
        child_initialize(self, self.get_raw_child_mut())
    }
}

impl WidgetImpl for ApplicationWindow {
    fn paint(&mut self, mut _painter: Painter) {}
}

impl ApplicationWindow {
    pub fn new(width: i32, height: i32) -> ApplicationWindow {
        Object::new(&[("width", &width), ("height", &height)])
    }

    pub fn size_probe(&mut self) {
        let child = self.get_raw_child_mut();
        if let Some(child) = child {
            child_width_probe(self.size(), self.size(), child);
        }
    }

    pub fn position_probe(&mut self) {
        let child = self.get_raw_child_mut();
        child_position_probe(self, child)
    }
}

#[inline]
fn child_initialize(mut parent: *mut dyn WidgetImpl, mut child: Option<*mut dyn WidgetImpl>) {
    let board = unsafe { BOARD.load(Ordering::SeqCst).as_mut().unwrap() };
    let type_registry = TypeRegistry::instance();
    while let Some(child_ptr) = child {
        let child_ref = unsafe { child_ptr.as_mut().unwrap() };
        child_ref.set_parent(parent);
        parent = child_ptr;

        board.add_element(child_ref.as_element());

        child_ref.initialize();
        child_ref.inner_type_register(type_registry);
        child_ref.type_register(type_registry);
        child = child_ref.get_raw_child_mut();
    }
}

fn child_width_probe(window_size: Size, parent_size: Size, widget: *mut dyn WidgetImpl) -> Size {
    let widget_ref = unsafe { widget.as_mut().unwrap() };
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
        let image_rect = widget_ref.image_rect();
        return Size::new(image_rect.width(), image_rect.height());
    } else {
        let size = widget_ref.size();
        let child_size = child_width_probe(window_size, size, widget_ref.get_raw_child_mut().unwrap());
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
    mut child: Option<*mut dyn WidgetImpl>,
) {
    while let Some(child_ptr) = child {
        let child_ref = unsafe { child_ptr.as_mut().unwrap() };
        let child_rect = child_ref.rect();
        let parent_rect = unsafe { parent.as_ref().unwrap().rect() };

        let halign = child_ref.get_property("halign").unwrap().get::<Align>();
        let valign = child_ref.get_property("valign").unwrap().get::<Align>();

        match halign {
            Align::Start => child_ref.set_fixed_x(parent_rect.x() as i32 + child_ref.margin_left()),
            Align::Center => {
                let offset =
                    (parent_rect.width() - child_ref.rect().width()) as i32 / 2 + child_ref.margin_left();
                child_ref.set_fixed_x(parent_rect.x() as i32 + offset)
            }
            Align::End => {
                let offset =
                    parent_rect.width() as i32 - child_ref.rect().width() as i32 + child_ref.margin_left();
                child_ref.set_fixed_x(parent_rect.x() as i32 + offset)
            }
        }

        match valign {
            Align::Start => {
                child_ref.set_fixed_y(parent_rect.y() as i32 + child_rect.y() as i32 + child_ref.margin_top())
            }
            Align::Center => {
                let offset =
                    (parent_rect.height() - child_ref.rect().height()) as i32 / 2 + child_ref.margin_top();
                child_ref.set_fixed_y(parent_rect.y() as i32 + offset)
            }
            Align::End => {
                let offset =
                    parent_rect.height() as i32 - child_ref.rect().height() as i32 + child_ref.margin_top();
                child_ref.set_fixed_y(parent_rect.y() as i32 + offset)
            }
        }

        parent = child_ptr;
        child = child_ref.get_raw_child_mut();
    }
}
