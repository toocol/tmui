use tlib::Value;

pub const CODE_SYSTEM: u32 = 1024;
pub const CODE_USER: u32 = 2048;

pub const CODE_VSYNC: u32 = CODE_SYSTEM + 1;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Message(pub u32, pub Option<Value>);

unsafe impl Send for Message {}
unsafe impl Sync for Message {}

impl Message {
    pub const MESSAGE_VSNYC: Message = Message(CODE_VSYNC, None);
}
