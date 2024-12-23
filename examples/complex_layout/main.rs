mod layouts;
mod ctx_menu;
mod asset;

use layouts::View;
use tmui::{
    prelude::*,
    application::Application,
    application_window::ApplicationWindow,
};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app: Application<(), ()> = Application::builder()
        .width(1280)
        .height(800)
        .title("Complex Layout")
        .transparent(true)
        .defer_display(true)
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    window.child(View::new());
}