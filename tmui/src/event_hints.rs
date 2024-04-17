use derivative::Derivative;
use once_cell::sync::Lazy;

#[derive(Derivative)]
#[derivative(Default)]
pub struct EventHints {
    /// The maximum time interval used to determine the number of mouse clicks.
    #[derivative(Default(value = "300"))]
    double_click_interval: i32,

    #[derivative(Default(value = "3"))]
    wheel_scroll_lines: i32,

    /// The time interval for cursor blinking
    #[derivative(Default(value = "500"))]
    cursor_blinking_time: u32,

    /// The minimum distance that the mouse must move 
    /// before the user starts the drag operation, 
    /// 
    /// representing the number of pixels that the mouse must move 
    /// to trigger the drag operation 
    #[derivative(Default(value = "10"))]
    start_drag_distance: i32,
}

#[inline]
pub(crate) fn event_hints() -> &'static mut EventHints {
    static mut EVENT_HINTS: Lazy<EventHints> = Lazy::new(EventHints::default);
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

    #[inline]
    pub fn set_cursor_blinking_time(&mut self, blinking_time_ms: u32) {
        self.cursor_blinking_time = blinking_time_ms;
    }

    #[inline]
    pub fn cursor_blinking_time(&self) -> u32 {
        self.cursor_blinking_time
    }

    #[inline]
    pub fn set_start_drag_distance(&mut self, distance: i32) {
        self.start_drag_distance = distance
    }

    #[inline]
    pub fn start_drag_distance(&self) -> i32 {
        self.start_drag_distance
    }
}
