use std::time::Instant;
use tlib::{
    events::{DeltaType, Event, EventType, FocusEvent, KeyEvent, MouseEvent, ResizeEvent},
    figure::Point,
    global::{to_static, SemanticExt},
    namespace::{KeyCode, KeyboardModifier, MouseButton},
    prelude::SystemCursorShape, payload::PayloadWeight,
};

pub const IPC_KEY_EVT_SIZE: usize = 8;
pub const IPC_TEXT_EVT_SIZE: usize = 4096;

pub enum IpcEvent<T: 'static + Copy> {
    None,
    Exit,
    /// (width, height, timestamp)
    ResizeEvent(i32, i32, Instant),
    /// The vsync event.
    VSync(Instant),
    /// (characters, key_code, modifier, timestamp)
    KeyPressedEvent(String, u32, u32, Instant),
    /// (characters, key_code, modifier, timestamp)
    KeyReleasedEvent(String, u32, u32, Instant),
    /// (n_press, x, y, button, modifier, timestamp)
    MousePressedEvent(i32, i32, i32, u32, u32, Instant),
    /// (x, y, button, modifier, timestamp)
    MouseReleaseEvent(i32, i32, u32, u32, Instant),
    /// (x, y, modifier, timestamp)
    MouseEnterEvent(i32, i32, u32, Instant),
    /// (modifier, timestamp)
    MouseLeaveEvent(u32, Instant),
    /// (x, y, button, modifier, timestamp)
    MouseMoveEvent(i32, i32, u32, u32, Instant),
    /// (x, y, delta, delta_type, modifier, timestamp)
    MouseWheelEvent(i32, i32, Point, DeltaType, u32, Instant),
    /// (is_focus, timestamp)
    RequestFocusEvent(bool, Instant),
    /// (system_cursor_shape)
    SetCursorShape(SystemCursorShape),
    /// (text, timestamp)
    TextEvent(String, Instant),
    /// (customize_content, timestamp)
    UserEvent(T, Instant),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum InnerIpcEvent<T: 'static + Copy> {
    None,
    Exit,
    /// (width, height, timestamp)
    ResizeEvent(i32, i32, Instant),
    /// (instant_of_vsync)
    VSync(Instant),
    /// (characters, key_code, modifier, timestamp)
    KeyPressedEvent([u8; IPC_KEY_EVT_SIZE], u32, u32, Instant),
    /// (characters, key_code, modifier, timestamp)
    KeyReleasedEvent([u8; IPC_KEY_EVT_SIZE], u32, u32, Instant),
    /// (n_press, x, y, button, modifier, timestamp)
    MousePressedEvent(i32, i32, i32, u32, u32, Instant),
    /// (x, y, button, modifier, timestamp)
    MouseReleaseEvent(i32, i32, u32, u32, Instant),
    /// (x, y, modifier, timestamp)
    MouseEnterEvent(i32, i32, u32, Instant),
    /// (modifier, timestamp)
    MouseLeaveEvent(u32, Instant),
    /// (x, y, button, modifier, timestamp)
    MouseMoveEvent(i32, i32, u32, u32, Instant),
    /// (x, y, delta, modifier, timestamp)
    MouseWheelEvent(i32, i32, Point, DeltaType, u32, Instant),
    /// (is_focus, timestamp)
    RequestFocusEvent(bool, Instant),
    /// (system_cursor_shape)
    SetCursorShape(SystemCursorShape),
    /// (text, timestamp)
    TextEvent([u8; IPC_TEXT_EVT_SIZE], Instant),
    /// (customize_content, timestamp)
    UserEvent(T, Instant),
}

impl<T: 'static + Copy> Into<InnerIpcEvent<T>> for IpcEvent<T> {
    fn into(self) -> InnerIpcEvent<T> {
        match self {
            Self::None => InnerIpcEvent::None,
            Self::Exit => InnerIpcEvent::Exit,
            Self::ResizeEvent(a, b, c) => InnerIpcEvent::ResizeEvent(a, b, c),
            Self::VSync(a) => InnerIpcEvent::VSync(a),
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
            Self::MouseMoveEvent(a, b, c, d, e) => InnerIpcEvent::MouseMoveEvent(a, b, c, d, e),
            Self::MouseWheelEvent(a, b, c, d, e, f) => {
                InnerIpcEvent::MouseWheelEvent(a, b, c, d, e, f)
            }
            Self::RequestFocusEvent(a, b) => InnerIpcEvent::RequestFocusEvent(a, b),
            Self::SetCursorShape(a) => InnerIpcEvent::SetCursorShape(a),
            Self::TextEvent(a, b) => {
                let bytes = a.as_bytes();
                if bytes.len() > IPC_TEXT_EVT_SIZE {
                    panic!(
                        "The input string of NativeEvent exceed limit, max: {}, get: {}",
                        IPC_TEXT_EVT_SIZE,
                        bytes.len()
                    )
                }
                let mut data = [0u8; IPC_TEXT_EVT_SIZE];
                data[0..bytes.len()].copy_from_slice(bytes);
                InnerIpcEvent::TextEvent(data, b)
            }
            Self::UserEvent(a, b) => InnerIpcEvent::UserEvent(a, b),
        }
    }
}

impl<T: 'static + Copy> Into<IpcEvent<T>> for InnerIpcEvent<T> {
    fn into(self) -> IpcEvent<T> {
        match self {
            Self::None => IpcEvent::None,
            Self::Exit => IpcEvent::Exit,
            Self::ResizeEvent(a, b, c) => IpcEvent::ResizeEvent(a, b, c),
            Self::VSync(a) => IpcEvent::VSync(a),
            Self::KeyPressedEvent(a, b, c, d) => {
                let str = String::from_utf8(a.to_vec())
                    .unwrap()
                    .trim_end_matches('\0')
                    .to_string();
                IpcEvent::KeyPressedEvent(str, b, c, d)
            }
            Self::KeyReleasedEvent(a, b, c, d) => {
                let str = String::from_utf8(a.to_vec())
                    .unwrap()
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
            Self::MouseMoveEvent(a, b, c, d, e) => IpcEvent::MouseMoveEvent(a, b, c, d, e),
            Self::MouseWheelEvent(a, b, c, d, e, f) => IpcEvent::MouseWheelEvent(a, b, c, d, e, f),
            Self::RequestFocusEvent(a, b) => IpcEvent::RequestFocusEvent(a, b),
            Self::SetCursorShape(a) => IpcEvent::SetCursorShape(a),
            Self::TextEvent(a, b) => {
                let str = String::from_utf8_lossy(&a)
                    .trim_end_matches('\0')
                    .to_string();
                IpcEvent::TextEvent(str, b)
            }
            Self::UserEvent(a, b) => IpcEvent::UserEvent(a, b),
        }
    }
}

impl<T: 'static + Copy> Into<Event> for IpcEvent<T> {
    fn into(self) -> Event {
        match self {
            Self::ResizeEvent(w, h, _time) => ResizeEvent::new(w, h).boxed(),
            Self::KeyPressedEvent(ch, key_code, modifier, _time) => {
                let key_code = KeyCode::from(key_code);
                let modifier = KeyboardModifier::from(modifier);
                KeyEvent::new(EventType::KeyPress, key_code, modifier, to_static(ch)).boxed()
            }
            Self::KeyReleasedEvent(ch, key_code, modifier, _time) => {
                let key_code = KeyCode::from(key_code);
                let modifier = KeyboardModifier::from(modifier);
                KeyEvent::new(EventType::KeyRelease, key_code, modifier, to_static(ch)).boxed()
            }
            Self::MousePressedEvent(n_press, x, y, button, modifier, _time) => {
                let button = MouseButton::from(button);
                let modifier = KeyboardModifier::from(modifier);
                MouseEvent::new(
                    EventType::MouseButtonPress,
                    (x, y),
                    button,
                    modifier,
                    n_press,
                    Point::default(),
                    DeltaType::default(),
                )
                .boxed()
            }
            Self::MouseReleaseEvent(x, y, button, modifier, _time) => {
                let button = MouseButton::from(button);
                let modifier = KeyboardModifier::from(modifier);
                MouseEvent::new(
                    EventType::MouseButtonRelease,
                    (x, y),
                    button,
                    modifier,
                    1,
                    Point::default(),
                    DeltaType::default(),
                )
                .boxed()
            }
            Self::MouseEnterEvent(x, y, modifier, _time) => {
                let modifier = KeyboardModifier::from(modifier);
                MouseEvent::new(
                    EventType::MouseEnter,
                    (x, y),
                    MouseButton::NoButton,
                    modifier,
                    0,
                    Point::default(),
                    DeltaType::default(),
                )
                .boxed()
            }
            Self::MouseLeaveEvent(modifier, _time) => {
                let modifier = KeyboardModifier::from(modifier);
                MouseEvent::new(
                    EventType::MouseLeave,
                    (0, 0),
                    MouseButton::NoButton,
                    modifier,
                    0,
                    Point::default(),
                    DeltaType::default(),
                )
                .boxed()
            }
            Self::MouseMoveEvent(x, y, button, modifier, _time) => {
                let button = MouseButton::from(button);
                let modifier = KeyboardModifier::from(modifier);
                MouseEvent::new(
                    EventType::MouseMove,
                    (x, y),
                    button,
                    modifier,
                    0,
                    Point::default(),
                    DeltaType::default(),
                )
                .boxed()
            }
            Self::MouseWheelEvent(x, y, delta, delta_type, modifier, _time) => {
                let modifier = KeyboardModifier::from(modifier);
                MouseEvent::new(
                    EventType::MouseWhell,
                    (x, y),
                    MouseButton::NoButton,
                    modifier,
                    0,
                    delta,
                    delta_type,
                )
                .boxed()
            }
            Self::RequestFocusEvent(is_focus, _time) => FocusEvent::new(is_focus).boxed(),
            _ => unreachable!(),
        }
    }
}

impl<T: 'static + Copy> PayloadWeight for IpcEvent<T> {
    fn payload_wieght(&self) -> f32 {
        match self {
            Self::ResizeEvent(..) => 10.,
            Self::MouseMoveEvent(..) => 0.2,
            _ => 1.,
        }
    }
}