use crate::{
    graphics::{board::Board, drawing_context::DrawingContext, element::ElementImpl},
    prelude::*,
};
use lazy_static::lazy_static;
use skia_safe::{Color, Font, Paint, Path};
use std::{
    cell::RefCell,
    ptr::{null_mut, NonNull},
    sync::{
        atomic::{AtomicPtr, Ordering},
        Once,
    },
};
use tlib::object::{IsSubclassable, ObjectImpl, ObjectSubclass};

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

#[extends_element]
#[derive(Default)]
pub struct Widget {
    board: RefCell<Option<NonNull<Board>>>,
    _child: Option<Box<dyn WidgetImpl>>,
}

impl ObjectSubclass for Widget {
    const NAME: &'static str = "Widget";

    type Type = Widget;
    type ParentType = Object;
}

impl Widget {
    pub fn board(&self) -> &mut Board {
        unsafe { self.board.borrow_mut().as_mut().unwrap().as_mut() }
    }
}

pub trait WidgetAcquire: WidgetImpl {}

////////////////////////////////////// WidgetExt //////////////////////////////////////
/// The extended actions of [`Widget`], impl by proc-macro [`extends_widget`] automaticly.
pub trait WidgetExt {}

impl WidgetExt for Widget {}

////////////////////////////////////// WidgetImpl //////////////////////////////////////
/// Every struct modified by proc-macro [`extends_widget`] should impl this trait manually.
pub trait WidgetImpl: WidgetExt + ElementImpl {}

pub trait WidgetImplExt {
    fn child<T: WidgetImpl + ElementImpl + IsA<Widget>>(&self, child: T) {
        let mut child = child;
        let _c = &mut child as *mut T as *mut dyn ElementImpl;
    }
}

////////////////////////////////////// Widget Implements //////////////////////////////////////
impl IsSubclassable for Widget {}

impl ObjectImpl for Widget {
    fn construct(&self) {
        self.parent_construct();
        *self.board.borrow_mut() = NonNull::new(BOARD.load(Ordering::SeqCst));

        println!("`Widget` construct")
    }
}

impl ElementImpl for Widget {
    fn on_renderer(&self, cr: &DrawingContext) {
        let mut surface = cr.surface();
        let canvas = surface.canvas();
        let mut paint = Paint::default();
        canvas.clear(Color::BLUE);
        paint.set_color(Color::BLACK);
        paint.set_anti_alias(true);
        paint.set_stroke_width(1.0);

        canvas.scale((1.2, 1.2));
        let mut path = Path::new();
        path.move_to((36., 48.));
        path.quad_to((330., 440.), (600., 180.));
        canvas.translate((10., 10.));
        paint.set_stroke_width(10.);
        paint.set_style(skia_safe::PaintStyle::Stroke);
        canvas.draw_path(&path, &paint);

        paint.reset();
        paint.set_color(Color::WHITE);
        let mut font = Font::default();
        font.set_size(20.);
        canvas.draw_str("Hello world", (0, 30), &font, &paint);
    }
}

#[cfg(test)]
mod tests {
    use tlib::object::{ObjectImpl, ObjectSubclass};

    use crate::prelude::*;

    use super::WidgetImpl;

    #[extends_widget]
    #[derive(Default)]
    struct SubWidget {}

    impl ObjectSubclass for SubWidget {
        const NAME: &'static str = "SubWidget";

        type Type = SubWidget;
        type ParentType = Widget;
    }

    impl ObjectImpl for SubWidget {}

    impl WidgetImpl for SubWidget {}

    #[test]
    fn test_sub_widget() {
        let widget: SubWidget = Object::new(&[("width", &&120), ("height", &&80)]);
        assert_eq!(120, widget.get_property("width").unwrap().get::<i32>());
        assert_eq!(80, widget.get_property("height").unwrap().get::<i32>());
    }
}
