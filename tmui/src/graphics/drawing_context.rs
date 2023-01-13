#![allow(dead_code)]
use skia_safe::{Canvas, Surface};
use std::cell::{Ref, RefMut};

use super::{board::Board, figure::Rect};

/// DrawingContext contains Board reference which contains Skia surface.
/// And has basic Point, Path, Paint of Skia renderering.
/// Elements call function in DrawingContext to renderering.
pub struct DrawingContext<'a> {
    board: &'a Board,
    rect: Ref<'a, Rect>,
}

impl<'a> DrawingContext<'a> {
    pub fn new(board: &'a Board, rect: Ref<'a, Rect>) -> Self {
        Self {
            board,
            rect,
        }
    }

    #[inline]
    pub fn surface(&self) -> RefMut<Surface> {
        self.board.front_surface.borrow_mut()
    }

    #[inline]
    pub fn canvas(&self) -> RefMut<Canvas> {
        RefMut::map(self.board.front_surface.borrow_mut(), |surface| {
            surface.canvas()
        })
    }
}
