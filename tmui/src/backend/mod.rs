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

    fn new(front: Bitmap, back: Bitmap) -> Self::Type;

    fn wrap(self) -> Box<dyn BackendWrapper> {
        Box::new(RefCell::new(self))
    }

    fn surface(&self) -> (Surface, Surface);

    fn width(&self) -> u32;

    fn height(&self) -> u32;
}

pub trait BackendWrapper {
    fn surface(&self) -> (Surface, Surface);

    fn width(&self) -> u32;

    fn height(&self) -> u32;
}

impl<T: Backend> BackendWrapper for RefCell<T> {
    fn surface(&self) -> (Surface, Surface) {
        self.borrow().surface()
    }

    fn width(&self) -> u32 {
        self.borrow().width()
    }

    fn height(&self) -> u32 {
        self.borrow().height()
    }
}
