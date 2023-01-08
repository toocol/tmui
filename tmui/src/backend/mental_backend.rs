use super::Backend;
use crate::graphics::bitmap::Bitmap;
use skia_safe::Surface;

/// Backend for Mental,
/// Support GPU acceleration on MacOS.
#[derive(Debug)]
#[allow(dead_code)]
pub struct MentalBackend {
    bitmap: Bitmap,
}

impl Backend for MentalBackend {
    type Type = MentalBackend;

    fn new(bitmap: Bitmap) -> Self::Type {
        Self { bitmap }
    }

    fn surface(&self) -> Surface {
        todo!()
    }

    fn width(&self) -> i32 {
        self.bitmap.width()
    }

    fn height(&self) -> i32 {
        self.bitmap.height()
    }
}
