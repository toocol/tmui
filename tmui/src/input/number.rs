use super::{
    text::{TextExt, TextInnerExt, TextProps, TextPropsAcquire, TextShorcutRegister, TextSignals},
    Input, InputEle, InputSignals, InputType, InputWrapper,
};
use crate::{
    asset::Asset,
    cast_do, impl_text_shortcut_register, input_ele_impl,
    prelude::*,
    shortcut::ShortcutRegister,
    svg::{svg_attr::SvgAttr, svg_str::SvgStr},
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};
use lazy_static::lazy_static;
use regex::Regex;
use rust_decimal::{prelude::FromPrimitive, prelude::*, Decimal};
use tlib::{
    connect,
    events::{KeyEvent, MouseEvent},
    global::shown_value_64,
    global_watch,
    namespace::KeyCode,
    run_after, shortcut, signals,
    skia_safe::{ClipOp, FontMgr},
    typedef::SkiaSvgDom,
};

const ARROW_SIZE: f32 = 7.;
const SPINNER_WIDTH: f32 = 13.;
const SPINNER_HEIGHT: f32 = 8.;

const SPINNER_BACKGROUND: Color = Color::grey_with(220);
const SPINNER_EFFECT_BACKGROUND: Color = Color::grey_with(180);

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

    val: Option<f64>,
    init_val: Option<f32>,
    min: Option<f32>,
    max: Option<f32>,
    #[derivative(Default(value = "1."))]
    step: f32,
    #[derivative(Default(value = "true"))]
    enable_spinner: bool,

    arrow_up: Option<SkiaSvgDom>,
    arrow_down: Option<SkiaSvgDom>,

    spinner_up_effect: bool,
    spinner_down_effect: bool,
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

        let text = event.text();
        if !text.is_empty() && !self.check_number(text, false) {
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

        self.handle_spinner_press(event);
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
        if !self.props.blink_timer.is_active() {
            self.start_blink_timer()
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

        self.handle_spinner_hover(evt);

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
        if !self.check_number(val, true) {
            return false;
        }

        let len = val.chars().count();
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
        if let Some(val) = self.val {
            if let Some(max) = self.max {
                if val > max as f64 {
                    return None;
                }
            }

            if let Some(min) = self.max {
                if val < min as f64 {
                    return None;
                }
            }

            Some(val as f32)
        } else {
            self.val.map(|val| val as f32)
        }
    }
    #[inline]
    pub fn set_val(&mut self, val: f32) {
        self.init_val = Some(val);
        self.set_value(shown_value_64(val as f64));
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

        let size = ARROW_SIZE as u32;
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
            .expect("`Number` create svg dom `arrow_down_small` failed"),
        );

        connect!(self, value_changed(), self, handle_number_value_changed());
    }

    fn draw_spinner(&self, painter: &mut Painter) {
        if !self.enable_spinner {
            return;
        }

        let (inner1, inner2) = self.spinner_rect();

        let background = if self.spinner_up_effect {
            SPINNER_EFFECT_BACKGROUND
        } else {
            SPINNER_BACKGROUND
        };
        painter.fill_rect_global(inner1, background);

        let background = if self.spinner_down_effect {
            SPINNER_EFFECT_BACKGROUND
        } else {
            SPINNER_BACKGROUND
        };
        painter.fill_rect_global(inner2, background);

        let x = inner1.x() + (inner1.width() - ARROW_SIZE) / 2.;
        let y = inner1.y() + (inner1.height() - ARROW_SIZE) / 2.;
        if let Some(ref dom) = self.arrow_up {
            painter.save();
            painter.translate(x, y);
            painter.draw_dom(dom);
            painter.restore();
        }

        let x = inner2.x() + (inner1.width() - ARROW_SIZE) / 2.;
        let y = inner2.y() + (inner1.height() - ARROW_SIZE) / 2.;
        if let Some(ref dom) = self.arrow_down {
            painter.save();
            painter.translate(x, y);
            painter.draw_dom(dom);
            painter.restore();
        }
    }

    #[inline]
    fn check_number(&self, val: &str, whole_set: bool) -> bool {
        let res =
            if (val == "." || val == "e" || val == "E" || val == "+" || val == "-") && !whole_set {
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
    fn spinner_rect(&self) -> (FRect, FRect) {
        if !self.enable_spinner {
            return (FRect::default(), FRect::default());
        }

        let rect = self.rect_f();
        let text_window = self.props().text_window;

        let outer_rect = FRect::new(
            rect.x() + text_window.width() + self.props().text_padding,
            rect.y(),
            rect.width() - text_window.width(),
            rect.height(),
        );

        let center_height = SPINNER_HEIGHT * 2.;

        let y1 = outer_rect.y() + (outer_rect.height() - center_height) / 2.;
        let y2 = y1 + SPINNER_HEIGHT;
        let xm = outer_rect.x();
        let inner1 = FRect::new(xm, y1, SPINNER_WIDTH, SPINNER_HEIGHT);
        let inner2 = FRect::new(xm, y2, SPINNER_WIDTH, SPINNER_HEIGHT);
        (inner1, inner2)
    }

    #[inline]
    fn handle_spinner_hover(&mut self, evt: &MouseEvent) {
        let (spinner1, spinner2) = self.spinner_rect();
        let pos = evt.position().into();

        let mut need_update = false;
        let spinner1_effect = spinner1.contains(&pos);
        let spinner2_effect = spinner2.contains(&pos);
        if self.spinner_up_effect != spinner1_effect || self.spinner_down_effect != spinner2_effect
        {
            need_update = true;
        }
        self.spinner_up_effect = spinner1_effect;
        self.spinner_down_effect = spinner2_effect;

        let window = self.window();
        if self.spinner_up_effect || self.spinner_down_effect {
            window.set_cursor_shape(SystemCursorShape::ArrowCursor);
        } else if self.props().entered {
            window.set_cursor_shape(SystemCursorShape::TextCursor);
        }

        if need_update {
            self.update_styles_rect(CoordRect::new(
                FRect::new(
                    spinner1.x(),
                    spinner1.y(),
                    spinner1.width(),
                    spinner1.height() + spinner2.height(),
                ),
                Coordinate::World,
            ))
        }
    }

    fn handle_spinner_press(&mut self, evt: &MouseEvent) {
        let pos = self.map_to_global_f(&evt.position().into());
        let (spinner1, spinner2) = self.spinner_rect();
        let spinner1_effect = spinner1.contains(&pos);
        let spinner2_effect = spinner2.contains(&pos);

        if !spinner1_effect && !spinner2_effect {
            return;
        }

        let mut res = if spinner1_effect {
            if let Some(val) = self.val {
                if val == f64::INFINITY {
                    self.step as f64
                } else {
                    let val = Decimal::from_f64(val).unwrap();
                    let step = Decimal::from_f32(self.step).unwrap();
                    (val + step).to_f64().unwrap()
                }
            } else {
                self.step as f64
            }
        } else if spinner2_effect {
            if let Some(val) = self.val {
                if val == f64::INFINITY {
                    -self.step as f64
                } else {
                    let val = Decimal::from_f64(val).unwrap();
                    let step = Decimal::from_f32(self.step).unwrap();
                    (val - step).to_f64().unwrap()
                }
            } else {
                -self.step as f64
            }
        } else {
            0.
        };

        if let Some(max) = self.max {
            if res > max as f64 {
                res = max as f64;
            }
        }

        if let Some(min) = self.min {
            if res < min as f64 {
                res = min as f64;
            }
        }

        self.props_mut().blink_timer.stop();
        self.props_mut().cursor_visible = true;

        self.set_value(shown_value_64(res));
    }

    fn handle_number_value_changed(&mut self) {
        let parse = self.value_ref().parse::<f64>();
        match parse {
            Ok(val) => self.val = Some(val),
            Err(_) => {
                if let Some(init_val) = self.init_val {
                    self.val = Some(init_val as f64)
                } else {
                    self.val = None
                }
            }
        }
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

    let spinner_width = if enable_spinner { SPINNER_WIDTH } else { 0. };
    window.set_width(rect.width() - 2. * props.text_padding - spinner_width);
    window.set_height(font_height);

    window
}

impl InputSignals for Number {}
impl TextSignals for Number {}
impl TextExt for Number {}
impl TextInnerExt for Number {}
impl_text_shortcut_register!(Number);
input_ele_impl!(Number);

#[cfg(test)]
mod tests {
    use super::NUMBER_REGEX;

    #[test]
    fn test_number_regex() {
        assert!(NUMBER_REGEX.is_match("2.3e1"));
        assert!(NUMBER_REGEX.is_match("23.e"));
        assert!(!NUMBER_REGEX.is_match("2e.1"));
        assert!(NUMBER_REGEX.is_match("-2"));
        assert!(NUMBER_REGEX.is_match("+2"));
        assert!(NUMBER_REGEX.is_match("-23.e"));
    }
}
