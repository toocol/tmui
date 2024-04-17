use tmui::{
    application::Application, application_window::ApplicationWindow, label::Label, prelude::*,
};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("label window")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let mut label = Label::new(Some("DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#$%^&*()/\\"));
    label.set_font(Font::with_families(&["Courier New"]));
    window.child(label)
}
