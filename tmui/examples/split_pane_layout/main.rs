mod split_pane_layout;

use split_pane_layout::SplitPaneLayout;
use tlib::Object;
use tmui::{
    application::Application,
    application_window::ApplicationWindow,
    widget::{WidgetExt, WidgetImplExt},
};

fn main() {
    log4rs::init_file("tmui/examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("window")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let mut split_pane: SplitPaneLayout = Object::new(&[]);
    split_pane.width_request(window.size().width());
    split_pane.height_request(window.size().height());

    window.child(split_pane);
}
