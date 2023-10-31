use super::Backend;
use crate::skia_safe::{AlphaType, ColorSpace, ColorType, ImageInfo, Surface};

/// Backend for Raster,
/// CPU rendering, no GPU acceleration.
#[derive(Debug)]
pub struct RasterBackend {
    image_info: ImageInfo,
    surface: Surface,
}

impl RasterBackend {
    pub fn new(width: i32, height: i32) -> Box<Self> {
        #[cfg(windows_platform)]
        let color_type = ColorType::BGRA8888;
        #[cfg(not(windows_platform))]
        let color_type = ColorType::RGBA8888;

        let image_info = ImageInfo::new(
            (width, height),
            color_type,
            AlphaType::Premul,
            ColorSpace::new_srgb(),
        );

        let surface = Surface::new_raster_n32_premul((width, height)).unwrap();
        // let surface = Surface::new_raster_direct(
        //     &image_info,
        //     buffer.get_pixels(),
        //     buffer.row_bytes(),
        //     None,
        // )
        // .expect("Create rawster skia surface failed.")
        // .to_owned();

        Box::new(Self {
            image_info: image_info,
            surface: surface,
        })
    }
}

impl Backend for RasterBackend {
    fn resize(&mut self, width: i32, height: i32) {
        #[cfg(windows_platform)]
        let color_type = ColorType::BGRA8888;
        #[cfg(not(windows_platform))]
        let color_type = ColorType::RGBA8888;

        self.image_info = ImageInfo::new(
            (width, height),
            color_type,
            AlphaType::Premul,
            ColorSpace::new_srgb(),
        );

        self.surface = self.surface.new_surface_with_dimensions((width, height)).unwrap();
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
