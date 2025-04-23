pub mod border_with_child;

use border_with_child::BorderWithChild;
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
    let mut vbox = VBox::new();

    let mut hbox = HBox::new();
    let mut widget1 = Widget::new_alloc();
    let mut widget2 = Widget::new_alloc();

    widget1.set_background(Color::CYAN);
    widget1.width_request(400);
    widget1.height_request(400);
    widget1.set_border_radius(10.);
    widget1.set_borders(2., 4., 6., 8.);
    widget1.set_border_top_color(Color::BLACK);
    widget1.set_border_right_color(Color::YELLOW);
    widget1.set_border_bottom_color(Color::BLUE);
    widget1.set_border_left_color(Color::GREEN);

    widget2.set_background(Color::PURPLE);
    widget2.width_request(400);
    widget2.height_request(400);
    widget2.set_border_radius(20.);
    widget2.set_borders(2., 2., 2., 2.);
    widget2.set_border_top_color(Color::BLACK);
    widget2.set_border_right_color(Color::YELLOW);
    widget2.set_border_bottom_color(Color::BLUE);
    widget2.set_border_left_color(Color::GREEN);

    hbox.add_child(widget1);
    hbox.add_child(widget2);
    hbox.set_halign(Align::Center);
    hbox.set_valign(Align::Center);
    hbox.width_request(800);
    hbox.height_request(400);
    hbox.set_background(Color::RED);

    vbox.set_halign(Align::Center);
    vbox.set_valign(Align::Center);
    vbox.add_child(hbox);
    vbox.set_content_halign(Align::Center);
    vbox.add_child(BorderWithChild::new_alloc());

    window.child(vbox);
}
