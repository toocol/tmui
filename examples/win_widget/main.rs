mod win_widget;

use tmui::{
    prelude::*,
    application::Application,
    application_window::ApplicationWindow,
};
use win_widget::MyWinWidget;

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Win Widget")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let mut hbox = HBox::new();
    hbox.set_hexpand(true);
    hbox.set_vexpand(true);

    let mut widget: Box<Widget> = Object::new(&[]);
    let mut win_widget = MyWinWidget::new();

    widget.set_hexpand(true);
    widget.set_vexpand(true);
    widget.set_hscale(0.5);
    widget.set_background(Color::RED);

    win_widget.set_hexpand(true);
    win_widget.set_vexpand(true);
    win_widget.set_hscale(0.5);

    hbox.add_child(widget);
    hbox.add_child(win_widget);

    window.child(hbox)
}