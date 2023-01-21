mod test_widget;

use test_widget::TestWidget;
use tmui::{application::Application, application_window::ApplicationWindow, widget::{WidgetImplExt, WidgetExt}};

fn main() {
    log4rs::init_file("tmui/examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("win32 widnow")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &ApplicationWindow) {
    let test_widget = TestWidget::new();
    test_widget.height_request(200);
    test_widget.width_request(200);
    window.child(test_widget)
}
