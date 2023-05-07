use crate::ipc_event::IpcEvent;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChannelType {
    Master,
    Slave,
}

pub(crate) struct IpcSender {
    id: i32,
    ty: ChannelType,
}
impl IpcSender {
    pub fn new(id: i32, ty: ChannelType) -> Self {
        Self { id, ty }
    }

    pub fn send(&self, event: IpcEvent) {}
}

pub(crate) struct IpcReceiver {
    id: i32,
    ty: ChannelType,
}
impl IpcReceiver {
    pub fn new(id: i32, ty: ChannelType) -> Self {
        Self { id, ty }
    }
}

pub(crate) fn channel(id: i32, ty: ChannelType) -> (IpcSender, IpcReceiver) {
    (IpcSender::new(id, ty), IpcReceiver::new(id, ty))
}
