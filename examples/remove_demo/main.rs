pub mod child_widget;
pub mod remove_widget;
pub mod split_pane_layout;
pub mod stack_widget;

use remove_widget::RemoveWidget;
use tmui::{application::Application, application_window::ApplicationWindow, prelude::*};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Remove Demo")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    window.child(RemoveWidget::new());
}
