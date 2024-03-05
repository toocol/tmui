use glutin::config::{Config, GlConfig};
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

use super::{create_image_info, Backend, BackendType};
use crate::{primitive::bitmap::Bitmap, skia_safe::Surface};

/// Backend for OpenGL,
/// Support cross platform GPU acceleration.
#[derive(Debug)]
#[allow(dead_code)]
pub struct OpenGLBackend {
    image_info: ImageInfo,
    surface: Surface,
    context: DirectContext,
    num_samples: usize,
    stencil_size: usize,
}

impl OpenGLBackend {
    pub(crate) fn new(bitmap: Arc<RwLock<Bitmap>>, config: &Config) -> Box<Self> {
        let mut context = DirectContext::new_gl(None, None).expect("Create `DirectContext` failed");
        // Setting Skia BackendRenderTarget
        let fb_info = FramebufferInfo {
            fboid: 0, 
            format: Format::RGBA8.into(),
        };

        let guard = bitmap.read();
        let backend_render_target = BackendRenderTarget::new_gl(
            (guard.width() as i32, guard.height() as i32),
            Some(config.num_samples() as usize),
            config.stencil_size() as usize,
            fb_info,
        );

        // Create Skia Surface
        let surface = Surface::from_backend_render_target(
            &mut context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        )
        .unwrap();

        let image_info = create_image_info((guard.width() as i32, guard.height() as i32));

        Self {
            image_info,
            surface,
            context,
            num_samples: config.num_samples() as usize,
            stencil_size: config.stencil_size() as usize,
        }
        .boxed()
    }
}

impl Backend for OpenGLBackend {
    #[inline]
    fn ty(&self) -> BackendType {
        BackendType::OpenGL
    }

    fn resize(&mut self, bitmap: Arc<RwLock<Bitmap>>) {
        let guard = bitmap.write();

        let dimensitions = (guard.width() as i32, guard.height() as i32);

        self.image_info = create_image_info(dimensitions);

        let fb_info = FramebufferInfo {
            fboid: 0, 
            format: Format::RGBA8.into(),
        };

        let backend_render_target = BackendRenderTarget::new_gl(
            (guard.width() as i32, guard.height() as i32),
            Some(self.num_samples),
            self.stencil_size as usize,
            fb_info,
        );

        // Create Skia Surface
        let mut new_surface = Surface::from_backend_render_target(
            &mut self.context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        )
        .unwrap();

        new_surface
            .canvas()
            .draw_image(self.surface.image_snapshot(), (0, 0), None);

        self.surface = new_surface;
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
