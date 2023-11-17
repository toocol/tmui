use std::sync::{Arc, RwLock};

use tlib::global::SemanticExt;

use super::Backend;
use crate::{skia_safe::Surface, primitive::bitmap::Bitmap};

/// Backend for OpenGL,
/// Support cross platform GPU acceleration.
#[derive(Debug)]
#[allow(dead_code)]
pub struct OpenGLBackend {
}

impl OpenGLBackend {
    pub fn new(_bitmap: Arc<RwLock<Bitmap>>) -> Box<Self> {
        Self {}.boxed()
    }
}

impl Backend for OpenGLBackend {
    fn resize(&mut self, _bitmap: Arc<RwLock<Bitmap>>) {
        todo!()
    }

    fn surface(&self) -> Surface {
        todo!()
    }

    fn image_info(&self) -> &tlib::skia_safe::ImageInfo {
        todo!()
    }
}
