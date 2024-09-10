mod test_widget;

use test_widget::TestWidget;
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
        .title("")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    window.child(TestWidget::new())
}