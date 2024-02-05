
use crate::primitive::Message;
use glutin::{
    config::{ConfigTemplateBuilder, GlConfig, Config},
    context::{ContextApi, ContextAttributesBuilder, Version, NotCurrentContext},
    display::{GetGlDisplay, GlDisplay},
};
use glutin_winit::DisplayBuilder;
use std::error::Error;
use tlib::{
    typedef::WinitWindow,
    winit::{
        event_loop::EventLoopWindowTarget, raw_window_handle::HasWindowHandle,
        window::WindowBuilder,
    },
};

pub(crate) fn bootstrap_gl_window(
    target: &EventLoopWindowTarget<Message>,
    win_builder: WindowBuilder,
) -> Result<(WinitWindow, Config, Option<NotCurrentContext>), Box<dyn Error>> {
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

    let raw_window_handle = window
        .as_ref()
        .map(|window| window.window_handle().unwrap().as_raw());

    // XXX The display could be obtained from any object created by it, so we can
    // query it from the config.
    let gl_display = gl_config.display();

    // TODO: Waiting `glutin` to bump `raw-window-handle` version to `0.6.0`
    // https://github.com/rust-windowing/glutin/pull/1582#issuecomment-1896218932

    // The context creation part. It can be created before surface and that's how
    // it's expected in multithreaded + multiwindow operation mode, since you
    // can send NotCurrentContext, but not Surface.
    // let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    // Since glutin by default tries to create OpenGL core context, which may not be
    // present we should try gles.
    // let fallback_context_attributes = ContextAttributesBuilder::new()
    //     .with_context_api(ContextApi::Gles(None))
    //     .build(raw_window_handle);

    // There are also some old devices that support neither modern OpenGL nor GLES.
    // To support these we can try and create a 2.1 context.
    // let legacy_context_attributes = ContextAttributesBuilder::new()
    //     .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
    //     .build(raw_window_handle);

    // let mut not_current_gl_context = Some(unsafe {
    //     gl_display
    //         .create_context(&gl_config, &context_attributes)
    //         .unwrap_or_else(|_| {
    //             gl_display
    //                 .create_context(&gl_config, &fallback_context_attributes)
    //                 .unwrap_or_else(|_| {
    //                     gl_display
    //                         .create_context(&gl_config, &legacy_context_attributes)
    //                         .expect("failed to create context")
    //                 })
    //         })
    // });

    todo!()
}