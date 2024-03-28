mod scroll_window;

use scroll_window::ScrollWindow;
use tlib::Object;
use tmui::{
    application::Application, application_window::ApplicationWindow, prelude::*,
    widget::WidgetImplExt,
};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("window")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let mut scroll_window: Box<ScrollWindow> = Object::new(&[]);
    // scroll_window.width_request(400);
    // scroll_window.height_request(300);
    scroll_window.set_hexpand(true);
    scroll_window.set_vexpand(true);
    scroll_window.set_hscale(0.8);
    scroll_window.set_vscale(0.8);
    scroll_window.set_background(Color::GREY_MEDIUM);
    scroll_window.set_halign(Align::Center);
    scroll_window.set_valign(Align::Center);
    window.child(scroll_window);
}
