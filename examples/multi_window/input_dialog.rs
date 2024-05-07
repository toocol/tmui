use tmui::{
    button::Button, prelude::*, tlib::object::{ObjectImpl, ObjectSubclass}, widget::{callbacks::CallbacksRegister, WidgetImpl}
};

use crate::text_bundle::TextBundle;

#[extends(Widget, Layout(VBox))]
#[derive(Childrenable)]
pub struct InputDialog {
    #[children]
    #[derivative(Default(value = "TextBundle::new(\"User Name: \")"))]
    username: Box<TextBundle>,

    #[children]
    submit: Box<Button>,
}

impl ObjectSubclass for InputDialog {
    const NAME: &'static str = "InputDialog";
}

impl ObjectImpl for InputDialog {
    fn initialize(&mut self) {
        self.set_vexpand(true);
        self.set_hexpand(true);
        self.set_homogeneous(false);
        self.set_spacing(20);

        self.submit.width_request(30);
        self.submit.height_request(20);
        self.submit.set_halign(Align::Center);
        self.submit.callback_mouse_released(|widget, _| {
            widget.window().call_response(|window| {
                println!("Main window `{}` call response.", window.name())
            });
        });
    }
}

impl WidgetImpl for InputDialog {}

impl InputDialog {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}