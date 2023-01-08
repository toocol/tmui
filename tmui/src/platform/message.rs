pub const CODE_USER: u32 = 1024;
pub const CODE_PIXELS_UPDATE: u32 = CODE_USER + 1;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Message(pub u32);

unsafe impl Send for Message {}
unsafe impl Sync for Message {}

impl Message {
    pub const MESSAGE_PIXELS_UPDATE: Message = Message(CODE_PIXELS_UPDATE);
}