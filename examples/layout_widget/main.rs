mod layout_widget;

use layout_widget::LayoutWidget;
use tmui::{application::Application, application_window::ApplicationWindow, widget::ChildOp};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Custom Widget")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    window.child(LayoutWidget::new())
}
