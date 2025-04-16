pub mod events;
pub mod widget;

use events::{EventBus, Events};
use tmui::{
    application::Application,
    application_window::ApplicationWindow,
    button::Button,
    hbox::HBox,
    prelude::{ContainerImplExt, ObjectOperation},
    widget::{callbacks::CallbacksRegister, ChildOp},
};
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
    let w1 = WidgetEvents::new();
    let id = w1.id();
    let w2 = WidgetEvents::new();

    let mut button = Button::new(Some("Emit event"));
    button.register_mouse_released(|_, _| {
        EventBus::push(Events::Test);
    });

    let mut hbox = HBox::new();
    hbox.add_child(button);
    hbox.add_child(w1);
    hbox.add_child(w2);
    let hbox_id = hbox.id();
    window.child(hbox);

    window.register_run_after(move |win| {
        EventBus::push(Events::Test);
        win.find_id_mut(hbox_id)
            .unwrap()
            .downcast_mut::<HBox>()
            .unwrap()
            .remove_children(id);
    });
}
