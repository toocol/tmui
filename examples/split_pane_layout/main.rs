mod split_pane_layout;

use split_pane_layout::SplitPaneLayout;
use tmui::{
    application::Application, application_window::ApplicationWindow, prelude::TrAlloc,
    widget::ChildOp,
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
    let split_pane = SplitPaneLayout::new_alloc();

    window.child(split_pane);
}
