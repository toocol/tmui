use super::{
    text::{TextExt, TextInnerExt, TextProps, TextPropsAcquire, TextShorcutRegister, TextSignals},
    Input, InputEle, InputSignals, InputWrapper, ReflectInputEle,
};
use crate::{
    cast_do, impl_text_shortcut_register, input_ele_impl,
    prelude::*,
    shortcut::ShortcutRegister,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};
use std::cell::{RefCell, RefMut};
use tlib::{
    connect,
    events::{KeyEvent, MouseEvent},
    global_watch,
    namespace::KeyCode,
    run_after, shortcut, signals,
    skia_safe::ClipOp,
};

#[extends(Widget)]
#[run_after]
#[popupable]
#[global_watch(MouseMove)]
pub struct Password {
    input_wrapper: InputWrapper<String>,
    props: TextProps,
    show_text: RefCell<String>,
    password_visible: bool,
}

impl TextPropsAcquire for Password {
    #[inline]
    fn props(&self) -> &TextProps {
        &self.props
    }

    #[inline]
    fn props_mut(&mut self) -> &mut TextProps {
        &mut self.props
    }

    #[inline]
    fn shown_text(&self) -> Ref<String> {
        if self.password_visible {
            self.value_ref()
        } else {
            self.show_text.borrow()
        }
    }

    #[inline]
    fn shown_text_mut(&self) -> RefMut<String> {
        if self.password_visible {
            self.input_wrapper().value_mut()
        } else {
            self.show_text.borrow_mut()
        }
    }
}

impl ObjectSubclass for Password {
    const NAME: &'static str = "Password";
}

impl ObjectImpl for Password {
    fn construct(&mut self) {
        self.parent_construct();

        self.input_wrapper.init(self.id());
        self.register_shortcuts();
        self.text_construct();

        connect!(self, value_changed(), self, update_shown_text());
    }

    #[inline]
    fn type_register(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<Self, ReflectInputEle>()
    }
}

impl WidgetImpl for Password {
    #[inline]
    fn run_after(&mut self) {
        self.calc_text_geometry();

        self.on_value_changed();
    }

    #[inline]
    fn enable_focus(&self) -> bool {
        true
    }

    #[inline]
    fn paint(&mut self, painter: &mut Painter) {
        if self.is_enable() {
            painter.save();
            painter.clip_rect_global(self.props.text_window, ClipOp::Intersect);

            self.draw_enable(painter);

            painter.restore();
        } else {
            self.draw_disable(painter);
        }

        self.draw_require_invalid(painter);
    }

    #[inline]
    fn font_changed(&mut self) {
        self.handle_font_changed();

        self.calc_text_geometry();
    }

    #[inline]
    fn on_get_focus(&mut self) {
        self.handle_get_focus()
    }

    #[inline]
    fn on_lose_focus(&mut self) {
        self.handle_lose_focus()
    }

    #[inline]
    fn on_key_pressed(&mut self, event: &KeyEvent) {
        if !self.is_enable() {
            return;
        }

        self.handle_key_pressed(event);

        self.update();
    }

    #[inline]
    fn on_key_released(&mut self, _: &KeyEvent) {
        if !self.is_enable() {
            return;
        }

        self.start_blink_timer();
    }

    #[inline]
    fn on_mouse_pressed(&mut self, event: &MouseEvent) {
        if !self.is_enable() || !self.is_focus() {
            return;
        }

        match event.n_press() {
            1 => self.handle_mouse_click(event),
            2 => self.handle_mouse_double_click(),
            _ => {}
        }
    }

    #[inline]
    fn on_mouse_released(&mut self, _: &MouseEvent) {
        if !self.is_enable() || !self.is_focus() {
            return;
        }
        if !self.props.entered {
            self.window()
                .set_cursor_shape(SystemCursorShape::ArrowCursor);
        }

        self.handle_mouse_release()
    }

    #[inline]
    fn on_mouse_enter(&mut self, _: &MouseEvent) {
        self.props.entered = true;
        self.window()
            .set_cursor_shape(SystemCursorShape::TextCursor);
    }

    #[inline]
    fn on_mouse_leave(&mut self, _: &MouseEvent) {
        self.props.entered = false;
        let window = self.window();
        if self.id() == window.pressed_widget() {
            return;
        }
        window.set_cursor_shape(SystemCursorShape::ArrowCursor);
    }
}

impl GlobalWatchImpl for Password {
    #[inline]
    fn on_global_mouse_move(&mut self, evt: &MouseEvent) -> bool {
        if !self.is_enable() {
            return false;
        }

        self.handle_mouse_move(evt);

        false
    }
}

impl Input for Password {
    type Value = String;

    #[inline]
    fn input_type(&self) -> super::InputType {
        super::InputType::Password
    }

    #[inline]
    fn input_wrapper(&self) -> &InputWrapper<Self::Value> {
        &self.input_wrapper
    }

    #[inline]
    fn required_handle(&mut self) -> bool {
        self.inner_required_handle()
    }

    #[inline]
    fn check_value(&mut self, val: &Self::Value) -> bool {
        let len = val.chars().count();
        self.props_mut().cursor_index = len;
        true
    }
}

impl Password {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }

    /// Set the input element `Password`'s text is visible or not.
    ///
    /// The default value was `false`.
    #[inline]
    pub fn set_password_visible(&mut self, password_visible: bool) {
        if self.password_visible == password_visible {
            return;
        }
        self.password_visible = password_visible;
        self.update();
        emit!(self, password_visible_changed(self.password_visible));
    }

    /// Is the input element `Password`'s text is visible or not.
    #[inline]
    pub fn is_password_visible(&mut self) -> bool {
        self.password_visible
    }
}

impl Password {
    #[inline]
    fn update_shown_text(&mut self) {
        let val = self.input_wrapper.value_ref();
        *self.show_text.borrow_mut() = "*".repeat(val.chars().count()).to_string();
    }
}

pub trait PasswordSignals: ActionExt {
    signals! {
        PasswordSignals:

        /// Emit when component's password visible was changed.
        ///
        /// @param [`bool`]
        password_visible_changed(bool);
    }
}
impl PasswordSignals for Password {}

impl TextSignals for Password {}
impl InputSignals for Password {}
impl TextExt for Password {}
impl TextInnerExt for Password {}
impl_text_shortcut_register!(Password);
input_ele_impl!(Password);
