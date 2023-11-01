pub mod opengl_backend;
pub mod raster_backend;

use tlib::skia_safe::ImageInfo;

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
    fn resize(&mut self, width: i32, height: i32);

    fn surface(&self) -> Surface;

    fn image_info(&self) -> &ImageInfo;
}
