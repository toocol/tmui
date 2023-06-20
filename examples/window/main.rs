mod test_widget;

use test_widget::TestWidget;
use tmui::{
    application::Application,
    application_window::ApplicationWindow,
    label::Label,
    prelude::{Align, Color, ContentAlignment},
    widget::{WidgetExt, WidgetImplExt},
};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("window")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let mut label = Label::new(Some("Hello World"));
    label.set_background(Color::CYAN);
    label.set_halign(Align::Center);
    label.set_valign(Align::Center);
    label.width_request(200);
    label.height_request(40);
    label.set_content_halign(Align::End);
    label.set_content_valign(Align::End);
    label.set_size(30);
    label.set_margin_left(50);
    label.set_margin_top(50);

    let mut test_widget = TestWidget::new();
    test_widget.set_background(Color::RED);
    test_widget.width_request(400);
    test_widget.height_request(300);
    test_widget.set_halign(Align::Center);
    test_widget.set_valign(Align::Center);

    test_widget.child(label);
    window.child(test_widget)
}