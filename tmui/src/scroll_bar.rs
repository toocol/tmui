use std::mem::size_of;
use tlib::{
    emit,
    namespace::{Orientation, KeyboardModifier},
    object::{ObjectImpl, ObjectSubclass},
    signals,
    values::{FromBytes, FromValue, ToBytes},
};
use crate::{prelude::*, widget::WidgetImpl, graphics::painter::Painter};

pub const DEFAULT_SCROLL_BAR_WIDTH: i32 = 50;

#[extends(Widget)]
#[derive(Default)]
pub struct ScrollBar {
    orientation: Orientation,
    value: i32,
    minimum: i32,
    maximum: i32,
    single_step: i32,
    page_step: i32,
    position: i32,
    pressed: bool,
}

impl ObjectSubclass for ScrollBar {
    const NAME: &'static str = "ScrollBar";
}

impl ObjectImpl for ScrollBar {
    fn construct(&mut self) {
        self.width_request(DEFAULT_SCROLL_BAR_WIDTH)
    }
}

impl WidgetImpl for ScrollBar {
    fn paint(&mut self, mut painter: Painter) {
        if self.size().height() <= 0 {
            return
        }
        let content_rect = self.contents_rect(Some(Coordinate::Widget));
        painter.draw_rect(content_rect);
        painter.fill_rect(content_rect, self.background());

        // Draw the slider.
        let val = self.value();
        let maximum = self.maximum();

        let percentage = val as f32 / maximum as f32;
        let start_pos = (self.size().height() as f32 * percentage) as i32;
    }
}

pub trait ScrollBarSignal: ActionExt {
    signals! {
        /// Emitted when ScrollBar's value has changed.
        /// @param value(i32)
        value_changed();

        /// Emitted when ScrollBar's slider has pressed.
        slider_pressed();

        /// Emitted when ScrollBar's slider has moved.
        /// @param position(i32)
        slider_moved();

        /// Emitted when ScrollBar's slider has released.
        slider_released();

        /// Emitted when ScrollBar's range has changed.
        /// @param min(i32)
        /// @param max(i32)
        range_changed();

        /// Emitted when ScrollBar triggered action.
        /// @param action(SliderAction)
        action_triggered();
    }
}
impl ScrollBarSignal for ScrollBar {}

impl ScrollBar {
    pub fn new(orientation: Orientation) -> Self {
        let mut scroll_bar: Self = Object::new(&[]);
        scroll_bar.orientation = orientation;
        scroll_bar
    }

    /// Get the orientation of Widget
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    /// Setter of property `value`.
    pub fn set_value(&mut self, value: i32) {
        if self.value == value || self.position == value {
            return;
        }
        self.value = value;

        if self.position != value {
            self.position = value;
            if self.pressed {
                emit!(self.slider_moved(), self.position)
            }
        }
        self.update();
        emit!(self.value_changed(), value)
    }
    /// Getter of property `value`.
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Setter of property `minimum`.
    pub fn set_minimum(&mut self, minimum: i32) {
        self.set_range(minimum, self.maximum.max(minimum))
    }
    /// Getter of property `minimum`.
    pub fn minimum(&self) -> i32 {
        self.minimum
    }

    /// Setter of property `maximum`.
    pub fn set_maximum(&mut self, maximum: i32) {
        self.set_range(self.minimum.min(maximum), maximum)
    }
    /// Getter of property `maximum`.
    pub fn maximum(&self) -> i32 {
        self.maximum
    }
    /// Set the range of ScrollBar.
    pub fn set_range(&mut self, min: i32, max: i32) {
        let old_min = self.minimum;
        let old_max = self.maximum;

        self.minimum = min;
        self.maximum = max.max(min);
        
        if old_min != self.minimum || old_max != self.maximum {
            self.update();
            emit!(self.range_changed(), self.minimum, self.maximum);
            self.set_value(self.value);
        }
    }

    /// Setter of property `page_step`.
    pub fn set_page_step(&mut self, page_step: i32) {
        if page_step != self.page_step {
            self.set_steps(self.single_step, page_step)
        }
    }
    /// Getter of property `page_step`.
    pub fn page_step(&self) -> i32 {
        self.page_step
    }

    /// Setter of property `single_step`.
    pub fn set_single_step(&mut self, mut step: i32) {
        if step < 0 {
            step = 1;
        }

        if step != self.single_step {
            self.set_steps(step, self.page_step)
        }
    }
    /// Getter of property `single_step`.
    pub fn single_step(&self) -> i32 {
        self.single_step
    }

    /// Setter of property `slider_position`.
    pub fn set_slider_position(&mut self, position: i32) {
        let position = self.bound(position);
        if position == self.position {
            return;
        }
        self.position = position;
        self.update();
        if self.pressed {
            emit!(self.slider_moved(), self.position)
        }
        self.trigger_action(SliderAction::SliderMove);
    }
    /// Getter of property `slider_position`.
    pub fn slider_position(&self) -> i32 {
        self.position
    }

    /// Trigger action manually.
    pub fn trigger_action(&self, action: SliderAction) {
        emit!(self.action_triggered(), action)
    }

    fn set_steps(&mut self, single: i32, page: i32) {
        self.single_step = single.abs();
        self.page_step = page.abs();
        self.update();
    }

    fn bound(&self, val: i32) -> i32 {
        self.minimum.max(self.maximum.min(val))
    }

    fn scroll_by_delta(&mut self, orientation: Orientation, modifier: KeyboardModifier, delta: i32) {
        todo!()
    }
}

#[repr(u8)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum SliderAction {
    #[default]
    SliderNoAction = 0,
    SliderSingleStepAdd,
    SliderSingleStepSub,
    SliderPageStepAdd,
    SliderPageStepSub,
    SliderToMinimum,
    SliderToMaximum,
    SliderMove,
}
impl SliderAction {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::SliderNoAction => 0,
            Self::SliderSingleStepAdd => 1,
            Self::SliderSingleStepSub => 2,
            Self::SliderPageStepAdd => 3,
            Self::SliderPageStepSub => 4,
            Self::SliderToMinimum => 5,
            Self::SliderToMaximum => 6,
            Self::SliderMove => 7,
        }
    }
}
impl From<u8> for SliderAction {
    fn from(x: u8) -> Self {
        match x {
            0 => Self::SliderNoAction,
            1 => Self::SliderSingleStepAdd,
            2 => Self::SliderSingleStepSub,
            3 => Self::SliderPageStepAdd,
            4 => Self::SliderPageStepSub,
            5 => Self::SliderToMinimum,
            6 => Self::SliderToMaximum,
            7 => Self::SliderMove,
            _ => unimplemented!()
        }
    }
}
impl StaticType for SliderAction {
    fn static_type() -> Type {
        Type::from_name("SliderAction")
    }

    fn bytes_len() -> usize {
        size_of::<u8>()
    }
}
impl ToBytes for SliderAction {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_u8().to_bytes()
    }
}
impl ToValue for SliderAction {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}
impl FromBytes for SliderAction {
    fn from_bytes(data: &[u8], len: usize) -> Self {
        SliderAction::from(u8::from_bytes(data, len))
    }
}
impl FromValue for SliderAction {
    fn from_value(value: &Value) -> Self {
        SliderAction::from_bytes(value.data(), Self::bytes_len())
    }
}
