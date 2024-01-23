mod non_strict_clip_widget;

use non_strict_clip_widget::NonStrictClipWidget;
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
        .title("Non strict clip")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    window.child(NonStrictClipWidget::new())
}