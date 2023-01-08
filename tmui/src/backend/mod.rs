use crate::platform::PlatformContextWrapper;
use skia_safe::Surface;
use std::cell::RefCell;

pub mod mental_backend;
pub mod opengl_backend;
pub mod raster_backend;

#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum BackendType {
    #[default]
    Raster,
    OpenGL,
    Mental,
}

/// Renderer backend, provide skia Surface
pub trait Backend: Sized + 'static {
    type Type: Backend;

    fn new() -> Self::Type;

    fn wrap(self) -> Box<dyn BackendWrapper> {
        Box::new(RefCell::new(self))
    }

    fn surface(&self, platform: &Box<dyn PlatformContextWrapper>) -> Surface;
}

pub trait BackendWrapper {
    fn surface(&self, platform: &Box<dyn PlatformContextWrapper>) -> skia_safe::Surface;
}

impl<T: Backend> BackendWrapper for RefCell<T> {
    fn surface(&self, platform: &Box<dyn PlatformContextWrapper>) -> Surface {
        self.borrow().surface(platform)
    }
}
