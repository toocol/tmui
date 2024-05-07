use tmui::{application::Application, application_window::ApplicationWindow, prelude::*};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Border")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let mut hbox = HBox::new();
    let mut widget1: Box<Widget> = Object::new(&[]);
    let mut widget2: Box<Widget> = Object::new(&[]);

    widget1.set_background(Color::BLUE);
    widget1.width_request(400);
    widget1.height_request(400);
    widget1.set_border_radius(10.);

    widget2.set_background(Color::YELLOW);
    widget2.width_request(400);
    widget2.height_request(400);
    widget2.set_border_radius(10.);

    hbox.add_child(widget1);
    hbox.add_child(widget2);
    hbox.set_halign(Align::Center);
    hbox.set_valign(Align::Center);
    hbox.width_request(800);
    hbox.height_request(400);
    hbox.set_background(Color::RED);

    window.child(hbox);
}
