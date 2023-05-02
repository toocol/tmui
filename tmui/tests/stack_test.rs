use tmui::{application::Application, application_window::ApplicationWindow, prelude::*, label::Label, stack::Stack, container::ContainerImplExt};

#[test]
fn main() {
    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Stack Demo")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let mut label = Label::new(Some("Hello World"));
    label.set_background(Color::RED);
    label.set_halign(Align::Center);
    label.set_valign(Align::Center);
    label.set_text_halign(Align::Center);
    label.set_text_valign(Align::Center);

    let mut stack = Stack::new();
    stack.set_background(Color::BLUE);
    stack.set_halign(Align::Center);
    stack.set_valign(Align::Center);
    stack.add_child(label);
    window.child(stack);
}
