mod scroll_window;

use scroll_window::ScrollWindow;
use tlib::Object;
use tmui::{
    prelude::*,
    application::Application,
    application_window::ApplicationWindow, widget::WidgetImplExt,
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
    scroll_window.width_request(400);
    scroll_window.height_request(300);
    scroll_window.set_halign(Align::Center);
    scroll_window.set_valign(Align::Center);
    window.child(scroll_window);
}
