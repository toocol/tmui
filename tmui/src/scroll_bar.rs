use crate::{
    application::wheel_scroll_lines,
    graphics::painter::Painter,
    overlay::{PartCovered, ReflectPartCovered},
    prelude::*,
    widget::{widget_inner::WidgetInnerExt, RegionClear, WidgetImpl},
};
use derivative::Derivative;
use std::{mem::size_of, time::Duration};
use tlib::{
    connect, emit,
    events::{DeltaType, MouseEvent},
    global::bound,
    implements_enum_value, isolated_visibility,
    namespace::{AsNumeric, BlendMode, KeyboardModifier, Orientation},
    object::{ObjectImpl, ObjectSubclass},
    run_after, signals,
    timer::Timer,
    values::{FromBytes, FromValue, ToBytes},
};

pub const DEFAULT_SCROLL_BAR_WIDTH: i32 = 10;
pub const DEFAULT_SCROLL_BAR_HEIGHT: i32 = 10;

pub const DEFAULT_SCROLL_BAR_BACKGROUND: Color = Color::GREY_LIGHT;
pub const DEFAULT_SLIDER_BACKGROUND: Color = Color::from_rgb(250, 250, 250);

#[extends(Widget)]
#[run_after]
#[isolated_visibility]
pub struct ScrollBar {
    #[derivative(Default(value = "Orientation::Vertical"))]
    orientation: Orientation,
    /// Indicates the distance of the slider from the start of the scroll bar.
    value: i32,
    /// The minimum value of field `value`.
    #[derivative(Default(value = "0"))]
    minimum: i32,
    /// The maximum value of field `value`.
    #[derivative(Default(value = "0"))]
    maximum: i32,
    /// The value represent the visible area.
    /// To determine the slider length with maximum;
    ///
    /// ### Default:
    /// slider_len = 0.15 * scroll_bar_size;
    ///
    /// ### The `visible_area` was specified:
    /// slider_len = (visible_area / maximum + visible_area) * scroll_bar_size;
    visible_area: Option<i32>,
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
    press_offset: i32,
    offset_accumulated: f32,
    scroll_bar_position: ScrollBarPosition,
    overlaid: bool,

    slider_radius: f32,
    color: Color,

    active_background: Option<Color>,
    active_color: Option<Color>,
    mouse_in: bool,
    visible_in_valid: bool,
    wheel_timer: Box<Timer>,
}

impl ObjectSubclass for ScrollBar {
    const NAME: &'static str = "ScrollBar";
}

impl ObjectImpl for ScrollBar {
    fn construct(&mut self) {
        self.parent_construct();

        match self.orientation {
            Orientation::Horizontal => {
                self.set_hexpand(true);
                self.height_request(DEFAULT_SCROLL_BAR_HEIGHT);
            }
            Orientation::Vertical => {
                self.set_vexpand(true);
                self.width_request(DEFAULT_SCROLL_BAR_WIDTH);
            }
        }

        self.set_mouse_tracking(true);
        self.set_background(DEFAULT_SCROLL_BAR_BACKGROUND);
        self.color = DEFAULT_SLIDER_BACKGROUND;

        connect!(self.wheel_timer, timeout(), self, wheel_timer_timeout());
    }

    #[inline]
    fn type_register(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<Self, ReflectPartCovered>()
    }
}

impl WidgetImpl for ScrollBar {
    #[inline]
    fn run_after(&mut self) {
        if self.auto_hide {
            self.window().add_shadow_mouse_watch(self);
            self.hide()
        }
        if self.visible_in_valid && self.minimum == self.maximum {
            self.hide()
        }
    }

    #[inline]
    fn blend_mode(&self) -> BlendMode {
        if self.overlaid && self.background().a() != 255 {
            BlendMode::SrcOver
        } else {
            BlendMode::default()
        }
    }

    #[inline]
    fn notify_update(&mut self) {
        if !self.initialized() {
            return;
        }

        if self.overlaid {
            emit!(self.need_update());
        } else {
            self.update();
        }
    }

    fn paint(&mut self, painter: &mut Painter) {
        let content_rect = self.contents_rect(Some(Coordinate::Widget));

        painter.set_antialiasing(true);
        let transparency = self.transparency();
        let background = if self.is_active() && self.active_background.is_some() {
            self.active_background.unwrap()
        } else {
            self.background()
        };
        if background.a() != 255 || transparency != 255 {
            painter.set_blend_mode(BlendMode::SrcOver);

            if !self.overlaid {
                self.clear(painter, content_rect);
            }
        } else {
            painter.set_blend_mode(BlendMode::default());
        }
        painter.fill_rect(content_rect, background);

        // Draw the slider.
        let color = if self.is_active() && self.active_color.is_some() {
            self.active_color.unwrap()
        } else {
            self.color
        };
        if color.a() != 255 || transparency != 255 {
            painter.set_blend_mode(BlendMode::SrcOver);
        } else {
            painter.set_blend_mode(BlendMode::default());
        }
        if self.slider_radius > 0. {
            painter.fill_round_rect(self.calculate_slider(), self.slider_radius, color);
        } else {
            painter.fill_rect(self.calculate_slider(), color);
        }
    }

    fn on_mouse_wheel(&mut self, event: &MouseEvent) {
        let horizontal = event.delta().x().abs() > event.delta().y().abs();

        if !horizontal && event.delta().x() != 0 || self.orientation() == Orientation::Horizontal {
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
            self.notify_update()
        }

        if self.auto_hide {
            self.show();
        }
        self.reset_wheel_timer();
    }

    fn on_mouse_pressed(&mut self, event: &MouseEvent) {
        let (x, y) = event.position();
        let slider = self.calculate_slider();

        if slider.contains(&event.position().into()) {
            match self.orientation {
                Orientation::Horizontal => self.press_offset = x - slider.x(),
                Orientation::Vertical => self.press_offset = y - slider.y(),
            }
            self.pressed = true;
            if self.repaint_when_active() {
                self.notify_update();
            }
        } else {
            let size = self.size();
            let slider_len = self.slider_len();

            let value = match self.orientation {
                Orientation::Horizontal => {
                    let start_x = x - slider_len / 2;
                    (start_x as i64 * self.maximum as i64) / (size.height() - slider_len) as i64
                }
                Orientation::Vertical => {
                    let start_y = y - slider_len / 2;
                    (start_y as i64 * self.maximum as i64) / (size.height() - slider_len) as i64
                }
            } as i32;

            self.set_value(value.min(self.maximum).max(0));
        }

        self.window().high_load_request(true);
    }

    fn on_mouse_released(&mut self, event: &MouseEvent) {
        self.pressed = false;
        self.window().high_load_request(false);

        let rect = self.origin_rect(Some(Coordinate::Widget));
        if !rect.contains(&event.position().into()) && self.auto_hide {
            self.hide();
            return;
        }

        if self.repaint_when_active() {
            self.notify_update();
        }
    }

    fn on_mouse_move(&mut self, event: &MouseEvent) {
        if !self.pressed {
            return;
        }
        if self.minimum == self.maximum {
            return;
        }
        match self.orientation {
            Orientation::Horizontal => {
                let size = self.size();
                let (x, _) = event.position();

                let start_x = x - self.press_offset;

                let slider_len = self.slider_len();
                let maximum = self.maximum();
                let value =
                    (start_x as i64 * maximum as i64) / (size.width() as i64 - slider_len as i64);
                self.set_value((value as i32).min(maximum).max(0));
            }
            Orientation::Vertical => {
                let size = self.size();
                let (_, y) = event.position();

                let start_y = y - self.press_offset;

                let slider_len = self.slider_len();
                let maximum = self.maximum();
                let value =
                    (start_y as i64 * maximum as i64) / (size.height() as i64 - slider_len as i64);
                self.set_value((value as i32).min(maximum).max(0));
            }
        }
    }

    #[inline]
    fn on_mouse_enter(&mut self, _: &MouseEvent) {
        self.mouse_in = true;
        if self.visible_in_valid && self.minimum == self.maximum {
            return;
        }
        if !self.visible() && self.auto_hide {
            self.show();
        } else if self.repaint_when_active() {
            self.notify_update();
        }
    }

    #[inline]
    fn on_mouse_leave(&mut self, _: &MouseEvent) {
        self.mouse_in = false;
        if self.visible() && !self.is_active() && self.auto_hide {
            self.wheel_timer.stop();
            self.hide();
        } else if self.repaint_when_active() {
            self.notify_update();
        }
    }

    #[inline]
    fn visibility_check(&self) -> bool {
        !(self.visible_in_valid && self.minimum == self.maximum)
    }
}

pub trait ScrollBarSignal: ActionExt {
    signals! {
        ScrollBarSignal:

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

        /// Emitted when ScrollBar need update when under overlaid mode.
        need_update();
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
        match orientation {
            Orientation::Horizontal => {
                self.height_request(DEFAULT_SCROLL_BAR_HEIGHT);
                self.cancel_fixed_width();
                self.set_hexpand(true);
                self.set_vexpand(false);
            }
            Orientation::Vertical => {
                self.width_request(DEFAULT_SCROLL_BAR_WIDTH);
                self.cancel_fixed_height();
                self.set_hexpand(false);
                self.set_vexpand(true);
            }
        }
    }

    #[inline]
    pub fn color(&self) -> Color {
        self.color
    }

    #[inline]
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
        self.notify_update();
    }

    /// See [`ScrollBar::visible_area`]
    #[inline]
    pub fn get_visible_area(&self) -> Option<i32> {
        self.visible_area
    }
    /// See [`ScrollBar::visible_area`]
    #[inline]
    pub fn set_visible_area(&mut self, visible_area: i32) {
        if visible_area <= 0 {
            return;
        }
        self.visible_area = Some(visible_area);
    }

    /// Setter of property `value`.
    pub fn set_value(&mut self, value: i32) {
        self.value = value;

        if self.position != value {
            self.position = value;
        }
        if self.pressed {
            emit!(ScrollBar::set_value => self.slider_moved(), self.position)
        }
        emit!(ScrollBar::set_value => self.value_changed(), value);
        self.notify_update();
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
            self.set_value(if self.value > self.maximum {
                self.maximum
            } else {
                self.value
            });
            emit!(ScrollBar::set_range => self.range_changed(), self.minimum, self.maximum);

            if self.minimum != self.maximum && self.visible_in_valid && !self.auto_hide {
                self.show()
            } else if self.visible_in_valid && self.minimum == self.maximum {
                self.hide()
            }

            if self.visible() {
                self.notify_update();
            }
        }
    }
    #[inline]
    pub fn get_range(&self) -> (i32, i32) {
        (self.minimum, self.maximum)
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
        self.notify_update();
    }

    /// Setter of property `slider_position`.
    pub fn set_slider_position(&mut self, position: i32) {
        let position = self.bound(position);
        if position == self.position {
            return;
        }
        self.position = position;
        self.notify_update();
        if self.pressed {
            emit!(ScrollBar::set_slider_position => self.slider_moved(), self.position)
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
        emit!(ScrollBar::trigger_action => self.action_triggered(), action);
    }

    /// Scroll the ScrollBar. </br>
    /// delta was positive value when scroll down/right.
    #[inline]
    pub fn scroll(&mut self, delta: i32, delta_type: DeltaType) {
        self.scroll_by_delta(KeyboardModifier::NoModifier, delta, delta_type);
    }

    #[inline]
    pub fn slider_pressed(&self) -> bool {
        self.pressed
    }

    /// Do not call this function directly.
    /// 
    /// Use [`ScrollArea::set_layout_mode`](crate::scroll_area::ScrollArea::set_layout_mode).
    #[inline]
    pub fn set_overlaid(&mut self, overlaid: bool) {
        self.overlaid = overlaid;
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.mouse_in || self.pressed
    }

    #[inline]
    pub fn set_active_color(&mut self, color: Option<Color>) {
        self.active_color = color;
    }

    #[inline]
    pub fn set_active_background(&mut self, background: Option<Color>) {
        self.active_background = background;
    }

    #[inline]
    pub fn repaint_when_active(&self) -> bool {
        self.active_color.is_some() || self.active_background.is_some()
    }

    #[inline]
    pub fn set_slider_radius(&mut self, radius: f32) {
        self.slider_radius = radius;
    }
    #[inline]
    pub fn slider_radius(&self) -> f32 {
        self.slider_radius
    }

    #[inline]
    pub fn set_visible_in_valid(&mut self, visible_in_valid: bool) {
        self.visible_in_valid = visible_in_valid;
    }
    #[inline]
    pub fn visible_in_valid(&self) -> bool {
        self.visible_in_valid
    }

    #[inline]
    fn set_steps(&mut self, single: i32, page: i32) {
        self.single_step = single.abs();
        self.page_step = page.abs();
        self.notify_update();
    }

    pub fn scroll_by_delta(
        &mut self,
        modifier: KeyboardModifier,
        delta: i32,
        delta_type: DeltaType,
    ) -> bool {
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
        true
    }

    #[inline]
    fn slider_len(&self) -> i32 {
        let factor = if self.maximum == self.minimum {
            1.
        } else if let Some(visible_area) = self.visible_area {
            (visible_area as f32 / (self.maximum + visible_area) as f32).clamp(0.02, 1.)
        } else {
            0.15
        };

        match self.orientation {
            Orientation::Vertical => (self.size().height() as f32 * factor).max(20.) as i32,
            Orientation::Horizontal => (self.size().width() as f32 * factor).max(20.) as i32,
        }
    }

    fn calculate_slider(&mut self) -> Rect {
        let content_rect = self.contents_rect(Some(Coordinate::Widget));
        let size = content_rect.size();

        let slider_len = self.slider_len();
        let percentage = self.value as f32 / self.maximum as f32;

        match self.orientation {
            Orientation::Vertical => {
                let start_y = ((size.height() - slider_len) as f32 * percentage) as i32;

                Rect::new(content_rect.x(), start_y, size.width(), slider_len)
            }
            Orientation::Horizontal => {
                let start_x = ((size.width() - slider_len) as f32 * percentage) as i32;

                Rect::new(start_x, content_rect.y(), slider_len, size.height())
            }
        }
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

    #[inline]
    fn wheel_timer_timeout(&mut self) {
        if !self.pressed && !self.mouse_in && self.auto_hide {
            self.wheel_timer.stop();
            self.hide()
        }
    }

    #[inline]
    fn reset_wheel_timer(&mut self) {
        self.wheel_timer.stop();
        self.wheel_timer.start(Duration::from_millis(1000));
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

impl PartCovered for ScrollBar {
    #[inline]
    fn is_covered(&self) -> bool {
        self.overlaid
    }
}
