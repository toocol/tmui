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
    label: Tr<Label>,

    #[children]
    password: Tr<Password>,
}

impl ObjectSubclass for PasswordBundle {
    const NAME: &'static str = "PasswordBundle";
}

impl ObjectImpl for PasswordBundle {}

impl WidgetImpl for PasswordBundle {}

impl PasswordBundle {
    #[inline]
    pub fn new(label: &str) -> Tr<Self> {
        let mut tb = Self::new_alloc();
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
