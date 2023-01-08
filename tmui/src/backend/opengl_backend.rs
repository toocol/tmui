use super::Backend;
use crate::graphics::bitmap::Bitmap;
use skia_safe::Surface;

/// Backend for OpenGL,
/// Support cross platform GPU acceleration.
#[derive(Debug)]
#[allow(dead_code)]
pub struct OpenGLBackend {
    bitmap: Bitmap,
}

impl Backend for OpenGLBackend {
    type Type = OpenGLBackend;

    fn new(bitmap: Bitmap) -> Self::Type {
        Self { bitmap }
    }

    fn surface(&self) -> Surface {
        todo!()
    }

    fn width(&self) -> i32 {
        self.bitmap.width()
    }

    fn height(&self) -> i32 {
        self.bitmap.height()
    }
}
