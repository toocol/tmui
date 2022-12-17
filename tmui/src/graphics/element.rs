use std::cell::Cell;

use tlib::{prelude::*, object::{ObjectSubclass, ObjectImpl, IsSubclassable}};
use super::{drawing_context::DrawingContext, figure::Point};

/// Basic drawing element super type for basic graphics such as triangle, rectangle....
#[extends_object]
pub struct Element {
    invalidate: Cell<bool>,
    point: Cell<Point>,
}

impl IsSubclassable for Element {}

impl ObjectSubclass for Element {
    const NAME: &'static str = "Element";

    type Type = Element;

    type ParentType = Object;
}

impl ObjectImpl for Element {}

pub trait ElementExt {
    fn update(&self);

    fn force_update(&self);

    fn point(&self) -> Point;

    fn set_point(&self, point: Point);

    fn invalidate(&self) -> bool;

    fn validate(&self);
}

impl ElementExt for Element {
    /// Mark element's invalidate field to true, and element will be redrawed in next frame.
    fn update(&self) {
        self.invalidate.set(true);
    }

    /// Mark element's invalidate field to true, and element will be redrawed immediately.
    fn force_update(&self) {
        self.invalidate.set(true);
        // TODO: firgue out how to invoke `Board`'s `invalidate_visual` obligatory.
    }

    fn point(&self) -> Point {
        self.point.get()
    }

    fn set_point(&self, point: Point) {
        self.point.set(point)
    }

    fn invalidate(&self) -> bool {
        self.invalidate.get()
    }

    fn validate(&self) {
        self.invalidate.set(false)
    }
}

pub trait ElementImpl: ElementExt {
    fn on_renderer(&self, cr: &DrawingContext);
}
