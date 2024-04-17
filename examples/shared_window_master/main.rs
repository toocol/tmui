mod shared_widget;

use log::info;
use shared_widget::MasterSharedWidget;
use std::{sync::atomic::AtomicI32, time::Instant};
use tmui::{
    application::Application,
    application_window::ApplicationWindow,
    container::ContainerImplExt,
    label::Label,
    pane::Pane,
    widget::{widget_ext::WidgetExt, WidgetImplExt},
};

pub const IPC_NAME: &str = "shmem_ipc140";
pub static CNT: AtomicI32 = AtomicI32::new(0);

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
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
        .width(1280)
        .height(800)
        .title("Shared Window")
        .opti_track(true)
        .build();

    app.connect_activate(build_ui);

    app.connect_user_events_receive(user_events_receive);

    app.connect_request_receive(request_receive);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let mut pane = Pane::new();
    pane.set_vexpand(true);
    pane.set_hexpand(true);

    let mut label = Label::new(Some("Left label."));
    label.set_vexpand(true);
    label.width_request(300);

    pane.add_child(label);
    pane.add_child(MasterSharedWidget::new());

    window.child(pane);
}

fn user_events_receive(_: &mut ApplicationWindow, evt: UserEvent) {
    match evt {
        UserEvent::TestEvent(a, b) => {
            info!(
                "Recevie user event: {}ms",
                b.elapsed().as_micros() as f64 / 1000.
            );
            assert_eq!(a, CNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

            Application::<UserEvent, Request>::send_user_event(evt)
        }
        _ => unreachable!(),
    }
}

fn request_receive(_: &mut ApplicationWindow, _: Request) -> Option<Request> {
    Some(Request::Response(100))
}
