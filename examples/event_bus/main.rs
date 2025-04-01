pub mod events;
pub mod widget;

use events::{EventBus, Events};
use tmui::{application::Application, application_window::ApplicationWindow, widget::ChildOp};
use widget::WidgetEvents;

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Event Bus")
        .transparent(true)
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    window.child(WidgetEvents::new());

    window.register_run_after(|_| {
        EventBus::push(Events::Test);
    });
}
