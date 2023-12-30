mod tree_view_holder;

use tree_view_holder::TreeViewHolder;
use tmui::{
    application::Application,
    application_window::ApplicationWindow,
    prelude::*,
};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Tree View")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    window.child(TreeViewHolder::new())
}
