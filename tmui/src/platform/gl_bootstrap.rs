use crate::primitive::Message;
use glutin::{
    config::{Config, ConfigTemplateBuilder, GlConfig},
    context::{
        ContextApi, ContextAttributesBuilder, NotCurrentContext, NotCurrentGlContext,
        PossiblyCurrentContext, PossiblyCurrentGlContext, Version,
    },
    display::{GetGlDisplay, GlDisplay},
    surface::{GlSurface, Surface, WindowSurface},
};
use glutin_winit::{DisplayBuilder, GlWindow};
use log::info;
use raw_window_handle::HasRawWindowHandle;
use std::{
    error::Error,
    ffi::{CStr, CString},
    sync::{Arc, Once},
};
use tipc::parking_lot::RwLock;
use tlib::{
    typedef::WinitWindow,
    winit::{event_loop::EventLoopWindowTarget, window::WindowBuilder},
};

pub(crate) fn bootstrap_gl_window(
    target: &EventLoopWindowTarget<Message>,
    win_builder: WindowBuilder,
) -> Result<(WinitWindow, Arc<GlState>), Box<dyn Error>> {
    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(cfg!(cgl_backend));

    let display_builder = DisplayBuilder::new().with_window_builder(Some(win_builder));
    let (window, gl_config) = display_builder.build(target, template, |configs| {
        // Find the config with the maximum number of samples, so our triangle will
        // be smooth.
        configs
            .reduce(|accum, config| {
                let transparency_check = config.supports_transparency().unwrap_or(false)
                    & !accum.supports_transparency().unwrap_or(false);

                if transparency_check || config.num_samples() > accum.num_samples() {
                    config
                } else {
                    accum
                }
            })
            .unwrap()
    })?;
    let window = window.expect("gl_bootstrap create window failed.");

    let raw_window_handle = Some(window.raw_window_handle());

    // XXX The display could be obtained from any object created by it, so we can
    // query it from the config.
    let gl_display = gl_config.display();

    // The context creation part. It can be created before surface and that's how
    // it's expected in multithreaded + multiwindow operation mode, since you
    // can send NotCurrentContext, but not Surface.
    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    // Since glutin by default tries to create OpenGL core context, which may not be
    // present we should try gles.
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);

    // There are also some old devices that support neither modern OpenGL nor GLES.
    // To support these we can try and create a 2.1 context.
    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
        .build(raw_window_handle);

    let not_current_gl_context = unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_display
                    .create_context(&gl_config, &fallback_context_attributes)
                    .unwrap_or_else(|_| {
                        gl_display
                            .create_context(&gl_config, &legacy_context_attributes)
                            .expect("failed to create context")
                    })
            })
    };

    // Create window surface.
    let attrs = window.build_surface_attributes(Default::default());
    let gl_surface = unsafe {
        gl_display
            .create_window_surface(&gl_config, &attrs)
            .unwrap()
    };

    Ok((
        window,
        Arc::new(GlState(
            gl_config,
            RwLock::new(GlCtx::new(not_current_gl_context)),
            gl_surface,
        )),
    ))
}

pub(crate) struct GlState(Config, RwLock<GlCtx>, Surface<WindowSurface>);
static ONCE: Once = Once::new();

impl GlState {
    #[inline]
    pub(crate) fn config(&self) -> &Config {
        &self.0
    }

    #[inline]
    pub(crate) fn make_current(&self) {
        self.1.write().make_current(&self.2)
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn make_not_current(&self) {
        self.1.write().make_not_current()
    }

    #[inline]
    pub(crate) fn gl_load(&self) {
        ONCE.call_once(|| {
            gl::load_with(|symbol| {
                let symbol = CString::new(symbol).unwrap();
                self.config()
                    .display()
                    .get_proc_address(symbol.as_c_str())
                    .cast()
            });

            if let Some(renderer) = get_gl_string(gl::RENDERER) {
                info!("Running on {}", renderer.to_string_lossy());
            }
            if let Some(version) = get_gl_string(gl::VERSION) {
                info!("OpenGL Version {}", version.to_string_lossy());
            }

            if let Some(shaders_version) = get_gl_string(gl::SHADING_LANGUAGE_VERSION) {
                info!("Shaders version on {}", shaders_version.to_string_lossy());
            }
        })
    }

    #[inline]
    pub(crate) fn swap_buffers(&self) {
        if let Some(ctx) = self.1.read().possibly_current_ctx() {
            self.2.swap_buffers(ctx).expect("gl swap buffers failed.")
        }
    }
}

pub(crate) fn get_gl_string(variant: gl::types::GLenum) -> Option<&'static CStr> {
    unsafe {
        let s = gl::GetString(variant);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
}

struct GlCtx {
    not_current_ctx: Option<NotCurrentContext>,
    possibly_current_ctx: Option<PossiblyCurrentContext>,
}
impl GlCtx {
    #[inline]
    fn new(not_current_ctx: NotCurrentContext) -> Self {
        Self {
            not_current_ctx: Some(not_current_ctx),
            possibly_current_ctx: None,
        }
    }

    #[inline]
    fn possibly_current_ctx(&self) -> Option<&PossiblyCurrentContext> {
        self.possibly_current_ctx.as_ref()
    }

    fn make_current(&mut self, gl_surface: &Surface<WindowSurface>) {
        if let Some(not_current_ctx) = self.not_current_ctx.take() {
            self.possibly_current_ctx = Some(
                not_current_ctx
                    .make_current(gl_surface)
                    .expect("Make current context failed."),
            );
        }
    }

    #[allow(dead_code)]
    fn make_not_current(&mut self) {
        if let Some(possibly_current_ctx) = self.possibly_current_ctx.take() {
            self.not_current_ctx = Some(
                possibly_current_ctx 
                    .make_not_current()
                    .expect("Make current context failed."),
            );
        }
    }
}
