use crate::skia_safe::Canvas;

/// DrawingContext contains Board reference which contains Skia surface.
/// And has basic Point, Path, Paint of Skia renderering.
/// Elements call function in DrawingContext to renderering.
pub struct DrawingContext<'a> {
    canvas: &'a Canvas,
}

impl<'a> DrawingContext<'a> {
    #[inline]
    pub fn new(canvas: &'a Canvas) -> Self {
        Self { canvas }
    }

    #[inline]
    pub fn canvas(&self) -> &Canvas {
        self.canvas
    }
}
