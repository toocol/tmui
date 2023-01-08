use super::Backend;
use crate::graphics::bitmap::Bitmap;
use skia_safe::{AlphaType, ColorSpace, ColorType, ImageInfo, Surface};

/// Backend for Raster,
/// CPU rendering, no GPU acceleration.
#[derive(Debug)]
pub struct RasterBackend {
    image_info: ImageInfo,
    bitmap: Bitmap,
}

impl Backend for RasterBackend {
    type Type = RasterBackend;

    fn new(bitmap: Bitmap) -> Self::Type {
        Self {
            image_info: ImageInfo::new(
                (bitmap.width(), bitmap.height()),
                ColorType::RGBA8888,
                AlphaType::Premul,
                ColorSpace::new_srgb(),
            ),
            bitmap,
        }
    }

    fn surface(&self) -> Surface {
        Surface::new_raster_direct(
            &self.image_info,
            self.bitmap.get_pixels(),
            self.bitmap.row_bytes(),
            None,
        )
        .expect("Create rawster skia surface failed.")
        .to_owned()
    }

    fn width(&self) -> i32 {
        self.bitmap.width()
    }

    fn height(&self) -> i32 {
        self.bitmap.height()
    }
}
