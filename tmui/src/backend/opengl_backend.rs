use skia_safe::Surface;
use super::Backend;

/// Backend for OpenGL,
/// Support cross platform GPU acceleration.
#[derive(Debug)]
pub struct OpenGLBackend {
    width: i32, 
    height: i32,
}

impl Backend for OpenGLBackend {
    type Type = OpenGLBackend;

    fn create(width: i32, height: i32) -> Self::Type {
        Self { width, height }
    }

    fn surface(&self) -> Surface {
        todo!()
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    fn set_height(&mut self, height: i32) {
        self.height = height;
    }
}