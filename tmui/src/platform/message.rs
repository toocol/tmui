pub const CODE_SYSTEM: u32 = 1024;
pub const CODE_USER: u32 = 2048;

pub const CODE_VSYNC: u32 = CODE_SYSTEM + 1;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Message(pub u32);

unsafe impl Send for Message {}
unsafe impl Sync for Message {}

impl Message {
    pub const MESSAGE_VSNYC: Message = Message(CODE_VSYNC);
}
