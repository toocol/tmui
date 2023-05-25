use derivative::Derivative;
use once_cell::sync::Lazy;

#[derive(Derivative)]
#[derivative(Default)]
pub struct EventHints {
    #[derivative(Default(value = "300"))]
    double_click_interval: i32,
    #[derivative(Default(value = "3"))]
    wheel_scroll_lines: i32,
}

#[inline]
pub(crate) fn event_hints() -> &'static mut EventHints {
    static mut EVENT_HINTS: Lazy<EventHints> = Lazy::new(|| EventHints::default());
    unsafe { &mut EVENT_HINTS }
}

impl EventHints {
    #[inline]
    pub fn set_double_click_interval(&mut self, interval: i32) {
        self.double_click_interval = interval
    }

    #[inline]
    pub fn double_click_interval(&self) -> i32 {
        self.double_click_interval
    }

    #[inline]
    pub fn set_wheel_scroll_lines(&mut self, scroll_lines: i32) {
        self.wheel_scroll_lines = scroll_lines;
    }

    #[inline]
    pub fn wheel_scroll_lines(&self) -> i32 {
        self.wheel_scroll_lines
    }
}
