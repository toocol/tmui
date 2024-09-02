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
    let mut vbox = VBox::new();

    let mut label_1 = Label::new(Some("DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#$%^&*()/\\"));
    label_1.set_font(Font::with_families(&["Courier New"]));

    let mut label_2 = Label::new(Some(r#"virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2""#));
    label_2.set_auto_wrap(true);
    label_2.set_font(Font::with_families(&["Courier New"]));
    label_2.width_request(230);

    vbox.set_vexpand(true);
    vbox.set_hexpand(true);
    vbox.set_spacing(10);
    vbox.add_child(label_1);
    vbox.add_child(label_2);

    window.child(vbox)
}
