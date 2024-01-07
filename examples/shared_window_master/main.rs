mod shared_widget;

use log::info;
use shared_widget::MasterSharedWidget;
use std::{sync::atomic::AtomicI32, time::Instant};
use tmui::{
    application::Application, application_window::ApplicationWindow, widget::WidgetImplExt,
};

pub const IPC_NAME: &'static str = "shmem_ipc135";
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
        .build();

    app.connect_activate(build_ui);

    app.connect_user_events_receive(user_events_receive);

    app.connect_request_receive(request_receive);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    window.child(MasterSharedWidget::new());
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
