pub mod hbox_layout;
pub mod vbox_layout;

use tmui::{
    application::Application,
    application_window::ApplicationWindow,
    prelude::*,
};
use vbox_layout::VBoxLayout;

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Box layout")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    window.set_background(Color::from_rgb(100, 100, 100));
    window.child(VBoxLayout::new())
}
