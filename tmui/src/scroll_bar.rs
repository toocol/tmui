use crate::{
    application::wheel_scroll_lines, graphics::painter::Painter, prelude::*, widget::WidgetImpl,
};
use derivative::Derivative;
use std::mem::size_of;
use tlib::{
    emit,
    events::DeltaType,
    global::bound,
    implements_enum_value,
    namespace::{AsNumeric, KeyboardModifier, Orientation},
    object::{ObjectImpl, ObjectSubclass},
    signals,
    values::{FromBytes, FromValue, ToBytes},
};

pub const DEFAULT_SCROLL_BAR_WIDTH: i32 = 10;
pub const DEFAULT_SCROLL_BAR_HEIGHT: i32 = 10;

pub const DEFAULT_SCROLL_BAR_BACKGROUND: Color = Color::from_rgb(100, 100, 100);
pub const DEFAULT_SLIDER_BACKGROUND: Color = Color::from_rgb(250, 250, 250);

#[extends(Widget)]
pub struct ScrollBar {
    #[derivative(Default(value = "Orientation::Vertical"))]
    orientation: Orientation,
    /// Indicates the distance of the slider from the start of the scroll bar.
    value: i32,
    /// The minimum value of field `value`.
    #[derivative(Default(value = "0"))]
    minimum: i32,
    /// The maximum value of field `value`.
    #[derivative(Default(value = "99"))]
    maximum: i32,
    /// The distance the slider moves after a single click on the scroll arrow or pressing the move cursor key.
    #[derivative(Default(value = "1"))]
    single_step: i32,
    /// When pressing the up and down page keys or clicking the mouse on the scroll bar, the distance to move.
    #[derivative(Default(value = "10"))]
    page_step: i32,
    /// The current position of the slider, if the tracking attribute is true,
    /// its value is equal to the value attribute value
    position: i32,
    /// Confirm if the scroll bar slider is held down
    pressed: bool,
    offset_accumulated: f32,
    scroll_bar_position: ScrollBarPosition,
}

impl ObjectSubclass for ScrollBar {
    const NAME: &'static str = "ScrollBar";
}

impl ObjectImpl for ScrollBar {
    fn construct(&mut self) {
        self.parent_construct();

        match self.orientation {
            Orientation::Horizontal => self.height_request(DEFAULT_SCROLL_BAR_HEIGHT),
            Orientation::Vertical => self.width_request(DEFAULT_SCROLL_BAR_WIDTH),
        }
    }
}

impl WidgetImpl for ScrollBar {
    fn paint(&mut self, mut painter: Painter) {
        let size = self.size();
        let content_rect = self.contents_rect(Some(Coordinate::Widget));
        painter.draw_rect(content_rect);
        painter.fill_rect(content_rect, DEFAULT_SCROLL_BAR_BACKGROUND);

        let val = self.value();
        let slider_len = (size.height() as f32 * 0.2) as i32;
        let maximum = self.maximum();
        let percentage = val as f32 / maximum as f32;

        // Draw the slider.
        match self.orientation {
            Orientation::Vertical => {
                let start_y = ((size.height() - slider_len) as f32 * percentage) as i32;

                let rect = Rect::new(content_rect.x(), start_y, size.width(), slider_len);
                painter.draw_rect(rect);
                painter.fill_rect(rect, DEFAULT_SLIDER_BACKGROUND);
            }
            Orientation::Horizontal => {
                let start_x = ((size.width() - slider_len) as f32 * percentage) as i32;

                let rect = Rect::new(start_x, content_rect.y(), slider_len, size.height());
                painter.draw_rect(rect);
                painter.fill_rect(rect, DEFAULT_SLIDER_BACKGROUND);
            }
        }
    }

    fn on_mouse_wheel(&mut self, event: &tlib::events::MouseEvent) {
        let horizontal = event.delta().x().abs() > event.delta().y().abs();

        if !horizontal && event.delta().x() != 0 && self.orientation() == Orientation::Horizontal {
            return;
        }

        let delta = if horizontal {
            -event.delta().x()
        } else {
            match event.delta_type() {
                DeltaType::Line => -event.delta().y(),
                DeltaType::Pixel => event.delta().y(),
            }
        };

        if self.scroll_by_delta(event.modifier(), delta, event.delta_type()) {
            self.update()
        }
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
    #[inline]
    pub fn new(orientation: Orientation) -> Box<Self> {
        let mut scroll_bar: Box<Self> = Object::new(&[]);
        scroll_bar.orientation = orientation;
        scroll_bar
    }

    /// Get the orientation of Widget.
    #[inline]
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    /// Set the orientation of Widget.
    #[inline]
    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    /// Setter of property `value`.
    pub fn set_value(&mut self, value: i32) {
        self.value = value;

        if self.position != value {
            self.position = value;
        }
        if self.pressed {
            emit!(self.slider_moved(), self.position)
        }
        emit!(self.value_changed(), value);
        self.update();
    }
    /// Getter of property `value`.
    #[inline]
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Setter of property `minimum`.
    #[inline]
    pub fn set_minimum(&mut self, minimum: i32) {
        self.set_range(minimum, self.maximum.max(minimum))
    }
    /// Getter of property `minimum`.
    #[inline]
    pub fn minimum(&self) -> i32 {
        self.minimum
    }

    /// Setter of property `maximum`.
    #[inline]
    pub fn set_maximum(&mut self, maximum: i32) {
        self.set_range(self.minimum.min(maximum), maximum)
    }
    /// Getter of property `maximum`.
    #[inline]
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
    #[inline]
    pub fn set_page_step(&mut self, page_step: i32) {
        if page_step != self.page_step {
            self.set_steps(self.single_step, page_step)
        }
    }
    /// Getter of property `page_step`.
    #[inline]
    pub fn page_step(&self) -> i32 {
        self.page_step
    }

    /// Setter of property `single_step`.
    #[inline]
    pub fn set_single_step(&mut self, mut step: i32) {
        if step < 0 {
            step = 1;
        }

        if step != self.single_step {
            self.set_steps(step, self.page_step)
        }
    }
    /// Getter of property `single_step`.
    #[inline]
    pub fn single_step(&self) -> i32 {
        self.single_step
    }

    /// Get the scroll bar position
    #[inline]
    pub fn scroll_bar_position(&self) -> ScrollBarPosition {
        self.scroll_bar_position
    }

    /// Set the scroll bar position
    #[inline]
    pub fn set_scroll_bar_position(&mut self, scroll_bar_position: ScrollBarPosition) {
        self.scroll_bar_position = scroll_bar_position;
        self.update()
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
    #[inline]
    pub fn slider_position(&self) -> i32 {
        self.position
    }

    /// Trigger action manually.
    #[inline]
    pub fn trigger_action(&mut self, action: SliderAction) {
        match action {
            SliderAction::SliderSingleStepAdd => {
                self.set_slider_position(self.overflow_safe_add(self.effective_single_step()));
            }
            SliderAction::SliderSingleStepSub => {
                self.set_slider_position(self.overflow_safe_add(-self.effective_single_step()));
            }
            SliderAction::SliderPageStepAdd => {
                self.set_slider_position(self.overflow_safe_add(self.page_step));
            }
            SliderAction::SliderPageStepSub => {
                self.set_slider_position(self.overflow_safe_add(-self.page_step));
            }
            SliderAction::SliderToMinimum => self.set_slider_position(self.minimum),
            SliderAction::SliderToMaximum => self.set_slider_position(self.maximum),
            SliderAction::SliderMove | SliderAction::SliderNoAction => {}
        }
        self.set_value(self.position);
        emit!(self.action_triggered(), action);
    }

    /// Scroll the ScrollBar. </br>
    /// delta was positive value when scroll down/right.
    #[inline]
    pub fn scroll(&mut self, delta: i32, delta_type: DeltaType) {
        self.scroll_by_delta(KeyboardModifier::NoModifier, delta, delta_type);
    }

    #[inline]
    fn set_steps(&mut self, single: i32, page: i32) {
        self.single_step = single.abs();
        self.page_step = page.abs();
        self.update();
    }

    pub(crate) fn scroll_by_delta(&mut self, modifier: KeyboardModifier, delta: i32, delta_type: DeltaType) -> bool {
        let steps_to_scroll;
        let dividend = match delta_type {
            DeltaType::Line => 1.,
            DeltaType::Pixel => 120.,
        };
        let offset = delta as f32 / dividend;
        if modifier.has(KeyboardModifier::ControlModifier)
            || modifier.has(KeyboardModifier::ShiftModifier)
        {
            // Scroll one page regardless of delta:
            steps_to_scroll = bound(
                -self.page_step,
                offset as i32 * self.page_step,
                self.page_step,
            );
            self.offset_accumulated = 0.;
        } else {
            let steps_to_scroll_f =
                wheel_scroll_lines() as f32 * offset * self.effective_single_step() as f32;
            // Check if wheel changed direction since last event:
            if self.offset_accumulated != 0. && (offset / self.offset_accumulated) < 0. {
                self.offset_accumulated = 0.;
            }

            self.offset_accumulated += steps_to_scroll_f;

            // Don't scroll more than one page in any case:
            steps_to_scroll = bound(
                -self.page_step,
                self.offset_accumulated as i32,
                self.page_step,
            );

            self.offset_accumulated -= (self.offset_accumulated as i32) as f32;
            if steps_to_scroll == 0 {
                // We moved less than a line, but might still have accumulated partial scroll,
                // unless we already are at one of the ends.
                if self.offset_accumulated > 0. && self.value < self.maximum {
                    return true;
                }
                if self.offset_accumulated < 0. && self.value > self.minimum {
                    return true;
                }
                self.offset_accumulated = 0.;
                return false;
            }
        }

        let pref_value = self.value;
        self.position = self.bound(self.overflow_safe_add(steps_to_scroll));
        self.trigger_action(SliderAction::SliderMove);

        if pref_value == self.value {
            self.offset_accumulated = 0.;
            return false;
        }
        return true;
    }

    #[inline]
    fn effective_single_step(&self) -> i32 {
        self.single_step
    }

    #[inline]
    fn bound(&self, val: i32) -> i32 {
        self.minimum.max(self.maximum.min(val))
    }

    #[inline]
    fn overflow_safe_add(&self, add: i32) -> i32 {
        let mut new_value = self.value + add;
        if add > 0 && new_value < self.value {
            new_value = self.maximum;
        } else if add < 0 && new_value > self.value {
            new_value = self.minimum;
        }
        new_value
    }
}

#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum ScrollBarPosition {
    Start,
    #[default]
    End,
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
            _ => unimplemented!(),
        }
    }
}
impl AsNumeric<u8> for SliderAction {
    fn as_numeric(&self) -> u8 {
        self.as_u8()
    }
}
implements_enum_value!(SliderAction, u8);
