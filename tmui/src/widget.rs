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
    parent: RefCell<Option<*const dyn WidgetImpl>>,
    child: RefCell<Option<Box<dyn WidgetImpl>>>,
}

////////////////////////////////////// Widget Implements //////////////////////////////////////
impl ObjectSubclass for Widget {
    const NAME: &'static str = "Widget";

    type Type = Widget;
    type ParentType = Object;
}

impl IsSubclassable for Widget {}

impl ObjectImpl for Widget {
    fn construct(&self) {
        self.parent_construct();
        *self.board.borrow_mut() = NonNull::new(BOARD.load(Ordering::SeqCst));

        println!("`Widget` construct")
    }

    fn on_property_set(&self, name: &str, value: &Value) {
        println!("`Widget` on set property");

        match name {
            "width" => {
                let width = value.get::<i32>();
                self.set_fixed_width(width)
            }
            "height" => {
                let height = value.get::<i32>();
                self.set_fixed_height(height)
            }
            _ => {}
        }
    }
}

impl WidgetImpl for Widget {}

impl<T: WidgetImpl> ElementImpl for T {
    fn on_renderer(&self, cr: &DrawingContext) {
        self.paint(cr)
    }
}

impl Widget {
    fn board(&self) -> Option<&mut Board> {
        unsafe {
            match self.board.borrow_mut().as_mut() {
                Some(board) => Some(board.as_mut()),
                None => None,
            }
        }
    }
}

pub trait WidgetAcquire: WidgetImpl {}

////////////////////////////////////// WidgetExt //////////////////////////////////////
/// The extended actions of [`Widget`], impl by proc-macro [`extends_widget`] automaticly.
pub trait WidgetExt {
    /// ## Do not invoke this function directly.
    fn set_parent(&self, parent: *const dyn WidgetImpl);

    /// Get the raw pointer of parent.
    /// Please use `get_parent()` function in [`WidgetGenericExt`]
    fn get_raw_child(&self) -> Option<*const dyn WidgetImpl>;

    /// Get the raw pointer of parent.
    /// Please use `get_parent()` function in [`WidgetGenericExt`]
    fn get_raw_parent(&self) -> Option<*const dyn WidgetImpl>;

    /// Request the widget's maximum width.
    fn width_request(&self, width: i32);

    /// Request the widget's maximum width.
    fn height_request(&self, width: i32);
}

impl WidgetExt for Widget {
    fn set_parent(&self, parent: *const dyn WidgetImpl) {
        *self.parent.borrow_mut() = Some(parent)
    }

    fn get_raw_child(&self) -> Option<*const dyn WidgetImpl> {
        match self.child.borrow().as_ref() {
            Some(child) => Some(child.as_ref() as *const dyn WidgetImpl),
            None => None,
        }
    }

    fn get_raw_parent(&self) -> Option<*const dyn WidgetImpl> {
        match self.parent.borrow().as_ref() {
            Some(parent) => Some(*parent),
            None => None,
        }
    }

    fn width_request(&self, width: i32) {
        self.set_property("width", width.to_value());
    }

    fn height_request(&self, height: i32) {
        self.set_property("height", height.to_value());
    }
}

////////////////////////////////////// WidgetGenericExt //////////////////////////////////////
/// The trait provide some functions include the generic types.
pub trait WidgetGenericExt {
    fn get_parent<T: IsA<Widget> + StaticType + ObjectType>(&self) -> Option<&T>;

    fn get_child<T: IsA<Widget> + StaticType + ObjectType>(&self) -> Option<&T>;
}
impl<T: WidgetImpl> WidgetGenericExt for T {
    fn get_parent<R: IsA<Widget> + StaticType + ObjectType>(&self) -> Option<&R> {
        let raw_parent = self.get_raw_parent();
        match raw_parent {
            Some(parent) => unsafe {
                if parent.as_ref().is_none() {
                    return None;
                }
                if parent
                    .as_ref()
                    .unwrap()
                    .object_type()
                    .is_a(R::static_type())
                {
                    (parent as *const R).as_ref()
                } else {
                    None
                }
            },
            None => None,
        }
    }

    fn get_child<R: IsA<Widget> + StaticType + ObjectType>(&self) -> Option<&R> {
        let raw_child = self.get_raw_child();
        match raw_child {
            Some(child) => unsafe {
                if child.as_ref().is_none() {
                    return None;
                }
                if child.as_ref().unwrap().object_type().is_a(R::static_type()) {
                    (child as *const R).as_ref()
                } else {
                    None
                }
            },
            None => None,
        }
    }
}

////////////////////////////////////// WidgetImpl //////////////////////////////////////
/// Every struct modified by proc-macro [`extends_widget`] should impl this trait manually.
/// WidgetImpl's `paint()` function Will be proxy executated by ElementImpl `on_renderer` method .
pub trait WidgetImpl: WidgetExt + ElementExt + ObjectOperation + ObjectType {
    fn paint(&self, cr: &DrawingContext) {
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

pub trait WidgetImplExt: WidgetImpl {
    fn child<T: WidgetImpl + ElementImpl + IsA<Widget>>(&self, child: T);
}

pub trait WidgetImplInternal {
    fn child_internal<P, T>(&self, parent: &P, child: T)
    where
        P: WidgetImpl + ElementImpl + IsA<Widget>,
        T: WidgetImpl + ElementImpl + IsA<Widget>;
}

impl WidgetImplInternal for Widget {
    fn child_internal<P, T>(&self, parent: &P, child: T)
    where
        P: WidgetImpl + ElementImpl + IsA<Widget>,
        T: WidgetImpl + ElementImpl + IsA<Widget>,
    {
        let mut child = Box::new(child);

        child.set_parent(parent as *const dyn WidgetImpl);

        if let Some(board) = self.board() {
            board.add_element(child.as_mut() as *mut dyn ElementImpl);
        }

        *self.child.borrow_mut() = Some(child);
    }
}

#[cfg(test)]
mod tests {
    use super::WidgetImpl;
    use crate::{prelude::*, widget::WidgetGenericExt};
    use tlib::object::{ObjectImpl, ObjectSubclass};

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

    #[extends_widget]
    #[derive(Default)]
    struct ChildWidget {}

    impl ObjectSubclass for ChildWidget {
        const NAME: &'static str = "ChildWidget";

        type Type = SubWidget;
        type ParentType = Widget;
    }

    impl ObjectImpl for ChildWidget {}

    impl WidgetImpl for ChildWidget {}

    #[test]
    fn test_sub_widget() {
        let widget: SubWidget = Object::new(&[("width", &&120), ("height", &&80)]);
        assert_eq!(120, widget.get_property("width").unwrap().get::<i32>());
        assert_eq!(80, widget.get_property("height").unwrap().get::<i32>());

        let child: ChildWidget = Object::new(&[("width", &&120), ("height", &&80)]);
        let child_id = child.id();

        widget.child(child);

        let child_ref = widget.get_child::<ChildWidget>().unwrap();
        assert_eq!(child_ref.id(), child_id);
        assert_eq!(120, child_ref.get_property("width").unwrap().get::<i32>());
        assert_eq!(80, child_ref.get_property("height").unwrap().get::<i32>());

        let parent_ref = child_ref.get_parent::<SubWidget>().unwrap();
        assert_eq!(parent_ref.id(), widget.id());
        assert_eq!(120, parent_ref.get_property("width").unwrap().get::<i32>());
        assert_eq!(80, parent_ref.get_property("height").unwrap().get::<i32>());

        parent_ref.set_property("length", 100.to_value());
        assert_eq!(100, widget.get_property("length").unwrap().get::<i32>());

        let rect = parent_ref.rect();
        assert_eq!(120, rect.width());
        assert_eq!(80, rect.height());
    }
}
