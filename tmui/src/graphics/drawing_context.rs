use crate::{primitive::frame::Frame, skia_safe::Canvas};

/// DrawingContext contains Board reference which contains Skia surface.
/// And has basic Point, Path, Paint of Skia renderering.
/// Elements call function in DrawingContext to renderering.
pub struct DrawingContext<'a> {
    canvas: &'a Canvas,
    frame: Frame,
}

impl<'a> DrawingContext<'a> {
    #[inline]
    pub fn new(canvas: &'a Canvas, frame: Frame) -> Self {
        Self { canvas, frame }
    }

    #[inline]
    pub fn canvas(&self) -> &Canvas {
        self.canvas
    }

    #[inline]
    pub fn frame(&self) -> Frame {
        self.frame
    }
}
