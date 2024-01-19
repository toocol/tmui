mod self_adaption_widget;

use self_adaption_widget::SelfAdaptionWidget;
use tmui::{application::Application, application_window::ApplicationWindow, prelude::*};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Self adaption")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let widget = SelfAdaptionWidget::new();
    window.child(widget);

    window.set_borders(2., 2., 2., 2.);
}
