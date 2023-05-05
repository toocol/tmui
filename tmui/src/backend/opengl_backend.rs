use super::Backend;
use crate::graphics::bitmap::Bitmap;
use skia_safe::Surface;

/// Backend for OpenGL,
/// Support cross platform GPU acceleration.
#[derive(Debug)]
#[allow(dead_code)]
pub struct OpenGLBackend {
    front_buffer: Bitmap,
    back_buffer: Bitmap,
}

impl Backend for OpenGLBackend {
    type Type = OpenGLBackend;

    fn new(front: Bitmap, back: Bitmap) -> Self::Type {
        Self { front_buffer: front, back_buffer: back }
    }

    fn surface(&self) -> (Surface, Surface) {
        todo!()
    }

    fn width(&self) -> u32 {
        self.front_buffer.width()
    }

    fn height(&self) -> u32 {
        self.front_buffer.height()
    }
}
