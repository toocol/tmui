mod ctrl_button;
mod icons;
mod font_icons;
mod svg_icons;

use icons::Icons;
use tmui::{
    prelude::*,
    application::Application,
    application_window::ApplicationWindow,
};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Icon List")
        .transparent(true)
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    window.child(Icons::new())
}