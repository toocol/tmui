use crate::{
    graphics::{
        board::Board, drawing_context::DrawingContext, element::ElementImpl, figure::Size,
        painter::Painter,
    },
    prelude::*,
};
use lazy_static::lazy_static;
use log::debug;
use skia_safe::Font;
use std::{
    cell::RefCell,
    ptr::{null_mut, NonNull},
    sync::{
        atomic::{AtomicPtr, Ordering},
        Once,
    },
};
use tlib::{
    namespace::Align,
    object::{IsSubclassable, ObjectImpl, ObjectSubclass},
};

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
    font: Font,
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

        debug!("`Widget` construct")
    }

    fn on_property_set(&self, name: &str, value: &Value) {
        debug!("`Widget` on set property, name = {}", name);

        match name {
            "width" => {
                let width = value.get::<i32>();
                self.set_fixed_width(width)
            }
            "height" => {
                let height = value.get::<i32>();
                self.set_fixed_height(height)
            }
            "invalidate" => {
                let invalidate = value.get::<bool>();
                if invalidate {
                    // Notify all the child widget to invalidate, preparing rerenderer after.
                    self.notify_invalidate();
                }
            }
            _ => {}
        }
    }
}

impl WidgetImpl for Widget {}

impl<T: WidgetImpl> ElementImpl for T {
    fn on_renderer(&mut self, cr: &DrawingContext) {
        self.paint(Painter::new(cr.canvas(), self))
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

    pub fn child_internal<P, T>(&self, parent: &P, child: T)
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

        let child = self.get_raw_child();
        Self::child_region_probe(parent.rect(), child)
    }

    #[inline]
    pub fn child_region_probe(mut parent_rect: Rect, mut child: Option<*const dyn WidgetImpl>) {
        while let Some(child_ptr) = child {
            let child_ref = unsafe { child_ptr.as_ref().unwrap() };
            let child_rect = child_ref.rect();

            let _halign = child_ref.get_property("halign");
            let _valign = child_ref.get_property("valign");
            child_ref.set_fixed_x(parent_rect.x() + child_rect.x());
            child_ref.set_fixed_y(parent_rect.y() + child_rect.y());

            parent_rect = child_rect;
            child = child_ref.get_raw_child();
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

    /// Notify all the child widget to invalidate.
    fn notify_invalidate(&self);

    /// Set alignment on the horizontal direction.
    fn set_halign(&self, halign: Align);

    /// Set alignment on the vertical direction.
    fn set_valign(&self, valign: Align);

    /// Set the font of widget.
    fn set_font(&mut self, font: Font);

    /// Get the font of widget.
    fn font(&self) -> Font;
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

    fn notify_invalidate(&self) {
        if let Some(child) = self.get_raw_child() {
            unsafe { child.as_ref().unwrap().update() }
        }
    }

    fn set_halign(&self, halign: Align) {
        self.set_property("halign", halign.to_value())
    }

    fn set_valign(&self, valign: Align) {
        self.set_property("valign", valign.to_value())
    }

    fn set_font(&mut self, font: Font) {
        self.font = font
    }

    fn font(&self) -> Font {
        let mut font = Font::default();
        font.set_force_auto_hinting(self.font.is_force_auto_hinting());
        font.set_embedded_bitmaps(self.font.is_embedded_bitmaps());
        font.set_subpixel(self.font.is_subpixel());
        font.set_linear_metrics(self.font.is_baseline_snap());
        font.set_embolden(self.font.is_embolden());
        font.set_baseline_snap(self.font.is_baseline_snap());
        font.set_edging(self.font.edging());
        font.set_hinting(self.font.hinting());
        if let Some(typeface) = self.font.typeface() {
            font.set_typeface(typeface);
        }
        font.set_size(self.font.size());
        font.set_scale_x(self.font.scale_x());
        font.set_skew_x(self.font.skew_x());
        font
    }
}

////////////////////////////////////// WidgetGenericExt //////////////////////////////////////
/// The trait provide some functions include the generic types.
pub trait WidgetGenericExt {
    fn get_parent<T: IsA<Widget> + ObjectType>(&self) -> Option<&T>;

    fn get_child<T: IsA<Widget> + ObjectType>(&self) -> Option<&T>;
}

impl<T: WidgetImpl> WidgetGenericExt for T {
    fn get_parent<R: IsA<Widget> + ObjectType>(&self) -> Option<&R> {
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

    fn get_child<R: IsA<Widget> + ObjectType>(&self) -> Option<&R> {
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
#[allow(unused_variables)]
#[allow(unused_mut)]
pub trait WidgetImpl: WidgetExt + ElementExt + ObjectOperation + ObjectType {
    /// Invoke this function when widget's size change.
    fn size_hint(&mut self, size: Size) {}

    /// Invoke this function when renderering.
    fn paint(&mut self, mut painter: Painter) {}
}

pub trait WidgetImplExt: WidgetImpl {
    fn child<T: WidgetImpl + ElementImpl + IsA<Widget>>(&self, child: T);
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

        parent_ref.update();
    }
}
