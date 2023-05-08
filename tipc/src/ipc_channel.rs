use crate::{ipc_event::IpcEvent, native::IpcAdapter};
use log::warn;
use std::{error::Error, fmt::Display};

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

    /// Unblocked send the event to another side.
    #[inline]
    pub fn send(&self, event: IpcEvent) {
        if let IpcEvent::SharedMessage(_, _) = event {
            warn!("Ignore `send()` function, `IpcEvent::SharedMessage` please use `send_with_response` function instead.");
            return;
        }
        match self.ty {
            ChannelType::Master => IpcAdapter::send_event_master(self.id, event.into()),
            ChannelType::Slave => IpcAdapter::send_event_slave(self.id, event.into()),
        }
    }

    /// This method only support the [`IpcEvent::SharedMessage`].
    /// This message will blocked the thread, wait until another side was consumed and response this shared message
    #[inline]
    pub fn send_with_response(&self, event: IpcEvent) -> Result<String, IpcError> {
        match event {
            IpcEvent::SharedMessage(msg, shared_string_type) => match self.ty {
                ChannelType::Slave => Ok(IpcAdapter::send_msg(self.id, &msg, shared_string_type)),
                _ => Err(IpcError::new(
                    "This method only support from Slave to Master for now",
                )),
            },
            _ => Err(IpcError::new(
                "This method only support the [`IpcEvent::SharedMessage`]",
            )),
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

    /// Blocked receive the IpcEvent.
    #[inline]
    pub fn recv(&self) -> IpcEvent {
        match self.ty {
            ChannelType::Master => IpcAdapter::recv_from_slave(self.id).into(),
            ChannelType::Slave => IpcAdapter::recv_from_master(self.id).into(),
        }
    }

    /// Unblocked receive the IpcEvent, if there is no event,
    /// return the [`IpcEvent::None`]
    #[inline]
    pub fn try_recv(&self) -> IpcEvent {
        match self.ty {
            ChannelType::Master => IpcAdapter::try_recv_from_slave(self.id).into(),
            ChannelType::Slave => IpcAdapter::try_recv_from_master(self.id).into(),
        }
    }
}

#[inline]
pub(crate) fn channel(id: i32, ty: ChannelType) -> (IpcSender, IpcReceiver) {
    (IpcSender::new(id, ty), IpcReceiver::new(id, ty))
}

#[derive(Debug)]
pub struct IpcError {
    msg: &'static str,
}

impl IpcError {
    pub fn new(msg: &'static str) -> Self {
        Self { msg }
    }
}

impl Error for IpcError {}

impl Display for IpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg)
    }
}
