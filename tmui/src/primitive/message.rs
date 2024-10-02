use crate::{application_window::ApplicationWindow, window::Window};
use std::{fmt::Debug, time::Instant};
use tipc::ipc_event::IpcEvent;
use tlib::{
    events::{downcast_event_ref, Event, EventType::*, KeyEvent, MouseEvent, ResizeEvent},
    figure::{Point, Rect, Size},
    namespace::AsNumeric,
    object::ObjectId,
    payload::PayloadWeight,
    prelude::SystemCursorShape,
    winit::window::WindowId,
};

pub(crate) enum Message {
    /// VSync signal to redraw the window
    VSync(WindowId, Instant),

    /// Set the cursor shape by user.
    SetCursorShape(SystemCursorShape, WindowId),

    /// Events like MouseEvent, KeyEvent...
    Event(Event),

    /// Create new window.
    /// (Parent window id, child window)
    CreateWindow(WindowId, Window),

    /// Window has closed.
    WindowClosed,

    /// Reqeust window to close.
    WindowCloseRequest(WindowId),

    /// Reqeust window to minimize.
    WindowMinimizeRequest(WindowId),

    /// Reqeust window to maximize.
    WindowMaximizeRequest(WindowId),

    /// Reqeust window to restore.
    WindowRestoreRequest(WindowId),

    /// Request window the set the visibility.
    WindowVisibilityRequest(WindowId, bool),

    /// Request window resize.
    WindowResizeRequest(WindowId, Size),

    /// Request window position.
    WindowPositionRequest(WindowId, Point),

    /// Sub window calling response.
    WindowResponse(
        WindowId,
        Box<dyn FnOnce(&mut ApplicationWindow) + 'static + Send + Sync>,
    ),

    /// Window has moved.
    WindowMoved(Point),

    /// Window's visibility has changed.
    WindowVisibilityChanged(bool),

    /// Request the child window correspondent to the id change the size and location.
    WinWidgetGeometryChangedRequest(ObjectId, Rect),

    /// Child window's geometry has changed, notify correspondent WinWidget change the size.
    WinWidgetGeometryReverseRequest(WindowId, Rect),

    /// Request the child window correspondent to the id change the visibility.
    WinWidgetVisibilityChangedRequest(ObjectId, bool),

    /// @see [`WinWidgetGeometryReverseRequest`](Message::WinWidgetGeometryReverseRequest)
    WinWidgetGeometryChanged(ObjectId, Rect),
}

impl Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VSync(arg0, arg1) => f.debug_tuple("VSync").field(arg0).field(arg1).finish(),
            Self::SetCursorShape(arg0, arg1) => f
                .debug_tuple("SetCursorShape")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::Event(arg0) => f.debug_tuple("Event").field(arg0).finish(),
            Self::CreateWindow(arg0, arg1) => f
                .debug_tuple("CreateWindow")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::WindowClosed => write!(f, "WindowClosed"),
            Self::WindowCloseRequest(arg0) => {
                f.debug_tuple("WindowCloseRequest").field(arg0).finish()
            }
            Self::WindowMinimizeRequest(arg0) => {
                f.debug_tuple("WindowMinimizeRequest").field(arg0).finish()
            }
            Self::WindowMaximizeRequest(arg0) => {
                f.debug_tuple("WindowMaximizeRequest").field(arg0).finish()
            }
            Self::WindowRestoreRequest(arg0) => {
                f.debug_tuple("WindowRestoreRequest").field(arg0).finish()
            }
            Self::WindowVisibilityRequest(arg0, arg1) => f
                .debug_tuple("WindowVisibilityRequest")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::WindowResizeRequest(arg0, arg1) => f
                .debug_tuple("WindowResizeRequest")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::WindowPositionRequest(arg0, arg1) => f
                .debug_tuple("WindowPositionRequest")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::WindowResponse(arg0, _) => f.debug_tuple("WindowResponse").field(arg0).finish(),
            Self::WindowMoved(arg0) => f.debug_tuple("WindowMoved").field(arg0).finish(),
            Self::WindowVisibilityChanged(arg0) => f
                .debug_tuple("WindowVisibilityChanged")
                .field(arg0)
                .finish(),
            Self::WinWidgetGeometryChangedRequest(arg0, arg1) => f
                .debug_tuple("WinWidgetGeometryChangedRequest")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::WinWidgetGeometryReverseRequest(arg0, arg1) => f
                .debug_tuple("WinWidgetGeometryReverseRequest")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::WinWidgetVisibilityChangedRequest(arg0, arg1) => f
                .debug_tuple("WinWidgetVisibilityChangedRequest")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::WinWidgetGeometryChanged(arg0, arg1) => f
                .debug_tuple("WinWidgetGeometryChanged")
                .field(arg0)
                .field(arg1)
                .finish(),
        }
    }
}

impl PayloadWeight for Message {
    #[inline]
    fn payload_wieght(&self) -> f32 {
        match self {
            Self::VSync(..) => 1.,
            Self::SetCursorShape(..) => 0.,
            Self::Event(..) => 1.,
            Self::CreateWindow(..) => 1.,
            Self::WindowClosed => 0.,
            _ => 0.,
        }
    }
}

impl<T: 'static + Copy + Sync + Send> From<Message> for IpcEvent<T> {
    #[inline]
    fn from(val: Message) -> Self {
        match val {
            Message::VSync(_, a) => IpcEvent::VSync(a),
            Message::SetCursorShape(a, _) => IpcEvent::SetCursorShape(a),
            Message::Event(evt) => convert_event(&evt),
            _ => unreachable!(),
        }
    }
}

/// Maybe it's useless, write this just in case.
#[inline]
pub(crate) fn convert_event<T: 'static + Copy + Sync + Send>(evt: &Event) -> IpcEvent<T> {
    let ty = evt.event_type();
    match ty {
        Resize => {
            let evt = downcast_event_ref::<ResizeEvent>(evt).unwrap();
            IpcEvent::ResizeEvent(evt.width(), evt.height(), Instant::now())
        }
        MouseButtonPress => {
            let evt = downcast_event_ref::<MouseEvent>(evt).unwrap();
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
            let evt = downcast_event_ref::<MouseEvent>(evt).unwrap();
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
            let evt = downcast_event_ref::<MouseEvent>(evt).unwrap();
            let pos = evt.position();
            IpcEvent::MouseMoveEvent(
                pos.0,
                pos.1,
                evt.mouse_button().as_numeric(),
                evt.modifier().as_numeric(),
                Instant::now(),
            )
        }
        MouseWhell => {
            let evt = downcast_event_ref::<MouseEvent>(evt).unwrap();
            let pos = evt.position();
            IpcEvent::MouseWheelEvent(
                pos.0,
                pos.1,
                evt.delta(),
                evt.delta_type(),
                evt.modifier().as_numeric(),
                Instant::now(),
            )
        }
        MouseEnter => {
            let evt = downcast_event_ref::<MouseEvent>(evt).unwrap();
            let pos = evt.position();
            IpcEvent::MouseEnterEvent(pos.0, pos.1, evt.modifier().as_numeric(), Instant::now())
        }
        MouseLeave => {
            let evt = downcast_event_ref::<MouseEvent>(evt).unwrap();
            IpcEvent::MouseLeaveEvent(evt.modifier().as_numeric(), Instant::now())
        }
        KeyPress => {
            let evt = downcast_event_ref::<KeyEvent>(evt).unwrap();
            IpcEvent::KeyPressedEvent(
                evt.text().to_string(),
                evt.key_code().as_numeric(),
                evt.modifier().as_numeric(),
                Instant::now(),
            )
        }
        KeyRelease => {
            let evt = downcast_event_ref::<KeyEvent>(evt).unwrap();
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
