use gl::types::GLint;
use glutin::{
    config::{Config, GlConfig},
    display::{GetGlDisplay, GlDisplay},
};
use std::{ffi::CString, sync::Arc};
use tipc::parking_lot::RwLock;
use tlib::{
    global::SemanticExt,
    skia_safe::{
        gpu::{
            backend_render_targets::make_gl, gl::{Format, FramebufferInfo}, surfaces::wrap_backend_render_target, DirectContext, Protected, SurfaceOrigin
        },
        ColorType, ImageInfo,
    },
};

use super::{create_image_info, Backend, BackendType};
use crate::{primitive::bitmap::Bitmap, skia_safe::Surface};

/// Backend for OpenGL,
/// Support cross platform GPU acceleration.
#[derive(Debug)]
pub struct OpenGLBackend {
    fb_info: FramebufferInfo,
    image_info: ImageInfo,
    surface: Surface,
    context: DirectContext,
    num_samples: usize,
    stencil_size: usize,
}

impl OpenGLBackend {
    pub(crate) fn new(bitmap: Arc<RwLock<Bitmap>>, config: &Config) -> Box<Self> {
        // Load interface and create the GrDirectContext:
        let interface = tlib::skia_safe::gpu::gl::Interface::new_load_with(|name| {
            if name == "eglGetCurrentDisplay" {
                return std::ptr::null();
            }
            config
                .display()
                .get_proc_address(CString::new(name).unwrap().as_c_str())
        })
        .expect("Could not create interface");
        let mut context =
            DirectContext::new_gl(interface, None).expect("Create `GrDirectContext` failed");

        // Create frame buffer info:
        let mut fboid: GLint = 0;
        unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };
        let fb_info = FramebufferInfo {
            fboid: fboid.try_into().unwrap(),
            format: Format::RGBA8.into(),
            protected: Protected::No,
        };

        let num_samples = config.num_samples() as usize;
        let stencil_size = config.stencil_size() as usize;

        let guard = bitmap.read();
        // Create Skia Surface:
        let surface = create_gl_surface(
            &mut context,
            fb_info,
            num_samples,
            stencil_size,
            guard.width() as i32,
            guard.height() as i32,
        );

        let image_info = create_image_info((guard.width() as i32, guard.height() as i32));

        Self {
            fb_info,
            image_info,
            surface,
            context,
            num_samples,
            stencil_size,
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

        // Create Skia Surface
        let mut new_surface = create_gl_surface(
            &mut self.context,
            self.fb_info,
            self.num_samples,
            self.stencil_size,
            dimensitions.0,
            dimensitions.1,
        );

        let snapshot = self.surface.image_snapshot();

        // use std::io::Write;
        // use std::sync::atomic::AtomicUsize;
        // static COUNTER: AtomicUsize = AtomicUsize::new(0);
        // let data = snapshot
        //     .encode_to_data(tlib::skia_safe::EncodedImageFormat::PNG)
        //     .unwrap();
        // tlib::async_do!(move {
        //     let mut file = std::fs::File::create(format!(
        //         "snapshot-{}.png",
        //         COUNTER.fetch_add(1, std::sync::atomic::Ordering::Release)
        //     ))
        //     .unwrap();
        //     file.write_all(data.as_bytes()).unwrap();
        //     ()
        // });

        new_surface.canvas().draw_image(snapshot, (0, 0), None);

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

fn create_gl_surface(
    context: &mut DirectContext,
    fb_info: FramebufferInfo,
    num_samples: usize,
    stencil_size: usize,
    width: i32,
    height: i32,
) -> Surface {
    let backend_render_target =
        make_gl((width, height), Some(num_samples), stencil_size, fb_info);

    wrap_backend_render_target(
        context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        None,
    )
    .unwrap()
}
