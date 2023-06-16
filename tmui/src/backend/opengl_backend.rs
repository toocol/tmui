use super::Backend;
use crate::graphics::bitmap::Bitmap;
use crate::skia_safe::Surface;

/// Backend for OpenGL,
/// Support cross platform GPU acceleration.
#[derive(Debug)]
#[allow(dead_code)]
pub struct OpenGLBackend {
    buffer: Bitmap,
}

impl OpenGLBackend {
    pub fn new(buffer: Bitmap) -> Box<Self> {
        Box::new(Self {
            buffer,
        })
    }
}

impl Backend for OpenGLBackend {
    fn surface(&self) -> Surface {
        todo!()
    }

    fn width(&self) -> u32 {
        self.buffer.width()
    }

    fn height(&self) -> u32 {
        self.buffer.height()
    }
}
