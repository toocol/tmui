use tlib::global::SemanticExt;

use super::Backend;
use crate::skia_safe::Surface;

/// Backend for OpenGL,
/// Support cross platform GPU acceleration.
#[derive(Debug)]
#[allow(dead_code)]
pub struct OpenGLBackend {
}

impl OpenGLBackend {
    pub fn new(_width: i32, _height: i32) -> Box<Self> {
        Self {}.boxed()
    }
}

impl Backend for OpenGLBackend {
    fn resize(&mut self, _width: i32, _height: i32) {
        todo!()
    }

    fn surface(&self) -> Surface {
        todo!()
    }

    fn image_info(&self) -> &tlib::skia_safe::ImageInfo {
        todo!()
    }
}
