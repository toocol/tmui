use crate::mem::{IPC_KEY_EVT_SIZE, IPC_NATIVE_EVT_SIZE, IPC_SHARED_MSG_SIZE};

pub enum IpcEvent {
    None,
    /// (characters, key_code, modifier, timestamp)
    KeyPressedEvent(String, i32, i32, u64),
    /// (characters, key_code, modifier, timestamp)
    KeyReleasedEvent(String, i32, i32, u64),
    /// (n_press, x, y, button, modifier, timestamp)
    MousePressedEvent(i32, f64, f64, i32, i32, u64),
    /// (x, y, button, modifier, timestamp)
    MouseReleaseEvent(f64, f64, i32, i32, u64),
    /// (x, y, modifier, timestamp)
    MouseEnterEvent(f64, f64, i32, u64),
    /// (modifier, timestamp)
    MouseLeaveEvent(i32, u64),
    /// (x, y, modifier, timestamp)
    MouseMoveEvent(f64, f64, i32, u64),
    /// (x, y, amount, modifier, timestamp)
    MouseWheelEvent(f64, f64, f64, i32, u64),
    /// (is_focus, timestamp)
    RequestFocusEvent(bool, u64),
    /// (customize_content, timestamp)
    NativeEvent(String, u64),
    /// `NOTICE!!` This message will blocked the thread, until another side was consumed and response this shared message.
    /// (message, shared_string_type)
    SharedMessage(String, i32),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum InnerIpcEvent {
    None,
    /// (characters, key_code, modifier, timestamp)
    KeyPressedEvent([u8; IPC_KEY_EVT_SIZE], i32, i32, u64),
    /// (characters, key_code, modifier, timestamp)
    KeyReleasedEvent([u8; IPC_KEY_EVT_SIZE], i32, i32, u64),
    /// (n_press, x, y, button, modifier, timestamp)
    MousePressedEvent(i32, f64, f64, i32, i32, u64),
    /// (x, y, button, modifier, timestamp)
    MouseReleaseEvent(f64, f64, i32, i32, u64),
    /// (x, y, modifier, timestamp)
    MouseEnterEvent(f64, f64, i32, u64),
    /// (modifier, timestamp)
    MouseLeaveEvent(i32, u64),
    /// (x, y, modifier, timestamp)
    MouseMoveEvent(f64, f64, i32, u64),
    /// (x, y, amount, modifier, timestamp)
    MouseWheelEvent(f64, f64, f64, i32, u64),
    /// (is_focus, timestamp)
    RequestFocusEvent(bool, u64),
    /// (customize_content, timestamp)
    NativeEvent([u8; IPC_NATIVE_EVT_SIZE], u64),
    /// `NOTICE!!` This message will blocked the thread, until another side was consumed and response this shared message.
    /// (message, shared_string_type)
    SharedMessage([u8; IPC_SHARED_MSG_SIZE], i32),
}

impl Into<InnerIpcEvent> for IpcEvent {
    fn into(self) -> InnerIpcEvent {
        match self {
            Self::None => InnerIpcEvent::None,
            Self::KeyPressedEvent(a, b, c, d) => {
                let bytes = a.as_bytes();
                if bytes.len() > IPC_KEY_EVT_SIZE {
                    panic!(
                        "The input string of KeyPressedEvent exceed limit, max: {}, get: {}",
                        IPC_KEY_EVT_SIZE,
                        bytes.len()
                    )
                }
                let mut data = [0u8; IPC_KEY_EVT_SIZE];
                data[0..bytes.len()].copy_from_slice(bytes);
                InnerIpcEvent::KeyPressedEvent(data, b, c, d)
            }
            Self::KeyReleasedEvent(a, b, c, d) => {
                let bytes = a.as_bytes();
                if bytes.len() > IPC_KEY_EVT_SIZE {
                    panic!(
                        "The input string of KeyReleasedEvent exceed limit, max: {}, get: {}",
                        IPC_KEY_EVT_SIZE,
                        bytes.len()
                    )
                }
                let mut data = [0u8; IPC_KEY_EVT_SIZE];
                data[0..bytes.len()].copy_from_slice(bytes);
                InnerIpcEvent::KeyReleasedEvent(data, b, c, d)
            }
            Self::MousePressedEvent(a, b, c, d, e, f) => {
                InnerIpcEvent::MousePressedEvent(a, b, c, d, e, f)
            }
            Self::MouseReleaseEvent(a, b, c, d, e) => {
                InnerIpcEvent::MouseReleaseEvent(a, b, c, d, e)
            }
            Self::MouseEnterEvent(a, b, c, d) => InnerIpcEvent::MouseEnterEvent(a, b, c, d),
            Self::MouseLeaveEvent(a, b) => InnerIpcEvent::MouseLeaveEvent(a, b),
            Self::MouseMoveEvent(a, b, c, d) => InnerIpcEvent::MouseMoveEvent(a, b, c, d),
            Self::MouseWheelEvent(a, b, c, d, e) => InnerIpcEvent::MouseWheelEvent(a, b, c, d, e),
            Self::RequestFocusEvent(a, b) => InnerIpcEvent::RequestFocusEvent(a, b),
            Self::NativeEvent(a, b) => {
                let bytes = a.as_bytes();
                if bytes.len() > IPC_NATIVE_EVT_SIZE {
                    panic!(
                        "The input string of NativeEvent exceed limit, max: {}, get: {}",
                        IPC_NATIVE_EVT_SIZE,
                        bytes.len()
                    )
                }
                let mut data = [0u8; IPC_NATIVE_EVT_SIZE];
                data[0..bytes.len()].copy_from_slice(bytes);
                InnerIpcEvent::NativeEvent(data, b)
            }
            Self::SharedMessage(a, b) => {
                let bytes = a.as_bytes();
                if bytes.len() > IPC_SHARED_MSG_SIZE {
                    panic!(
                        "The input string of NativeEvent exceed limit, max: {}, get: {}",
                        IPC_SHARED_MSG_SIZE,
                        bytes.len()
                    )
                }
                let mut data = [0u8; IPC_SHARED_MSG_SIZE];
                data[0..bytes.len()].copy_from_slice(bytes);
                InnerIpcEvent::SharedMessage(data, b)
            }
        }
    }
}

impl Into<IpcEvent> for InnerIpcEvent {
    fn into(self) -> IpcEvent {
        match self {
            Self::None => IpcEvent::None,
            Self::KeyPressedEvent(a, b, c, d) => {
                let str = String::from_utf8_lossy(&a)
                    .trim_end_matches('\0')
                    .to_string();
                IpcEvent::KeyPressedEvent(str, b, c, d)
            }
            Self::KeyReleasedEvent(a, b, c, d) => {
                let str = String::from_utf8_lossy(&a)
                    .trim_end_matches('\0')
                    .to_string();
                IpcEvent::KeyReleasedEvent(str, b, c, d)
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
                let str = String::from_utf8_lossy(&a)
                    .trim_end_matches('\0')
                    .to_string();
                IpcEvent::NativeEvent(str, b)
            }
            Self::SharedMessage(a, b) => {
                let str = String::from_utf8_lossy(&a)
                    .trim_end_matches('\0')
                    .to_string();
                IpcEvent::SharedMessage(str, b)
            }
        }
    }
}
