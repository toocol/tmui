use tmui::{
    application::Application, application_window::ApplicationWindow,
    graphics::box_shadow::BoxShadow, prelude::*,
};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Box Shadow")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let mut widget = Widget::new_alloc();
    widget.width_request(400);
    widget.height_request(200);
    widget.set_hexpand(true);
    widget.set_vexpand(true);
    widget.set_halign(Align::Center);
    widget.set_valign(Align::Center);

    // use tmui::graphics::box_shadow::{ShadowPos, ShadowSide};
    // widget.set_box_shadow(BoxShadow::new(
    //     8.,
    //     Color::BLACK,
    //     Some(ShadowPos::Inset),
    //     Some(ShadowSide::new(&[ShadowSide::RIGHT, ShadowSide::BOTTOM])),
    //     None,
    // ));

    widget.set_box_shadow(BoxShadow::new(8., Color::BLACK, None, None, None, None));

    window.child(widget);
    window.set_background(Color::GREY_LIGHT);
}
