mod layout_widget;

use layout_widget::CustomWidget;
use tmui::{application::Application, application_window::ApplicationWindow, widget::WidgetImplExt};

fn main() {
    log4rs::init_file("tmui/examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Custom Widget")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    window.child(CustomWidget::new())
}