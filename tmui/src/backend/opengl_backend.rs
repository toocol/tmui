use std::sync::Arc;
use tipc::parking_lot::RwLock;
use tlib::{
    global::SemanticExt,
    skia_safe::{
        gpu::{
            gl::{Format, FramebufferInfo},
            BackendRenderTarget, DirectContext, SurfaceOrigin,
        },
        ColorType, ImageInfo,
    },
};

use super::{Backend, create_image_info, BackendType};
use crate::{primitive::bitmap::Bitmap, skia_safe::Surface};

/// Backend for OpenGL,
/// Support cross platform GPU acceleration.
#[derive(Debug)]
#[allow(dead_code)]
pub struct OpenGLBackend {
    image_info: ImageInfo,
    surface: Surface,
}

impl OpenGLBackend {
    pub(crate) fn new(bitmap: Arc<RwLock<Bitmap>>) -> Box<Self> {
        let mut context = DirectContext::new_gl(None, None).expect("Create `DirectContext` failed");
        // 设置 Skia BackendRenderTarget
        let fb_info = FramebufferInfo {
            fboid: 0, // 默认的帧缓冲区
            format: Format::RGBA8.into(),
        };

        let guard = bitmap.read();
        // let pixel_format = windowed_context.get_pixel_format();
        let backend_render_target = BackendRenderTarget::new_gl(
            (guard.width() as i32, guard.height() as i32),
            None,
            0,
            fb_info,
        );

        // 创建 Skia Surface
        let surface = Surface::from_backend_render_target(
            &mut context,
            &backend_render_target,
            SurfaceOrigin::TopLeft,
            ColorType::RGBA8888,
            None,
            None,
        )
        .unwrap();


        let image_info = create_image_info((guard.width() as i32, guard.height() as i32));

        Self {
            image_info,
            surface,
        }.boxed()
    }
}

impl Backend for OpenGLBackend {
    #[inline]
    fn ty(&self) -> BackendType {
        BackendType::OpenGL
    }

    fn resize(&mut self, _bitmap: Arc<RwLock<Bitmap>>) {
        todo!()
    }

    #[inline]
    fn surface(&self) -> Surface {
        self.surface.clone()
    }

    #[inline]
    fn image_info(&self) -> &tlib::skia_safe::ImageInfo {
        &self.image_info
    }
}
