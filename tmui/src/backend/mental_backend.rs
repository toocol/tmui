use super::Backend;
use crate::platform::PlatformContextWrapper;
use skia_safe::Surface;

/// Backend for Mental,
/// Support GPU acceleration on MacOS.
#[derive(Debug)]
pub struct MentalBackend;

impl Backend for MentalBackend {
    type Type = MentalBackend;

    fn new() -> Self::Type {
        Self {}
    }

    fn surface(&self, _platform: &Box<dyn PlatformContextWrapper>) -> Surface {
        todo!()
    }
}
