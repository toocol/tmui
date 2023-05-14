use std::time::Instant;

use tipc::ipc_event::IpcEvent;
use tlib::prelude::SystemCursorShape;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Message {
    /// VSync signal to redraw the window
    VSync(Instant),

    /// Set the cursor shape by user.
    SetCursorShape(SystemCursorShape),
}

impl<T: 'static + Copy + Sync + Send> Into<IpcEvent<T>> for Message {
    fn into(self) -> IpcEvent<T> {
        match self {
            Self::VSync(a) => IpcEvent::VSync(a),
            Self::SetCursorShape(a) => IpcEvent::SetCursorShape(a),
        }
    }
}