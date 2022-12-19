use super::Backend;
use skia_safe::Surface;

/// Backend for Raster,
/// CPU rendering, no GPU acceleration.
#[derive(Debug)]
pub struct RasterBackend {
    width: i32,
    height: i32,
}

impl Backend for RasterBackend {
    type Type = RasterBackend;

    fn create(width: i32, height: i32) -> Self::Type {
        Self { width, height }
    }

    fn surface(&self) -> Surface {
        Surface::new_raster_n32_premul((self.width, self.height))
            .expect("RasterBackend: No Skia surface available.")
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
