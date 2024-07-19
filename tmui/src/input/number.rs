use super::{
    text::{TextExt, TextInnerExt, TextProps, TextPropsAcquire, TextShorcutRegister, TextSignals},
    Input, InputSignals, InputType, InputWrapper, INPUT_DEFAULT_BORDER_COLOR,
    INPUT_FOCUSED_BORDER_COLOR,
};
use crate::{
    asset::Asset,
    cast_do, impl_text_shortcut_register,
    prelude::*,
    shortcut::ShortcutRegister,
    svg::{svg_attr::SvgAttr, svg_str::SvgStr},
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};
use lazy_static::lazy_static;
use regex::Regex;
use tlib::{
    events::{KeyEvent, MouseEvent},
    global_watch,
    namespace::KeyCode,
    run_after, shortcut, signals,
    skia_safe::{ClipOp, FontMgr},
    typedef::SkiaSvgDom,
};

const SPINNER_SIZE: f32 = 5.;
const SPINNER_PADDING: f32 = 3.;

lazy_static! {
    // Use `\d*` to match the number after scientific counting symbols `e/E`
    static ref NUMBER_REGEX: Regex =
        Regex::new(r"^[+-]?(\d+\.?\d*|\.\d+)([eE][+-]?\d*)?$").unwrap();
}

#[extends(Widget)]
#[run_after]
#[popupable]
#[global_watch(MouseMove)]
pub struct Number {
    input_wrapper: InputWrapper<String>,
    props: TextProps,

    val: Option<f32>,
    min: Option<f32>,
    max: Option<f32>,
    #[derivative(Default(value = "1."))]
    step: f32,
    #[derivative(Default(value = "true"))]
    enable_spinner: bool,

    arrow_up: Option<SkiaSvgDom>,
    arrow_down: Option<SkiaSvgDom>,
}

pub trait NumberSignals: ActionExt {
    signals! {
        NumberSingals:

        /// Emit when check value failed.
        value_invalid();
    }
}
impl NumberSignals for Number {}

impl ObjectSubclass for Number {
    const NAME: &'static str = "Number";
}

impl ObjectImpl for Number {
    fn construct(&mut self) {
        self.parent_construct();

        self.construct_number();
    }
}

impl WidgetImpl for Number {
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

        self.draw_spinner(painter);
    }

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
    fn font_changed(&mut self) {
        self.handle_font_changed();
    }

    #[inline]
    fn on_get_focus(&mut self) {
        if !self.is_enable() {
            return;
        }

        self.check_blink_timer(true);
        self.set_border_color(INPUT_FOCUSED_BORDER_COLOR);
        self.set_borders(2., 2., 2., 2.);
    }

    #[inline]
    fn on_lose_focus(&mut self) {
        if !self.is_enable() {
            return;
        }

        self.check_blink_timer(false);
        self.set_border_color(INPUT_DEFAULT_BORDER_COLOR);
        self.set_borders(1., 1., 1., 1.);
    }

    #[inline]
    fn on_key_pressed(&mut self, event: &KeyEvent) {
        if !self.is_enable() {
            return;
        }

        let text = event.text();
        if !text.is_empty() && !self.check_number(text) {
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
        if !self.is_enable() {
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
        if !self.is_enable() {
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

impl GlobalWatchImpl for Number {
    #[inline]
    fn on_global_mouse_move(&mut self, evt: &MouseEvent) -> bool {
        if !self.is_enable() {
            return false;
        }

        self.handle_mouse_move(evt);

        false
    }
}

impl Input for Number {
    type Value = String;

    #[inline]
    fn input_type(&self) -> InputType {
        InputType::Number
    }

    #[inline]
    fn input_wrapper(&self) -> &InputWrapper<Self::Value> {
        &self.input_wrapper
    }

    #[inline]
    fn required_handle(&mut self) -> bool {
        true
    }

    #[inline]
    fn check_value(&mut self, val: &Self::Value) -> bool {
        if !self.check_number(val) {
            return false;
        }

        let len = self.value_ref().chars().count();
        self.props_mut().cursor_index = len;
        true
    }
}

impl TextPropsAcquire for Number {
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
        self.value_ref()
    }
}

/// Public implement.
impl Number {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }

    #[inline]
    pub fn val(&self) -> Option<f32> {
        self.val
    }
    #[inline]
    pub fn set_val(&mut self, val: f32) {
        self.val = Some(val)
    }

    #[inline]
    pub fn min(&self) -> Option<f32> {
        self.min
    }
    #[inline]
    pub fn set_min(&mut self, min: f32) {
        self.min = Some(min)
    }

    #[inline]
    pub fn max(&self) -> Option<f32> {
        self.max
    }
    #[inline]
    pub fn set_max(&mut self, max: f32) {
        self.max = Some(max)
    }

    #[inline]
    pub fn step(&self) -> f32 {
        self.step
    }
    #[inline]
    pub fn set_step(&mut self, step: f32) {
        self.step = step
    }

    #[inline]
    pub fn is_enable_spinner(&self) -> bool {
        self.enable_spinner
    }
    #[inline]
    pub fn set_enable_spinner(&mut self, enable_spinner: bool) {
        if self.enable_spinner == enable_spinner {
            return;
        }

        self.enable_spinner = enable_spinner;

        if enable_spinner {
            self.props_mut().fn_calc_text_window = Some(Box::new(calc_text_window_with_spinner));
        } else {
            self.props_mut().fn_calc_text_window = Some(Box::new(calc_text_window_without_spinner));
        }

        self.update()
    }
}

/// Private implement.
impl Number {
    fn construct_number(&mut self) {
        self.input_wrapper.init(self.id());

        self.props_mut().fn_calc_text_window = Some(Box::new(calc_text_window_with_spinner));
        self.register_shortcuts();
        self.text_construct();

        let size = SPINNER_SIZE as u32;
        self.arrow_up = Some(
            SkiaSvgDom::from_str(
                SvgStr::get::<Asset>("arrow_up_small.svg", SvgAttr::new(size, size, Color::BLACK))
                    .unwrap(),
                FontMgr::default(),
            )
            .expect("`Number` create svg dom `arrow_up_small` failed"),
        );
        self.arrow_down = Some(
            SkiaSvgDom::from_str(
                SvgStr::get::<Asset>(
                    "arrow_down_small.svg",
                    SvgAttr::new(size, size, Color::BLACK),
                )
                .unwrap(),
                FontMgr::default(),
            )
            .expect("`Number` create svg dom `arrow_up_small` failed"),
        );
    }

    fn draw_spinner(&self, painter: &mut Painter) {
        if !self.enable_spinner {
            return;
        }

        let spinner_rect = self.spinner_rect();
        if let Some(ref dom) = self.arrow_up {
            painter.save();
            painter.translate(spinner_rect.x(), spinner_rect.y());
            painter.draw_dom(dom);
            painter.restore();
        }
    }

    #[inline]
    fn check_number(&self, val: &str) -> bool {
        let res = if val == "." || val == "e" || val == "E" {
            let mut text = self.value();
            text.insert_str(self.map(self.props().cursor_index), val);
            NUMBER_REGEX.is_match(&text)
        } else {
            NUMBER_REGEX.is_match(val)
        };

        if !res {
            emit!(self.value_invalid());
        }

        res
    }

    #[inline]
    fn spinner_rect(&self) -> FRect {
        if !self.enable_spinner {
            return FRect::default();
        }

        let rect = self.rect_f();
        let text_window = self.props().text_window;

        FRect::new(
            rect.x() + text_window.width(),
            rect.y(),
            rect.width() - text_window.width(),
            rect.height(),
        )
    }
}

fn calc_text_window_with_spinner(props: &TextProps, rect: FRect) -> FRect {
    calc_text_window(props, rect, true)
}
fn calc_text_window_without_spinner(props: &TextProps, rect: FRect) -> FRect {
    calc_text_window(props, rect, false)
}
#[inline]
fn calc_text_window(props: &TextProps, rect: FRect, enable_spinner: bool) -> FRect {
    let font_height = props.font_dimension.1;
    let calced_height = props.calc_widget_height();
    let mut window = FRect::default();

    window.set_x(rect.x() + props.text_padding);
    if calced_height == rect.height() {
        window.set_y(rect.y() + props.text_padding);
    } else {
        let offset = (rect.height() - font_height) / 2.;
        window.set_y(rect.y() + offset.floor());
    }

    let spinner_width = if enable_spinner {
        2. * SPINNER_PADDING + SPINNER_SIZE
    } else {
        0.
    };
    window.set_width(rect.width() - 2. * props.text_padding - spinner_width);
    window.set_height(font_height);

    window
}

impl InputSignals for Number {}
impl TextSignals for Number {}
impl TextExt for Number {}
impl TextInnerExt for Number {}
impl_text_shortcut_register!(Number);

#[cfg(test)]
mod tests {
    use super::NUMBER_REGEX;

    #[test]
    fn test_number_regex() {
        assert!(NUMBER_REGEX.is_match("2.3e1"));
        assert!(NUMBER_REGEX.is_match("23.e"));
        assert!(!NUMBER_REGEX.is_match("2e.1"));
    }
}
