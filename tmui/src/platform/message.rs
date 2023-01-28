use tlib::{Value, prelude::SystemCursorShape, values::ToValue};

pub const CODE_SYSTEM: u32 = 1024;
pub const CODE_USER: u32 = 2048;

pub const CODE_VSYNC: u32 = CODE_SYSTEM + 1;
pub const CODE_SET_CURSOR_SHAPE: u32 = CODE_SYSTEM + 2;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Message(pub u32, pub Option<Value>);

unsafe impl Send for Message {}
unsafe impl Sync for Message {}

impl Message {
    pub const MESSAGE_VSNYC: Message = Message(CODE_VSYNC, None);

    pub fn message_vsync() -> Self {
        Self::MESSAGE_VSNYC
    }

    pub fn message_set_cursor_shape(cursor: SystemCursorShape) -> Self {
        Message(CODE_SET_CURSOR_SHAPE, Some(cursor.to_value()))
    }
}
