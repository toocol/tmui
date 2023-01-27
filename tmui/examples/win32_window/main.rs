mod test_widget;

use test_widget::TestWidget;
use tmui::{application::Application, application_window::ApplicationWindow, widget::{WidgetImplExt, WidgetExt}, label::Label, prelude::{Color, Align}};

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
    let mut label = Label::new(Some("Hello World"));
    label.set_background(Color::BLUE);
    label.set_halign(Align::Center);
    label.set_valign(Align::Center);
    label.set_size(30);
    label.set_margin_left(50);
    label.set_margin_top(50);
    label.set_paddings(15, 0, 15, 10);

    let mut test_widget = TestWidget::new();
    test_widget.set_background(Color::RED);
    test_widget.height_request(300);
    test_widget.width_request(300);
    test_widget.set_halign(Align::Center);
    test_widget.set_valign(Align::Center);

    test_widget.child(label);
    window.child(test_widget)
}
