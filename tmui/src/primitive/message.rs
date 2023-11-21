use std::time::Instant;
use tipc::ipc_event::IpcEvent;
use tlib::{
    events::{downcast_event, Event, EventType::*, KeyEvent, MouseEvent, ResizeEvent},
    namespace::AsNumeric,
    payload::PayloadWeight,
    prelude::SystemCursorShape,
};

#[derive(Debug)]
pub enum Message {
    /// VSync signal to redraw the window
    VSync(Instant),

    /// Set the cursor shape by user.
    SetCursorShape(SystemCursorShape),

    /// Events like MouseEvent, KeyEvent...
    Event(Event),
}

impl PayloadWeight for Message {
    #[inline]
    fn payload_wieght(&self) -> f32 {
        match self {
            Self::VSync(..) => 1.,
            Self::SetCursorShape(..) => 0.,
            Self::Event(..) => 1.,
        }
    }
}

impl<T: 'static + Copy + Sync + Send> Into<IpcEvent<T>> for Message {
    #[inline]
    fn into(self) -> IpcEvent<T> {
        match self {
            Self::VSync(a) => IpcEvent::VSync(a),
            Self::SetCursorShape(a) => IpcEvent::SetCursorShape(a),
            Self::Event(evt) => convert_event(evt),
        }
    }
}

/// Maybe it's useless, write this just in case.
#[inline]
pub(crate) fn convert_event<T: 'static + Copy + Sync + Send>(evt: Event) -> IpcEvent<T> {
    let ty = evt.event_type();
    match ty {
        Resize => {
            let evt = downcast_event::<ResizeEvent>(evt).unwrap();
            IpcEvent::ResizeEvent(evt.width(), evt.height(), Instant::now())
        }
        MouseButtonPress => {
            let evt = downcast_event::<MouseEvent>(evt).unwrap();
            let pos = evt.position();
            IpcEvent::MousePressedEvent(
                evt.n_press(),
                pos.0,
                pos.1,
                evt.mouse_button().as_numeric(),
                evt.modifier().as_numeric(),
                Instant::now(),
            )
        }
        MouseButtonRelease => {
            let evt = downcast_event::<MouseEvent>(evt).unwrap();
            let pos = evt.position();
            IpcEvent::MouseReleaseEvent(
                pos.0,
                pos.1,
                evt.mouse_button().as_numeric(),
                evt.modifier().as_numeric(),
                Instant::now(),
            )
        }
        MouseMove => {
            let evt = downcast_event::<MouseEvent>(evt).unwrap();
            let pos = evt.position();
            IpcEvent::MouseMoveEvent(pos.0, pos.1, evt.modifier().as_numeric(), Instant::now())
        }
        MouseWhell => {
            let evt = downcast_event::<MouseEvent>(evt).unwrap();
            let pos = evt.position();
            IpcEvent::MouseWheelEvent(
                pos.0,
                pos.1,
                evt.delta(),
                evt.modifier().as_numeric(),
                Instant::now(),
            )
        }
        MouseEnter => {
            let evt = downcast_event::<MouseEvent>(evt).unwrap();
            let pos = evt.position();
            IpcEvent::MouseEnterEvent(pos.0, pos.1, evt.modifier().as_numeric(), Instant::now())
        }
        MouseLeave => {
            let evt = downcast_event::<MouseEvent>(evt).unwrap();
            IpcEvent::MouseLeaveEvent(evt.modifier().as_numeric(), Instant::now())
        }
        KeyPress => {
            let evt = downcast_event::<KeyEvent>(evt).unwrap();
            IpcEvent::KeyPressedEvent(
                evt.text().to_string(),
                evt.key_code().as_numeric(),
                evt.modifier().as_numeric(),
                Instant::now(),
            )
        }
        KeyRelease => {
            let evt = downcast_event::<KeyEvent>(evt).unwrap();
            IpcEvent::KeyReleasedEvent(
                evt.text().to_string(),
                evt.key_code().as_numeric(),
                evt.modifier().as_numeric(),
                Instant::now(),
            )
        }
        FocusIn => IpcEvent::RequestFocusEvent(true, Instant::now()),
        FocusOut => IpcEvent::RequestFocusEvent(false, Instant::now()),
        _ => unreachable!(),
    }
}
