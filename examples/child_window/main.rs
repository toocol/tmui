use tlib::figure::OptionSize;
use tmui::{
    application::Application,
    application_window::ApplicationWindow,
    graphics::styles::Styles,
    label::Label,
    prelude::*,
    tooltip::Tooltip,
    widget::callbacks::CallbacksRegister,
    window::{win_builder::WindowBuilder, win_config::WindowConfig},
};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Child Window")
        .transparent(true)
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    window.set_transparency(180);
    window.register_run_after(|win| {
        win.create_window(
            WindowBuilder::new()
                .config(
                    WindowConfig::builder()
                        .width(600)
                        .height(400)
                        .decoration(false)
                        .build(),
                )
                .inner_window(true)
                .on_activate(|win| {
                    win.set_transparency(180);
                    win.set_background(Color::GREY_LIGHT);
                    win.child(Label::new(Some("Label on child window.")))
                }),
        )
    });

    window.register_mouse_released(|_, evt| {
        let mut pos: Point = evt.position().into();
        pos.offset(-100, -100);
        Tooltip::show(
            "Test tooltip overlap",
            pos,
            OptionSize::none(),
            Some(Styles::default().with_border(Border::default().with_border_radius(6.))),
        );
    });
}
