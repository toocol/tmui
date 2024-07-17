use lazy_static::lazy_static;
use regex::Regex;
use super::{
    text::{TextExt, TextInnerExt, TextProps, TextPropsAcquire, TextShorcutRegister, TextSignals},
    Input, InputSignals, InputType, InputWrapper,
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
use tlib::{
    global_watch, namespace::KeyCode, run_after, shortcut, skia_safe::FontMgr, typedef::SkiaSvgDom,
};

const ARROW_SIZE: f32 = 5.;

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"^[+-]?(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?$").unwrap();
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
    show_spinner: bool,

    arrow_up: Option<SkiaSvgDom>,
    arrow_down: Option<SkiaSvgDom>,
}

impl ObjectSubclass for Number {
    const NAME: &'static str = "Number";
}

impl ObjectImpl for Number {
    fn construct(&mut self) {
        self.parent_construct();

        self.construct_number();
    }
}

impl WidgetImpl for Number {}

impl GlobalWatchImpl for Number {}

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

impl Number {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}

impl Number {
    #[inline]
    fn construct_number(&mut self) {
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
                SvgStr::get::<Asset>("arrow_up_down.svg", SvgAttr::new(size, size, Color::BLACK))
                    .unwrap(),
                FontMgr::default(),
            )
            .expect("`Number` create svg dom `arrow_up_small` failed"),
        );
    }

    fn check_number(&self, val: &String) -> bool {
        NUMBER_REGEX.is_match(val)
    }
}

impl InputSignals for Number {}
impl TextSignals for Number {}
impl TextExt for Number {}
impl TextInnerExt for Number {}
impl_text_shortcut_register!(Number);
