use tmui::{
    application::Application, application_window::ApplicationWindow, label::Label, pane::Pane,
    prelude::*,
};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Pane Widget")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let mut pane = Pane::new();
    pane.set_vexpand(true);
    pane.set_hexpand(true);

    let mut left = Label::new(Some("Left child."));
    left.set_background(Color::GREY_MEDIUM);
    left.set_hexpand(true);
    left.set_vexpand(true);
    left.set_size_hint(SizeHint::new().with_min_width(200).with_min_height(200));

    let mut right = Label::new(Some("Right child."));
    right.set_background(Color::MAGENTA);
    right.set_hexpand(true);
    right.set_vexpand(true);

    pane.add_child(left);
    pane.add_child(right);

    window.child(pane)
}
