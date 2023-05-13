use super::Backend;
use crate::graphics::bitmap::Bitmap;
use skia_safe::{AlphaType, ColorSpace, ColorType, ImageInfo, Surface};

/// Backend for Raster,
/// CPU rendering, no GPU acceleration.
#[derive(Debug)]
pub struct RasterBackend {
    image_info: ImageInfo,
    front_buffer: Bitmap,
    back_buffer: Bitmap,
}

impl RasterBackend {
    pub fn new(front: Bitmap, back: Bitmap) -> Box<Self> {
        #[cfg(target_os = "windows")]
        let color_type = ColorType::BGRA8888;
        #[cfg(target_os = "macos")]
        let color_type = ColorType::RGBA8888;

        Box::new(Self {
            image_info: ImageInfo::new(
                (front.width() as i32, front.height() as i32),
                color_type,
                AlphaType::Premul,
                ColorSpace::new_srgb(),
            ),
            front_buffer: front,
            back_buffer: back,
        })
    }
}

impl Backend for RasterBackend {
    fn surface(&self) -> (Surface, Surface) {
        let front_surface = Surface::new_raster_direct(
            &self.image_info,
            self.front_buffer.get_pixels(),
            self.front_buffer.row_bytes(),
            None,
        )
        .expect("Create rawster skia surface failed.")
        .to_owned();

        let back_surface = Surface::new_raster_direct(
            &self.image_info,
            self.back_buffer.get_pixels(),
            self.back_buffer.row_bytes(),
            None,
        )
        .expect("Create rawster skia surface failed.")
        .to_owned();

        (front_surface, back_surface)
    }

    fn width(&self) -> u32 {
        self.front_buffer.width()
    }

    fn height(&self) -> u32 {
        self.front_buffer.height()
    }
}
