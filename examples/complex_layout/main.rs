mod asset;
mod ctx_menu;
mod layouts;

use layouts::View;
use tmui::{application::Application, application_window::ApplicationWindow, prelude::*};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app: Application<(), ()> = Application::builder()
        .width(1280)
        .height(800)
        .title("Complex Layout")
        .transparent(true)
        .defer_display(true)
        .decoration(false)
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    window.set_border_radius(10.);
    window.child(View::new());
}
