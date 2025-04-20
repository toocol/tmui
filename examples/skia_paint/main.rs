mod skia_paint;
use skia_paint::SkiaPaint;
use tmui::{
    application::Application, application_window::ApplicationWindow, prelude::TrAlloc,
    widget::ChildOp,
};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("win32 widnow")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let w = SkiaPaint::new_alloc();
    window.child(w)
}
