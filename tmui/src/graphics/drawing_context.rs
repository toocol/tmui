#![allow(dead_code)]
use super::board::Board;
use crate::skia_safe::{Canvas, Surface};
use std::cell::RefMut;

/// DrawingContext contains Board reference which contains Skia surface.
/// And has basic Point, Path, Paint of Skia renderering.
/// Elements call function in DrawingContext to renderering.
pub struct DrawingContext<'a> {
    board: &'a Board,
}

impl<'a> DrawingContext<'a> {
    #[inline]
    pub fn new(board: &'a Board) -> Self {
        Self { board }
    }

    #[inline]
    pub fn surface(&self) -> RefMut<Surface> {
        self.board.surface()
    }

    #[inline]
    pub fn canvas(&self) -> RefMut<Canvas> {
        RefMut::map(self.board.surface(), |surface| {
            surface.canvas()
        })
    }
}
