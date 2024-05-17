use tmui::{
    button::Button,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{callbacks::CallbacksRegister, WidgetFinder, WidgetImpl},
};

use crate::{password_bundle::PasswordBundle, text_bundle::TextBundle};

#[extends(Widget, Layout(VBox))]
#[derive(Childrenable)]
pub struct InputDialog {
    #[children]
    #[derivative(Default(value = "TextBundle::new(\"User Name: \")"))]
    username: Box<TextBundle>,

    #[children]
    #[derivative(Default(value = "PasswordBundle::new(\"Password: \")"))]
    password: Box<PasswordBundle>,

    #[children]
    #[derivative(Default(value = "Button::new(Some(\"Submit\"))"))]
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
        self.set_spacing(5);

        let username_id = self.username.id();
        let password_id = self.password.id();

        self.password.set_spacing(8);

        self.submit.set_margin_top(15);
        self.submit.width_request(50);
        self.submit.height_request(20);
        self.submit.set_halign(Align::Center);
        self.submit.callback_mouse_released(move |widget, _| {
            // let username = widget
            //     .find_siblings::<TextBundle>()
            //     .first()
            //     .unwrap()
            //     .value();
            // let password = widget
            //     .find_siblings::<PasswordBundle>()
            //     .first()
            //     .unwrap()
            //     .value();
            let username = widget.find_id::<TextBundle>(username_id).unwrap().value();
            let password = widget
                .find_id::<PasswordBundle>(password_id)
                .unwrap()
                .value();
            widget.window().call_response(move |window| {
                println!(
                    "Main window `{}` call response, username={}, password={}",
                    window.name(),
                    username,
                    password
                )
            });
            widget.window().close();
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
