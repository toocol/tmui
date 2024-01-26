pub mod opengl_backend;
pub mod raster_backend;

use std::sync::Arc;

use tipc::parking_lot::RwLock;
use tlib::skia_safe::{ImageInfo, ColorType, AlphaType, ColorSpace};

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
    fn ty(&self) -> BackendType;

    fn resize(&mut self, bitmap: Arc<RwLock<Bitmap>>);

    fn surface(&self) -> Surface;

    fn image_info(&self) -> &ImageInfo;
}

#[inline]
pub(crate) fn create_image_info(size: (i32, i32)) -> ImageInfo {
    #[cfg(windows_platform)]
    let color_type = ColorType::BGRA8888;
    #[cfg(not(windows_platform))]
    let color_type = ColorType::RGBA8888;

    ImageInfo::new(
        size,
        color_type,
        AlphaType::Premul,
        ColorSpace::new_srgb(),
    )
}