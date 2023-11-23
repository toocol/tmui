use std::time::Instant;
use test_widget::TestWidget;
use tmui::{
    application::Application, application_window::ApplicationWindow, label::Label,
    platform::PlatformType, prelude::*, 
};

mod test_widget;

pub const IPC_NAME: &'static str = "shmem_ipc59";

#[derive(Debug, Clone, Copy)]
enum UserEvent {
    TestEvent(i32, Instant),
    _E,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum Request {
    Request,
    Response(i32),
}

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::<UserEvent, Request>::shared_builder(IPC_NAME)
        .platform(PlatformType::Ipc)
        .shared_widget_id("shmem_widget")
        .build();

    app.connect_activate(build_ui);

    app.connect_user_events_receive(user_events_receive);

    app.run();
}

fn user_events_receive(_window: &mut ApplicationWindow, evt: UserEvent) {
    println!("Receive user event: {:?}", evt)
}

fn build_ui(window: &mut ApplicationWindow) {
    let mut label = Label::new(Some("Hello World"));
    label.set_background(Color::CYAN);
    label.set_halign(Align::Center);
    label.set_valign(Align::Center);
    label.set_hexpand(true);
    label.set_vexpand(true);
    label.set_hscale(0.8);
    label.set_vscale(0.8);
    label.set_content_halign(Align::End);
    label.set_content_valign(Align::End);
    label.set_size(30);
    label.set_margin_left(50);
    label.set_margin_top(50);
    label.set_paddings(15, 0, 15, 0);

    let mut test_widget = TestWidget::new();
    test_widget.set_background(Color::RED);
    test_widget.set_hexpand(true);
    test_widget.set_vexpand(true);
    test_widget.set_hscale(0.5);
    test_widget.set_vscale(0.5);
    test_widget.set_halign(Align::Center);
    test_widget.set_valign(Align::Center);

    test_widget.child(label);
    window.child(test_widget)
}
