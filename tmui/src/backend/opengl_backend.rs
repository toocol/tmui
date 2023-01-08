use super::Backend;
use crate::platform::PlatformContextWrapper;
use skia_safe::Surface;

/// Backend for OpenGL,
/// Support cross platform GPU acceleration.
#[derive(Debug)]
pub struct OpenGLBackend;

impl Backend for OpenGLBackend {
    type Type = OpenGLBackend;

    fn new() -> Self::Type {
        Self {}
    }

    fn surface(&self, _platform: &Box<dyn PlatformContextWrapper>) -> Surface {
        todo!()
    }
}
