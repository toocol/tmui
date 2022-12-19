use skia_safe::Surface;
use std::cell::RefCell;

pub mod mental_backend;
pub mod opengl_backend;
pub mod raster_backend;

pub enum BackendType {
    Raster,
    OpenGL,
    Mental,
}

/// Renderer backend, provide skia Surface
pub trait Backend: Sized + 'static {
    type Type: Backend;

    fn create(width: i32, height: i32) -> Self::Type;

    fn surface(&self) -> Surface;

    fn wrap(self) -> Box<dyn BackendWrapper> {
        Box::new(RefCell::new(self))
    }

    fn width(&self) -> i32;

    fn height(&self) -> i32;

    fn set_width(&mut self, width: i32);

    fn set_height(&mut self, height: i32);
}

pub trait BackendWrapper {
    fn surface(&self) -> skia_safe::Surface;

    fn width(&self) -> i32;

    fn height(&self) -> i32;

    fn set_width(&self, width: i32);

    fn set_height(&self, height: i32);
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

    fn set_width(&self, width: i32) {
        self.borrow_mut().set_width(width);
    }

    fn set_height(&self, height: i32) {
        self.borrow_mut().set_height(height);
    }
}

#[cfg(test)]
mod tests {
    use super::{raster_backend::RasterBackend, *};

    #[test]
    fn test_raster_backend() {
        let backend_wrap = RasterBackend::create(100, 100).wrap();
        assert_eq!(100, backend_wrap.width());
        assert_eq!(100, backend_wrap.height());
    }
}
