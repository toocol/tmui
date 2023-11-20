use std::sync::Arc;
use tipc::RwLock;

use super::Backend;
use crate::{
    primitive::bitmap::Bitmap,
    skia_safe::{AlphaType, ColorSpace, ColorType, ImageInfo, Surface},
};

/// Backend for Raster,
/// CPU rendering, no GPU acceleration.
#[derive(Debug)]
pub struct RasterBackend {
    image_info: ImageInfo,
    surface: Surface,
}

impl RasterBackend {
    pub(crate) fn new(bitmap: Arc<RwLock<Bitmap>>) -> Box<Self> {
        #[cfg(windows_platform)]
        let color_type = ColorType::BGRA8888;
        #[cfg(not(windows_platform))]
        let color_type = ColorType::RGBA8888;

        let mut guard = bitmap.write();

        let image_info = ImageInfo::new(
            (guard.width() as i32, guard.height() as i32),
            color_type,
            AlphaType::Premul,
            ColorSpace::new_srgb(),
        );

        // let surface = Surface::new_raster_n32_premul((width, height)).unwrap();

        let row_bytes = guard.row_bytes();
        let surface = Surface::new_raster_direct(
            &image_info,
            guard.get_pixels_mut().0,
            row_bytes,
            None,
        )
        .unwrap()
        .to_owned();

        Box::new(Self {
            image_info: image_info,
            surface: surface,
        })
    }
}

impl Backend for RasterBackend {
    fn resize(&mut self, bitmap: Arc<RwLock<Bitmap>>) {
        #[cfg(windows_platform)]
        let color_type = ColorType::BGRA8888;
        #[cfg(not(windows_platform))]
        let color_type = ColorType::RGBA8888;

        let mut guard = bitmap.write();

        self.image_info = ImageInfo::new(
            (guard.width() as i32, guard.height() as i32),
            color_type,
            AlphaType::Premul,
            ColorSpace::new_srgb(),
        );

        // let mut new_surface = self
        //     .surface
        //     .new_surface_with_dimensions((width, height))
        //     .unwrap();

        let row_bytes = guard.row_bytes();
        let (pixels, _) = guard.get_pixels_mut();
        let mut new_surface = Surface::new_raster_direct(
            &self.image_info,
            pixels,
            row_bytes,
            None,
        )
        .unwrap()
        .to_owned();

        new_surface
            .canvas()
            .draw_image(self.surface.image_snapshot(), (0, 0), None);

        guard.release_retention();

        self.surface = new_surface;
    }

    #[inline]
    fn surface(&self) -> Surface {
        self.surface.clone()
    }

    #[inline]
    fn image_info(&self) -> &ImageInfo {
        &self.image_info
    }
}
