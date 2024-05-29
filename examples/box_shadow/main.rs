use tmui::{
    application::Application, application_window::ApplicationWindow, graphics::box_shadow::BoxShadow, prelude::*
};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let mut widget: Box<Widget> = Object::new(&[]);
    widget.width_request(400);
    widget.height_request(200);
    widget.set_hexpand(true);
    widget.set_vexpand(true);
    widget.set_halign(Align::Center);
    widget.set_valign(Align::Center);
    widget.set_box_shadow(BoxShadow::new(6., Color::BLACK));

    window.child(widget);
}