use tlib::connect;

use crate::{
    graphics::styles::Styles,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{widget_ext::FocusStrat, WidgetImpl},
};

use super::{
    number::Number,
    password::Password,
    text::{Text, TextExt},
    Input, InputType,
};

#[extends(Popup, Layout(Stack), internal = true)]
#[derive(Childrenable)]
pub struct InputDialog {
    #[children]
    text: Box<Text>,

    #[children]
    password: Box<Password>,

    #[children]
    number: Box<Number>,

    // Other properties:
    hide_on_win_changed: bool,
}

impl ObjectSubclass for InputDialog {
    const NAME: &'static str = "InputDialog";
}

impl ObjectImpl for InputDialog {
    #[inline]
    fn construct(&mut self) {
        self.parent_construct();

        let window = ApplicationWindow::window();
        self.set_supervisor(window);

        connect!(window, size_changed(), self, on_window_size_changed(Size));
    }
}

impl WidgetImpl for InputDialog {
    #[inline]
    fn on_visibility_changed(&mut self, visible: bool) {
        if visible {
            self.take_over_focus(FocusStrat::TakeOver);
        } else {
            self.take_over_focus(FocusStrat::Restore);
        }
    }
}

impl PopupImpl for InputDialog {}

macro_rules! dialog_input {
    ( $ty:ident, $geometry:ident, $styles:ident ) => {{
        let input_dialog = ApplicationWindow::window().input_dialog();
        if input_dialog.visible() {
            return input_dialog;
        }
        input_dialog.set_type(InputType::$ty, $geometry, $styles);
        input_dialog
    }};
}

impl InputDialog {
    /// Get the [`InputDialog`], and set the inside input element to [`Text`]
    ///
    /// @params: </br>
    /// geometry: the position and size of [`InputDialog`] (Required). </br>
    /// styles: the styles of input element within the [`InputDialog`] (Optional). </br>
    #[inline]
    pub fn text(geometry: Rect, styles: Option<Styles>) -> &'static mut InputDialog {
        dialog_input!(Text, geometry, styles)
    }

    /// Get the [`InputDialog`], and set the inside input element to [`Password`]
    ///
    /// @params: </br>
    /// geometry: the position and size of [`InputDialog`] (Required). </br>
    /// styles: the styles of input element within the [`InputDialog`] (Optional). </br>
    #[inline]
    pub fn password(geometry: Rect, styles: Option<Styles>) -> &'static mut InputDialog {
        dialog_input!(Password, geometry, styles)
    }

    /// Get the [`InputDialog`], and set the inside input element to [`Number`]
    ///
    /// @params: </br>
    /// geometry: the position and size of [`InputDialog`] (Required). </br>
    /// styles: the styles of input element within the [`InputDialog`] (Optional). </br>
    #[inline]
    pub fn number(geometry: Rect, styles: Option<Styles>) -> &'static mut InputDialog {
        dialog_input!(Number, geometry, styles)
    }

    /// The [`InputDialog`] will be hidden or not when application window's size has changed.
    ///
    /// The default value is [`false`]
    #[inline]
    pub fn hide_on_win_changed(hide_on_win_changed: bool) {
        ApplicationWindow::window()
            .input_dialog()
            .hide_on_win_changed = hide_on_win_changed;
    }

    /// Get the casted reference of input element within [`InputDialog`].
    #[inline]
    pub fn input_ref<T: Input + StaticType>(&self) -> Option<&T> {
        self.current_child().unwrap().downcast_ref::<T>()
    }

    /// Get the casted mutable reference of input element within [`InputDialog`].
    #[inline]
    pub fn input_mut<T: Input + StaticType>(&mut self) -> Option<&mut T> {
        self.current_child_mut().unwrap().downcast_mut::<T>()
    }
}

impl InputDialog {
    #[inline]
    pub(crate) fn new() -> Box<Self> {
        Object::new(&[])
    }

    #[inline]
    fn on_window_size_changed(&mut self, _: Size) {
        if self.hide_on_win_changed {
            self.hide()
        }
    }

    fn set_type(&mut self, input_type: InputType, geometry: Rect, styles: Option<Styles>) {
        self.set_fixed_x(geometry.x());
        self.set_fixed_y(geometry.y());
        if geometry.width() != 0 {
            self.set_fixed_width(geometry.width());
        }
        if geometry.height() != 0 {
            self.set_fixed_height(geometry.height());
        }

        let (input, idx) = match input_type {
            InputType::Text => {
                self.text.clean();
                (self.text.as_widget_mut(), 0)
            }
            InputType::Password => {
                self.password.clean();
                (self.password.as_widget_mut(), 1)
            }
            InputType::Number => {
                self.number.clean();
                (self.number.as_widget_mut(), 2)
            }
            _ => panic!("Unsupported input type of `InputDialog`"),
        };

        if let Some(styles) = styles {
            input.set_styles(styles);
        };

        self.calc_relative_position();

        self.show();
        self.switch_index(idx);
        self.current_child_mut().unwrap().set_focus(true);
    }
}
