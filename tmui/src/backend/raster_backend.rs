use super::Backend;
use crate::platform::PlatformContextWrapper;
use skia_safe::Surface;

/// Backend for Raster,
/// CPU rendering, no GPU acceleration.
#[derive(Debug)]
pub struct RasterBackend;

impl Backend for RasterBackend {
    type Type = RasterBackend;

    fn new() -> Self::Type {
        Self {}
    }

    fn surface(&self, platform: &Box<dyn PlatformContextWrapper>) -> Surface {
        let bitmap = platform.context_bitmap();
        Surface::new_raster_direct(
            platform.image_info(),
            bitmap.get_pixels(),
            bitmap.raw_bytes(),
            None,
        ).expect("Create rawster skia surface failed.").to_owned()
    }
}
