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

impl OpenGLBackend {
    pub fn new(front: Bitmap, back: Bitmap) -> Box<Self> {
        Box::new(Self {
            front_buffer: front,
            back_buffer: back,
        })
    }
}

impl Backend for OpenGLBackend {
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
