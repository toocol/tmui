use crate::{ipc_event::IpcEvent, native::IpcAdapter};
use log::warn;
use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum IpcType {
    Master,
    Slave,
}

pub(crate) struct IpcSender {
    id: i32,
    ty: IpcType,
}
impl IpcSender {
    #[inline]
    pub fn new(id: i32, ty: IpcType) -> Self {
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
            IpcType::Master => IpcAdapter::send_event_master(self.id, event.into()),
            IpcType::Slave => IpcAdapter::send_event_slave(self.id, event.into()),
        }
    }

    /// This method only support the [`IpcEvent::SharedMessage`]. <br>
    /// `NOTICE!!!` This message will blocked the thread, wait until another side was consumed and response this shared message
    #[inline]
    pub fn send_shared_message(&self, event: IpcEvent) -> Result<String, IpcError> {
        match event {
            IpcEvent::SharedMessage(msg, shared_string_type) => match self.ty {
                IpcType::Master => Ok(IpcAdapter::send_msg_master(
                    self.id,
                    &msg,
                    shared_string_type,
                )),
                IpcType::Slave => Ok(IpcAdapter::send_msg_slave(
                    self.id,
                    &msg,
                    shared_string_type,
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
    ty: IpcType,
}
impl IpcReceiver {
    #[inline]
    pub fn new(id: i32, ty: IpcType) -> Self {
        Self { id, ty }
    }

    pub fn has_event(&self) -> bool {
        match self.ty {
            IpcType::Master => IpcAdapter::master_has_event(self.id),
            _ => false
        }
    }

    /// Blocked receive the IpcEvent.
    #[inline]
    pub fn recv(&self) -> IpcEvent {
        match self.ty {
            IpcType::Master => IpcAdapter::recv_from_slave(self.id).into(),
            IpcType::Slave => IpcAdapter::recv_from_master(self.id).into(),
        }
    }

    /// Unblocked receive the IpcEvent, if there is no event,
    /// return the [`IpcEvent::None`]
    #[inline]
    pub fn try_recv(&self) -> IpcEvent {
        match self.ty {
            IpcType::Master => IpcAdapter::try_recv_from_slave(self.id).into(),
            IpcType::Slave => IpcAdapter::try_recv_from_master(self.id).into(),
        }
    }

    #[inline]
    pub fn try_recv_shared_message(&self) -> Option<String> {
        match self.ty {
            IpcType::Master => {
                if IpcAdapter::master_has_shared_msg(self.id) {
                    Some(IpcAdapter::get_shared_msg_master(self.id))
                } else {
                    None
                }
            }
            IpcType::Slave => {
                if IpcAdapter::slave_has_shared_msg(self.id) {
                    Some(IpcAdapter::get_shared_msg_slave(self.id))
                } else {
                    None
                }
            }
        }
    }
}

#[inline]
pub(crate) fn channel(id: i32, ty: IpcType) -> (IpcSender, IpcReceiver) {
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
