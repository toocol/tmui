use crate::{
    graphics::styles::Styles, prelude::*, tlib::object::{ObjectImpl, ObjectSubclass}, widget::WidgetImpl
};

use super::{number::Number, password::Password, text::Text, InputType};

#[extends(Popup, internal = true)]
pub struct InputDialog {
    #[derivative(Default(value = "Text::new()"))]
    text: Box<Text>,
    #[derivative(Default(value = "Password::new()"))]
    password: Box<Password>,
    #[derivative(Default(value = "Number::new()"))]
    number: Box<Number>,
}

impl ObjectSubclass for InputDialog {
    const NAME: &'static str = "InputDialog";
}

impl ObjectImpl for InputDialog {
    #[inline]
    fn initialize(&mut self) {
        self.text.initialize();
        self.password.initialize();
        self.number.initialize();
    }
}

impl WidgetImpl for InputDialog {}

impl PopupImpl for InputDialog {}

impl InputDialog {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }

    #[inline]
    pub fn text(geometry: Rect, styles: Styles) -> &'static mut InputDialog {
        let input_dialog = ApplicationWindow::window().input_dialog();
        input_dialog.set_type(InputType::Text, geometry, styles);
        input_dialog
    }
}

impl InputDialog {
    pub(crate) fn setup_input(input: &mut dyn WidgetImpl, geometry: Rect, styles: Styles) {
        input.set_fixed_x(geometry.x());
        input.set_fixed_y(geometry.y());
        input.set_fixed_width(geometry.width());
        input.set_fixed_height(geometry.height());

        let fr: FRect = geometry.into();
        emit!(input.geometry_changed(), fr);
    }

    pub(crate) fn set_type(&mut self, input_type: InputType, geometry: Rect, styles: Styles) {
        let input = match input_type {
            InputType::Text => self.text.as_widget_mut(),
            InputType::Password => self.password.as_widget_mut(),
            InputType::Number => self.number.as_widget_mut(),
            _ => panic!("Unsupported input type of `InputDialog`"),
        };
        Self::setup_input(input, geometry, styles);

        let child = input.as_ptr_mut();
        unsafe { self._child_ref(child) }
    }
}