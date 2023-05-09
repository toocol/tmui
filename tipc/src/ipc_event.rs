use std::ffi::{c_char, CString};

pub enum IpcEvent {
    None,
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
    /// `NOTICE!!` This message will blocked the thread, until another side was consumed and response this shared message.
    /// (message, shared_string_type)
    SharedMessage(String, i32),
}

#[repr(C, i32)]
pub(crate) enum CIpcEvent {
    None = 0,
    /// (characters, key_code, modifier, timestamp)
    KeyPressedEvent(*const c_char, i32, i32, i64) = 1,
    /// (characters, key_code, modifier, timestamp)
    KeyReleasedEvent(*const c_char, i32, i32, i64) = 2,
    /// (n_press, x, y, button, modifier, timestamp)
    MousePressedEvent(i32, f64, f64, i32, i32, i64) = 3,
    /// (x, y, button, modifier, timestamp)
    MouseReleaseEvent(f64, f64, i32, i32, i64) = 4,
    /// (x, y, modifier, timestamp)
    MouseEnterEvent(f64, f64, i32, i64) = 5,
    /// (modifier, timestamp)
    MouseLeaveEvent(i32, i64) = 6,
    /// (x, y, modifier, timestamp)
    MouseMoveEvent(f64, f64, i32, i64) = 7,
    /// (x, y, amount, modifier, timestamp)
    MouseWheelEvent(f64, f64, f64, i32, i64) = 8,
    /// (is_focus, timestamp)
    RequestFocusEvent(bool, i64) = 9,
    /// (customize_content, timestamp)
    NativeEvent(*const c_char, i64) = 10,
    /// `NOTICE!!` This message will blocked the thread, until another side was consumed and response this shared message.
    /// (message, shared_string_type)
    SharedMessage(*const c_char, i32) = 11,
}

impl Into<CIpcEvent> for IpcEvent {
    fn into(self) -> CIpcEvent {
        match self {
            Self::None => CIpcEvent::None,
            Self::KeyPressedEvent(a, b, c, d) => {
                let c_str = CString::new(a).unwrap();
                CIpcEvent::KeyPressedEvent(c_str.as_ptr(), b, c, d)
            }
            Self::KeyReleasedEvent(a, b, c, d) => {
                let c_str = CString::new(a).unwrap();
                CIpcEvent::KeyReleasedEvent(c_str.as_ptr(), b, c, d)
            }
            Self::MousePressedEvent(a, b, c, d, e, f) => {
                CIpcEvent::MousePressedEvent(a, b, c, d, e, f)
            }
            Self::MouseReleaseEvent(a, b, c, d, e) => CIpcEvent::MouseReleaseEvent(a, b, c, d, e),
            Self::MouseEnterEvent(a, b, c, d) => CIpcEvent::MouseEnterEvent(a, b, c, d),
            Self::MouseLeaveEvent(a, b) => CIpcEvent::MouseLeaveEvent(a, b),
            Self::MouseMoveEvent(a, b, c, d) => CIpcEvent::MouseMoveEvent(a, b, c, d),
            Self::MouseWheelEvent(a, b, c, d, e) => CIpcEvent::MouseWheelEvent(a, b, c, d, e),
            Self::RequestFocusEvent(a, b) => CIpcEvent::RequestFocusEvent(a, b),
            Self::NativeEvent(a, b) => {
                let c_str = CString::new(a).unwrap();
                CIpcEvent::NativeEvent(c_str.as_ptr(), b)
            }
            Self::SharedMessage(a, b) => {
                let c_str = CString::new(a).unwrap();
                CIpcEvent::SharedMessage(c_str.as_ptr(), b)
            }
        }
    }
}

impl Into<IpcEvent> for CIpcEvent {
    fn into(self) -> IpcEvent {
        match self {
            Self::None => IpcEvent::None,
            Self::KeyPressedEvent(a, b, c, d) => {
                let c_str = unsafe { CString::from_raw(a as *mut c_char) };
                IpcEvent::KeyPressedEvent(c_str.to_str().unwrap().to_string(), b, c, d)
            }
            Self::KeyReleasedEvent(a, b, c, d) => {
                let c_str = unsafe { CString::from_raw(a as *mut c_char) };
                IpcEvent::KeyReleasedEvent(c_str.to_str().unwrap().to_string(), b, c, d)
            }
            Self::MousePressedEvent(a, b, c, d, e, f) => {
                IpcEvent::MousePressedEvent(a, b, c, d, e, f)
            }
            Self::MouseReleaseEvent(a, b, c, d, e) => IpcEvent::MouseReleaseEvent(a, b, c, d, e),
            Self::MouseEnterEvent(a, b, c, d) => IpcEvent::MouseEnterEvent(a, b, c, d),
            Self::MouseLeaveEvent(a, b) => IpcEvent::MouseLeaveEvent(a, b),
            Self::MouseMoveEvent(a, b, c, d) => IpcEvent::MouseMoveEvent(a, b, c, d),
            Self::MouseWheelEvent(a, b, c, d, e) => IpcEvent::MouseWheelEvent(a, b, c, d, e),
            Self::RequestFocusEvent(a, b) => IpcEvent::RequestFocusEvent(a, b),
            Self::NativeEvent(a, b) => {
                let c_str = unsafe { CString::from_raw(a as *mut c_char) };
                IpcEvent::NativeEvent(c_str.to_str().unwrap().to_string(), b)
            }
            Self::SharedMessage(a, b) => {
                let c_str = unsafe { CString::from_raw(a as *mut c_char) };
                IpcEvent::SharedMessage(c_str.to_str().unwrap().to_string(), b)
            }
        }
    }
}
