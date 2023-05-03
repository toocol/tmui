pub mod box_layout;

use box_layout::BoxLayout;
use tmui::{application::Application, application_window::ApplicationWindow, prelude::*};

fn main() {
    log4rs::init_file("tmui/examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Box layout")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    window.child(BoxLayout::new())
}
