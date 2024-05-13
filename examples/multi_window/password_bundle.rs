use tmui::{
    input::password::Password,
    input::Input,
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct PasswordBundle {
    #[children]
    label: Box<Label>,

    #[children]
    password: Box<Password>,
}

impl ObjectSubclass for PasswordBundle {
    const NAME: &'static str = "PasswordBundle";
}

impl ObjectImpl for PasswordBundle {}

impl WidgetImpl for PasswordBundle {}

impl PasswordBundle {
    #[inline]
    pub fn new(label: &str) -> Box<Self> {
        let mut tb: Box<Self> = Object::new(&[]);
        tb.label.set_margin_top(3);
        tb.label.set_text(label);
        tb
    }

    #[inline]
    pub fn set_spacing(&mut self, spacing: i32) {
        self.password.set_margin_left(spacing);
    }

    #[inline]
    pub fn value(&self) -> String {
        self.password.value()
    }
}
