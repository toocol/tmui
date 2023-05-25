pub mod opengl_backend;
pub mod raster_backend;

use crate::skia_safe::Surface;

#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum BackendType {
    #[default]
    Raster,
    OpenGL,
}

/// Renderer backend, provide skia Surface
pub trait Backend: 'static {
    fn surface(&self) -> (Surface, Surface);

    fn width(&self) -> u32;

    fn height(&self) -> u32;
}
