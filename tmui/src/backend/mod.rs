pub mod opengl_backend;
pub mod raster_backend;

use std::sync::Arc;

use tipc::RwLock;
use tlib::skia_safe::ImageInfo;

use crate::{skia_safe::Surface, primitive::bitmap::Bitmap};

#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum BackendType {
    #[default]
    Raster,
    OpenGL,
}

/// Renderer backend, provide skia Surface
pub(crate) trait Backend: 'static {
    fn resize(&mut self, bitmap: Arc<RwLock<Bitmap>>);

    fn surface(&self) -> Surface;

    fn image_info(&self) -> &ImageInfo;
}
