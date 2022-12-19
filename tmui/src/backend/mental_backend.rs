use skia_safe::Surface;

use super::Backend;

/// Backend for Mental,
/// Support GPU acceleration on MacOS.
#[derive(Debug)]
pub struct MentalBackend {
    width: i32,
    height: i32,
}

impl Backend for MentalBackend {
    type Type = MentalBackend;

    fn create(width: i32, height: i32) -> Self::Type {
        Self { width, height }
    }

    fn surface(&self) -> Surface {
        todo!()
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    fn set_height(&mut self, height: i32) {
        self.height = height;
    }
}