use tlib::winit::window::WindowLevel;
use tmui::{application::Application, application_window::ApplicationWindow, prelude::*};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    window.register_run_after(move |win| win.set_window_level(WindowLevel::AlwaysOnTop));
}
