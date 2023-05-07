#[repr(C, i32)]
pub enum IpcEvent {
    None = 0,
    /// (characters, key_code, modifier, timestamp)
    KeyPressedEvent(String, i32, i32, i64),
    /// (characters, key_code, modifier, timestamp)
    KeyReleasedEvent(String, i32, i32, i64),
    /// (n_press, x, y, button, modifier, timestamp)
    MousePressedEvent(i32, f64, f64, i32, i32, i64),
    /// (x, y, button, modifier, timestamp)
    MouseReleaseEvent(f64, f64, i32, i32, i64),
    /// (x, y, modifier, timestamp)
    MouseEnterEvent(f64, f64, i32, i64),
    /// (modifier, timestamp)
    MouseLeaveEvent(i32, i64),
    /// (x, y, modifier, timestamp)
    MouseMoveEvent(f64, f64, i32, i64),
    /// (x, y, amount, modifier, timestamp)
    MouseWheelEvent(f64, f64, f64, i32, i64),
    /// (is_focus, timestamp)
    RequestFocusEvent(bool, i64),
    /// (customize_content, timestamp)
    NativeEvent(String, i64),
}