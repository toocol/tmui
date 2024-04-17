use std::sync::Arc;
use tipc::{
    parking_lot::RwLock,
    parking_lot::{lock_api::RwLockWriteGuard, RawRwLock},
};
use tlib::{ptr_ref, skia_safe::surfaces::wrap_pixels};

use super::{create_image_info, Backend, BackendType};
use crate::{
    primitive::bitmap::Bitmap,
    skia_safe::{ImageInfo, Surface},
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
        let mut guard = bitmap.write();
        let _guard = ptr_ref!(&guard as *const RwLockWriteGuard<'_, RawRwLock, Bitmap>).ipc_write();

        let image_info = create_image_info((guard.width() as i32, guard.height() as i32));

        let row_bytes = guard.row_bytes();
        let surface = wrap_pixels(&image_info, guard.get_pixels_mut(), row_bytes, None)
            .unwrap()
            .to_owned();

        Box::new(Self {
            image_info,
            surface,
        })
    }
}

impl Backend for RasterBackend {
    #[inline]
    fn ty(&self) -> BackendType {
        BackendType::OpenGL
    }

    fn resize(&mut self, bitmap: Arc<RwLock<Bitmap>>) {
        let mut guard = bitmap.write();
        let _guard = ptr_ref!(&guard as *const RwLockWriteGuard<'_, RawRwLock, Bitmap>).ipc_write();

        let row_bytes = guard.row_bytes();
        let dimensitions = (guard.width() as i32, guard.height() as i32);

        self.image_info = create_image_info(dimensitions);

        let mut new_surface =
            wrap_pixels(&self.image_info, guard.get_pixels_mut(), row_bytes, None)
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
