use tlib::{connect, run_after};
use tmui::{
    input::{
        password::Password,
        text::{Text, TextExt, TextSignals},
        Input, InputSignals,
    },
    prelude::*,
    tlib::{
        namespace::MouseButton,
        object::{ObjectImpl, ObjectSubclass},
    },
    widget::{callbacks::CallbacksRegister, WidgetImpl},
};

#[extends(Widget, Layout(VBox))]
#[derive(Childrenable)]
#[run_after]
pub struct Holder {
    #[children]
    text1: Box<Text>,

    #[children]
    text2: Box<Text>,

    #[children]
    text3: Box<Text>,

    #[children]
    password: Box<Password>,
}

impl ObjectSubclass for Holder {
    const NAME: &'static str = "Holder";
}

impl ObjectImpl for Holder {
    fn initialize(&mut self) {
        // self.text1.set_background(Color::RED);
        self.text1.width_request(400);
        self.text1.height_request(25);
        self.text1.set_margin_left(20);
        self.text1.set_margin_top(10);
        self.text1.set_hexpand(true);
        self.text1.set_value("Contents of text-1.中文".to_string());
        // self.text1.set_vexpand(true);
        connect!(self.text1, value_changed(), self, text_value_changed());
        connect!(
            self.text1,
            selection_changed(),
            self,
            text_selection_changed()
        );

        // self.text2.set_background(Color::BLUE);
        // self.text2.width_request(200);
        // self.text2.height_request(25);
        self.text2.set_margin_left(20);
        self.text2.set_margin_top(10);
        self.text2
            .set_value("Contents of disabled text-2, some text may not be seen.".to_string());
        self.text2.disable();
        // self.text2.set_vexpand(true);

        // self.text3.width_request(200);
        // self.text3.height_request(25);
        self.text3.set_margin_left(20);
        self.text3.set_margin_top(10);
        self.text3
            .set_placeholder("Placeholder of text-3/中文提示符");
        // self.text3.set_vexpand(true);

        self.password.set_margin_left(20);
        self.password.set_margin_top(10);
        self.password.callback_mouse_released(|w, evt| {
            if evt.mouse_button() == MouseButton::RightButton {
                let pwd = w.downcast_mut::<Password>().unwrap();
                let v = !pwd.is_password_visible();
                pwd.set_password_visible(v);
            }
        });

        self.set_vexpand(true);
        self.set_spacing(30);
        self.set_hexpand(true);
        // self.set_background(Color::GREY_LIGHT);
    }
}

impl WidgetImpl for Holder {
    fn run_after(&mut self) {
        self.text1.set_focus(true);
    }
}

impl Holder {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }

    #[inline]
    pub fn text_value_changed(&self) {}

    #[inline]
    pub fn text_selection_changed(&self) {}
}
