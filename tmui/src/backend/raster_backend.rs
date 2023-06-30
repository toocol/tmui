use super::Backend;
use crate::graphics::bitmap::Bitmap;
use crate::skia_safe::{AlphaType, ColorSpace, ColorType, ImageInfo, Surface};

/// Backend for Raster,
/// CPU rendering, no GPU acceleration.
#[derive(Debug)]
pub struct RasterBackend {
    image_info: ImageInfo,
    buffer: Bitmap,
}

impl RasterBackend {
    pub fn new(buffer: Bitmap) -> Box<Self> {
        #[cfg(windows_platform)]
        let color_type = ColorType::BGRA8888;
        #[cfg(not(windows_platform))]
        let color_type = ColorType::RGBA8888;

        Box::new(Self {
            image_info: ImageInfo::new(
                (buffer.width() as i32, buffer.height() as i32),
                color_type,
                AlphaType::Premul,
                ColorSpace::new_srgb(),
            ),
            buffer,
        })
    }
}

impl Backend for RasterBackend {
    fn surface(&self) -> Surface {
        let front_surface = Surface::new_raster_direct(
            &self.image_info,
            self.buffer.get_pixels(),
            self.buffer.row_bytes(),
            None,
        )
        .expect("Create rawster skia surface failed.")
        .to_owned();

        front_surface
    }

    fn width(&self) -> u32 {
        self.buffer.width()
    }

    fn height(&self) -> u32 {
        self.buffer.height()
    }
}
