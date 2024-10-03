mod win_widget;

use tlib::{figure::OptionSize, namespace::MouseButton};
use tmui::{
    application::Application, application_window::ApplicationWindow, prelude::*, tooltip::Tooltip,
    widget::callbacks::CallbacksRegister,
};
use win_widget::{CorrMyWinWidget, CrsWinMsg};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Win Widget")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let mut hbox = HBox::new();
    hbox.set_hexpand(true);
    hbox.set_vexpand(true);

    let mut widget: Box<Widget> = Object::new(&[]);
    let mut win_widget = CorrMyWinWidget::new();

    widget.set_hexpand(true);
    widget.set_vexpand(true);
    widget.set_hscale(0.5);
    widget.set_background(Color::RED);
    widget.register_mouse_released(|_, e| {
        if e.mouse_button() == MouseButton::RightButton {
            let mut pos: Point = e.position().into();
            pos.offset(1, 0);
            Tooltip::show("Windowed tooltip content.", pos, OptionSize::none(), None);
        }
    });
    Tooltip::hide_on_window_resize(true);

    win_widget.set_hexpand(true);
    win_widget.set_vexpand(true);
    win_widget.set_hscale(0.5);
    let id = win_widget.id();

    hbox.add_child(widget);
    hbox.add_child(win_widget);

    window.child(hbox);

    window.register_run_after(move |win| {
        let w = win
            .find_id(id)
            .unwrap()
            .downcast_ref::<CorrMyWinWidget>()
            .unwrap();
        w.send_cross_win_msg(CrsWinMsg::Test(122, 290));
    })
}
