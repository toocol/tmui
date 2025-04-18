mod holder;
mod popup;

use holder::Holder;
use tmui::{application::Application, application_window::ApplicationWindow, prelude::*};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Input Elements")
        .defer_display(true)
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    window.child(Holder::new());
}
