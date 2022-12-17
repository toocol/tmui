#![allow(dead_code)]
use skia_safe::{Path, Surface, Paint, PaintStyle};
use std::cell::{RefCell, RefMut};

use super::{board::Board, figure::Point};

/// DrawingContext contains Board reference which contains Skia surface. 
/// And has basic Point, Path, Paint of Skia renderering.
pub struct DrawingContext<'a> {
    board: &'a Board,
    point: Point,
    path: RefCell<Path>,
    paint: RefCell<Paint>,
}

impl<'a> DrawingContext<'a> {
    pub fn new(board: &'a Board, point: Point) -> Self {
        Self {
            board,
            point,
            path: RefCell::new(Path::new()),
            paint: RefCell::new(Paint::default()),
        }
    }

    #[inline]
    pub fn surface(&self) -> RefMut<Surface> {
        self.board.surface.borrow_mut()
    }

    #[inline]
    pub fn save(&self) {
        self.surface().canvas().save();
    }

    #[inline]
    pub fn translate(&self, dx: f32, dy: f32) {
        self.surface().canvas().translate((dx, dy));
    }

    #[inline]
    pub fn scale(&self, sx: f32, sy: f32) {
        self.surface().canvas().scale((sx, sy));
    }

    #[inline]
    pub fn move_to(&self, x: f32, y: f32) {
        self.begin_path();
        self.path.borrow_mut().move_to((x, y));
    }

    #[inline]
    pub fn line_to(&self, x: f32, y: f32) {
        self.path.borrow_mut().line_to((x, y));
    }

    #[inline]
    pub fn quad_to(&self, cpx: f32, cpy: f32, x: f32, y: f32) {
        self.path.borrow_mut().quad_to((cpx, cpy), (x, y));
    }

    #[inline]
    pub fn bezier_curve_to(&self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
        self.path.borrow_mut().cubic_to((cp1x, cp1y), (cp2x, cp2y), (x, y));
    }

    #[inline]
    pub fn close_path(&self) {
        self.path.borrow_mut().close();
    }

    #[inline]
    pub fn begin_path(&self) {
        let new_path = Path::new();
        self.surface().canvas().draw_path(self.path.borrow().as_ref(), self.paint.borrow().as_ref());
        *self.path.borrow_mut() = new_path;
    }

    #[inline]
    pub fn stroke(&self) {
        self.paint.borrow_mut().set_style(PaintStyle::Stroke);
        self.surface().canvas().draw_path(self.path.borrow().as_ref(), self.paint.borrow().as_ref());
    }

    #[inline]
    pub fn fill(&self) {
        self.paint.borrow_mut().set_style(PaintStyle::Fill);
        self.surface().canvas().draw_path(self.path.borrow().as_ref(), self.paint.borrow().as_ref());
    }

    #[inline]
    pub fn set_line_width(&self, width: f32) {
        self.paint.borrow_mut().set_stroke_width(width);
    }
}
