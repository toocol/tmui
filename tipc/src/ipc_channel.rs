use crate::{ipc_event::IpcEvent, native::IpcAdapter};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum ChannelType {
    Master,
    Slave,
}

pub(crate) struct IpcSender {
    id: i32,
    ty: ChannelType,
}
impl IpcSender {
    #[inline]
    pub fn new(id: i32, ty: ChannelType) -> Self {
        Self { id, ty }
    }

    #[inline]
    pub fn send(&self, event: IpcEvent) {
        match self.ty {
            ChannelType::Master => IpcAdapter::send_event_master(self.id, event),
            ChannelType::Slave => IpcAdapter::send_event_slave(self.id, event),
        }
    }
}

pub(crate) struct IpcReceiver {
    id: i32,
    ty: ChannelType,
}
impl IpcReceiver {
    #[inline]
    pub fn new(id: i32, ty: ChannelType) -> Self {
        Self { id, ty }
    }

    #[inline]
    pub fn recv(&self) -> IpcEvent {
        match self.ty {
            ChannelType::Master => IpcAdapter::recv_from_slave(self.id),
            ChannelType::Slave => IpcAdapter::recv_from_master(self.id),
        }
    }

    #[inline]
    pub fn try_recv(&self) -> IpcEvent {
        match self.ty {
            ChannelType::Master => IpcAdapter::try_recv_from_slave(self.id),
            ChannelType::Slave => IpcAdapter::try_recv_from_master(self.id),
        }
    }
}

#[inline]
pub(crate) fn channel(id: i32, ty: ChannelType) -> (IpcSender, IpcReceiver) {
    (IpcSender::new(id, ty), IpcReceiver::new(id, ty))
}
