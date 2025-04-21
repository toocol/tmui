#[cfg(not(win_dialog))]
use super::Input;
use super::{
    number::Number,
    password::Password,
    text::{Text, TextExt},
    InputType,
};
use crate::{
    graphics::styles::Styles,
    input::InputSignals,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{widget_ext::FocusStrat, WidgetImpl},
};
#[cfg(win_dialog)]
use strum_macros::Display;
use tlib::values::FromValue;

#[cfg(not(win_dialog))]
pub type TyInputDialog = crate::input::dialog::InputDialog;
#[cfg(win_dialog)]
pub type TyInputDialog = crate::input::dialog::CorrInputDialog;

#[cfg(win_dialog)]
#[extends(Popup, Layout(Stack))]
#[derive(Childrenable)]
#[tlib::win_widget(
    o2s(InputDialogCrsMsg),
    s2o(InputDialogCrsMsg),
    fields(value = "Option<Value>")
)]
pub struct InputDialog {
    #[children]
    text: Tr<Text>,

    #[children]
    password: Tr<Password>,

    #[children]
    number: Tr<Number>,

    #[derivative(Default(value = "InputType::Text"))]
    current: InputType,
}

#[cfg(not(win_dialog))]
#[extends(Popup, Layout(Stack))]
#[derive(Childrenable)]
pub struct InputDialog {
    #[children]
    text: Tr<Text>,

    #[children]
    password: Tr<Password>,

    #[children]
    number: Tr<Number>,

    value: Option<Value>,
    #[derivative(Default(value = "InputType::Text"))]
    current: InputType,
}

impl ObjectSubclass for InputDialog {
    const NAME: &'static str = "InputDialog";
}

impl ObjectImpl for InputDialog {
    #[inline]
    fn construct(&mut self) {
        self.parent_construct();

        #[cfg(not(win_dialog))]
        {
            let window = ApplicationWindow::window();
            self.set_supervisor(window);
        }

        connect!(self.text, value_changed(), self, on_value_changed());
        connect!(self.password, value_changed(), self, on_value_changed());
        connect!(self.number, value_changed(), self, on_value_changed());
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

#[cfg(not(win_dialog))]
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
    pub fn text(geometry: Rect, styles: Option<Styles>) -> &'static mut TyInputDialog {
        dialog_input!(Text, geometry, styles)
    }

    /// Get the [`InputDialog`], and set the inside input element to [`Password`]
    ///
    /// @params: </br>
    /// geometry: the position and size of [`InputDialog`] (Required). </br>
    /// styles: the styles of input element within the [`InputDialog`] (Optional). </br>
    #[inline]
    pub fn password(geometry: Rect, styles: Option<Styles>) -> &'static mut TyInputDialog {
        dialog_input!(Password, geometry, styles)
    }

    /// Get the [`InputDialog`], and set the inside input element to [`Number`]
    ///
    /// @params: </br>
    /// geometry: the position and size of [`InputDialog`] (Required). </br>
    /// styles: the styles of input element within the [`InputDialog`] (Optional). </br>
    #[inline]
    pub fn number(geometry: Rect, styles: Option<Styles>) -> &'static mut TyInputDialog {
        dialog_input!(Number, geometry, styles)
    }

    /// The [`InputDialog`] will be hidden or not when application window's size has changed.
    ///
    /// The default value is [`false`]
    #[inline]
    pub fn hide_on_win_changed(hide_on_win_changed: bool) {
        ApplicationWindow::window()
            .input_dialog()
            .set_hide_on_win_change(hide_on_win_changed);
    }

    /// Get the casted reference of input element within [`InputDialog`].
    #[inline]
    #[cfg(not(win_dialog))]
    pub fn input_ref<T: Input + StaticType>(&self) -> Option<&T> {
        self.current_child().unwrap().downcast_ref::<T>()
    }

    /// Get the casted mutable reference of input element within [`InputDialog`].
    #[inline]
    #[cfg(not(win_dialog))]
    pub fn input_mut<T: Input + StaticType>(&mut self) -> Option<&mut T> {
        self.current_child_mut().unwrap().downcast_mut::<T>()
    }

    #[inline]
    #[cfg(not(win_dialog))]
    pub fn value<T: FromValue + StaticType>(&self) -> Option<T> {
        self.value.as_ref().map(|val| val.get::<T>())
    }
}

impl InputDialog {
    #[inline]
    #[cfg(not(win_dialog))]
    pub(crate) fn new() -> Tr<Self> {
        Self::new_alloc()
    }

    #[cfg(not(win_dialog))]
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
        self.current = input_type;
        self.current_child_mut().unwrap().set_focus(true);
    }

    #[cfg(win_dialog)]
    fn set_type(&mut self, input_type: InputType, styles: Option<Styles>) {
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

        self.switch_index(idx);
        self.current = input_type;
        self.current_child_mut().unwrap().set_focus(true);
    }

    #[cfg(not(win_dialog))]
    fn on_value_changed(&mut self) {
        use super::Input;

        match self.current {
            InputType::Text => self.value = Some(self.text.value().to_value()),
            InputType::Password => self.value = Some(self.password.value().to_value()),
            InputType::Number => self.value = Some(self.number.value().to_value()),
            _ => {}
        }
    }

    #[cfg(win_dialog)]
    fn on_value_changed(&mut self) {
        use super::Input;

        match self.current {
            InputType::Text => self.send_cross_win_msg(InputDialogCrsMsg::ValueChanged(
                self.text.value().to_value(),
            )),
            InputType::Password => self.send_cross_win_msg(InputDialogCrsMsg::ValueChanged(
                self.password.value().to_value(),
            )),
            InputType::Number => self.send_cross_win_msg(InputDialogCrsMsg::ValueChanged(
                self.number.value().to_value(),
            )),
            _ => {}
        }
    }
}

#[cfg(win_dialog)]
impl CorrInputDialog {
    #[inline]
    pub fn value<T: FromValue + StaticType>(&self) -> Option<T> {
        self.value.as_ref().map(|val| val.get::<T>())
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

        self.calc_relative_position();

        self.show();

        self.window().layout_change(self);

        self.send_cross_win_msg(InputDialogCrsMsg::SetType(input_type, styles));
    }
}

////////////////////////////// Cross window message define/handle
#[cfg(win_dialog)]
#[derive(Display)]
pub enum InputDialogCrsMsg {
    // Orgin to sink:
    SetType(InputType, Option<Styles>),

    // Skink to origin:
    ValueChanged(Value),
}

#[cfg(win_dialog)]
impl CrossWinMsgHandler for InputDialog {
    type T = InputDialogCrsMsg;

    #[inline]
    fn handle(&mut self, msg: Self::T) {
        if let InputDialogCrsMsg::SetType(input_type, styles) = msg {
            self.set_type(input_type, styles)
        }
    }
}

#[cfg(win_dialog)]
impl CrossWinMsgHandler for CorrInputDialog {
    type T = InputDialogCrsMsg;

    #[inline]
    fn handle(&mut self, msg: Self::T) {
        if let InputDialogCrsMsg::ValueChanged(value) = msg {
            self.value = Some(value)
        }
    }
}
