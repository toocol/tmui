use crate::graphics::bitmap::Bitmap;
use skia_safe::Surface;
use std::cell::RefCell;

pub mod opengl_backend;
pub mod raster_backend;

#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum BackendType {
    #[default]
    Raster,
    OpenGL,
}

/// Renderer backend, provide skia Surface
pub trait Backend: Sized + 'static {
    type Type: Backend;

    fn new(bitmap: Bitmap) -> Self::Type;

    fn wrap(self) -> Box<dyn BackendWrapper> {
        Box::new(RefCell::new(self))
    }

    fn surface(&self) -> Surface;

    fn width(&self) -> i32;

    fn height(&self) -> i32;
}

pub trait BackendWrapper {
    fn surface(&self) -> skia_safe::Surface;

    fn width(&self) -> i32;

    fn height(&self) -> i32;
}

impl<T: Backend> BackendWrapper for RefCell<T> {
    fn surface(&self) -> Surface {
        self.borrow().surface()
    }

    fn width(&self) -> i32 {
        self.borrow().width()
    }

    fn height(&self) -> i32 {
        self.borrow().height()
    }
}
