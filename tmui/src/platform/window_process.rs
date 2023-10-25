use super::{Message, PlatformContext};
use crate::{
    cursor::Cursor,
    winit::{
        self,
        event::{Event, WindowEvent},
        event_loop::EventLoop,
        window::Window,
    },
};
use log::debug;
use once_cell::sync::Lazy;
use std::{
    ops::DerefMut,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, Sender},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};
use tipc::{ipc_event::IpcEvent, ipc_master::IpcMaster, ipc_slave::IpcSlave, IpcNode};
use tlib::{
    events::{DeltaType, EventType, KeyEvent, MouseEvent},
    figure::Point,
    global::to_static,
    namespace::{KeyCode, KeyboardModifier, MouseButton},
    prelude::SystemCursorShape,
    winit::{
        event::{ElementState, MouseScrollDelta},
        keyboard::{Key, ModifiersState},
    },
};

pub(crate) struct WindowProcess;

impl WindowProcess {
    pub fn new() -> Self {
        Self {}
    }

    pub fn event_handle<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send>(
        &self,
        platform_context: &'static mut dyn PlatformContext,
        window: Window,
        event_loop: EventLoop<Message>,
        ipc_master: Option<Arc<IpcMaster<T, M>>>,
        user_ipc_event_sender: Option<Sender<Vec<T>>>,
    ) {
        static mut VSYNC_REC: Lazy<Option<Instant>> = Lazy::new(|| None);
        let vsync_rec = unsafe { VSYNC_REC.deref_mut() };

        static mut UPDATE_CNT: Lazy<usize> = Lazy::new(|| 0);
        let update_cnt = unsafe { UPDATE_CNT.deref_mut() };

        static mut MODIFIER: Lazy<KeyboardModifier> = Lazy::new(|| KeyboardModifier::NoModifier);
        let modifer = unsafe { MODIFIER.deref_mut() };

        static mut MOUSE_POSITION: Lazy<(i32, i32)> = Lazy::new(|| (0, 0));
        let mouse_position = unsafe { MOUSE_POSITION.deref_mut() };

        event_loop.run(move |event, _, control_flow| {
            let input_sender = platform_context.input_sender();

            // Adjusting CPU usage based on vsync signals.
            if let Some(ins) = vsync_rec {
                if ins.elapsed().as_millis() >= 500 {
                    if *update_cnt < 15 {
                        control_flow.set_wait_timeout(Duration::from_millis(10));
                        *vsync_rec = None;
                    } else {
                        *vsync_rec = Some(Instant::now());
                    }
                    *update_cnt = 0;
                }
            } else {
                control_flow.set_wait_timeout(Duration::from_millis(10));
            }

            if let Some(master) = ipc_master.clone() {
                let mut user_events = vec![];
                for evt in master.try_recv_vec() {
                    match evt {
                        IpcEvent::VSync(ins) => {
                            debug!(
                                "Ipc vsync track: {}ms",
                                ins.elapsed().as_micros() as f64 / 1000.
                            );
                            control_flow.set_poll();
                            window.request_redraw();
                            if vsync_rec.is_none() {
                                *vsync_rec = Some(Instant::now());
                            }
                            *update_cnt += 1;
                        }
                        IpcEvent::SetCursorShape(cursor) => match cursor {
                            SystemCursorShape::BlankCursor => window.set_cursor_visible(false),
                            _ => {
                                window.set_cursor_visible(true);
                                window.set_cursor_icon(cursor.into())
                            }
                        },
                        IpcEvent::UserEvent(evt, _timestamp) => user_events.push(evt),
                        _ => {}
                    }
                }
                if user_events.len() > 0 {
                    if let Some(ref sender) = user_ipc_event_sender {
                        sender.send(user_events).unwrap();
                    }
                }
            }

            match event {
                // Window close event.
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("The close button was pressed; stopping");
                    if let Some(master) = ipc_master.clone() {
                        master.try_send(IpcEvent::Exit).unwrap();
                        platform_context.wait();
                    }
                    control_flow.set_exit();
                }

                // Window destroy event.
                Event::WindowEvent {
                    event: WindowEvent::Destroyed,
                    ..
                } => {}

                // Modifier change event.
                Event::WindowEvent {
                    event: WindowEvent::ModifiersChanged(modifier),
                    ..
                } => change_modifer(modifer, modifier.state()),

                // Mouse enter window event.
                Event::WindowEvent {
                    event: WindowEvent::CursorEntered { .. },
                    ..
                } => {}

                // Mouse leave window event.
                Event::WindowEvent {
                    event: WindowEvent::CursorLeft { .. },
                    ..
                } => {}

                // Mouse moved event.
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved { position, .. },
                    ..
                } => {
                    let evt = MouseEvent::new(
                        EventType::MouseMove,
                        (position.x as i32, position.y as i32),
                        MouseButton::NoButton,
                        *modifer,
                        0,
                        Point::default(),
                        DeltaType::default(),
                    );
                    let pos = evt.position();
                    *mouse_position = (pos.0, pos.1);
                    Cursor::set_position(pos);
                    input_sender.send(Message::Event(Box::new(evt))).unwrap();
                }

                // Mouse wheel event.
                Event::WindowEvent {
                    event: WindowEvent::MouseWheel { delta, .. },
                    ..
                } => {
                    let (point, delta_type) = match delta {
                        MouseScrollDelta::PixelDelta(pos) => {
                            (Point::new(pos.x as i32, pos.y as i32), DeltaType::Pixel)
                        }
                        MouseScrollDelta::LineDelta(x, y) => {
                            (Point::new(x as i32, y as i32), DeltaType::Line)
                        }
                    };
                    let evt = MouseEvent::new(
                        EventType::MouseWhell,
                        (mouse_position.0, mouse_position.1),
                        MouseButton::NoButton,
                        *modifer,
                        0,
                        point,
                        delta_type,
                    );

                    input_sender.send(Message::Event(Box::new(evt))).unwrap();
                }

                // Mouse pressed event.
                Event::WindowEvent {
                    event:
                        WindowEvent::MouseInput {
                            state: ElementState::Pressed,
                            button,
                            ..
                        },
                    ..
                } => {
                    let evt = MouseEvent::new(
                        EventType::MouseButtonPress,
                        (mouse_position.0, mouse_position.1),
                        button.into(),
                        *modifer,
                        1,
                        Point::default(),
                        DeltaType::default(),
                    );

                    input_sender.send(Message::Event(Box::new(evt))).unwrap();
                }

                // Mouse release event.
                Event::WindowEvent {
                    event:
                        WindowEvent::MouseInput {
                            state: ElementState::Released,
                            button,
                            ..
                        },
                    ..
                } => {
                    let evt = MouseEvent::new(
                        EventType::MouseButtonRelease,
                        (mouse_position.0, mouse_position.1),
                        button.into(),
                        *modifer,
                        1,
                        Point::default(),
                        DeltaType::default(),
                    );

                    input_sender.send(Message::Event(Box::new(evt))).unwrap();
                }

                // Key pressed event.
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            event:
                                winit::event::KeyEvent {
                                    physical_key,
                                    logical_key,
                                    state: ElementState::Pressed,
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    let key_code: KeyCode = physical_key.into();
                    let text = match logical_key {
                        Key::Character(str) => to_static(str.to_string()),
                        _ => "",
                    };
                    let evt = KeyEvent::new(EventType::KeyPress, key_code, *modifer, text);

                    input_sender.send(Message::Event(Box::new(evt))).unwrap();
                }

                // Key released event.
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            event:
                                winit::event::KeyEvent {
                                    physical_key,
                                    logical_key,
                                    state: ElementState::Released,
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    let key_code: KeyCode = physical_key.into();
                    let text = match logical_key {
                        Key::Character(str) => to_static(str.to_string()),
                        _ => "",
                    };
                    let evt = KeyEvent::new(EventType::KeyRelease, key_code, *modifer, text);

                    input_sender.send(Message::Event(Box::new(evt))).unwrap();
                }

                // Cleared event.
                Event::MainEventsCleared => {}

                // VSync event.
                Event::UserEvent(Message::VSync(ins)) => {
                    debug!(
                        "vscyn track: {}ms",
                        ins.elapsed().as_micros() as f64 / 1000.
                    );
                    window.request_redraw();
                }

                // SetCursorShape event.
                Event::UserEvent(Message::SetCursorShape(cursor)) => match cursor {
                    SystemCursorShape::BlankCursor => window.set_cursor_visible(false),
                    _ => {
                        window.set_cursor_visible(true);
                        window.set_cursor_icon(cursor.into())
                    }
                },

                // Redraw event.
                Event::RedrawRequested(_) => {
                    // Redraw the application.
                    platform_context.redraw();
                }

                _ => (),
            }
        });
    }

    pub fn event_handle_ipc<T: 'static + Copy + Sync + Send, M: 'static + Copy + Sync + Send>(
        &self,
        platform_context: &'static mut dyn PlatformContext,
        output_receiver: Receiver<Message>,
        ipc_slave: Arc<IpcSlave<T, M>>,
        user_ipc_event_sender: Option<Sender<Vec<T>>>,
    ) {
        let _input_sender = platform_context.input_sender();

        let ipc_slave_clone = ipc_slave.clone();

        static mut EXIT: AtomicBool = AtomicBool::new(false);

        thread::Builder::new()
            .name("ipc-thread".to_string())
            .spawn(move || {
                while let Ok(message) = output_receiver.recv() {
                    if unsafe { EXIT.load(Ordering::SeqCst) } {
                        return;
                    }
                    ipc_slave_clone.try_send(message.into()).unwrap()
                }
            })
            .unwrap();

        'main: loop {
            let mut user_events = vec![];
            while ipc_slave.has_event() {
                let evt = ipc_slave.try_recv().unwrap();
                match evt {
                    IpcEvent::Exit => {
                        unsafe { EXIT.store(true, Ordering::SeqCst) };
                        break 'main;
                    }
                    IpcEvent::UserEvent(evt, _timestamp) => user_events.push(evt),
                    _ => {}
                }
            }
            if user_events.len() > 0 {
                if let Some(ref sender) = user_ipc_event_sender {
                    sender.send(user_events).unwrap();
                }
            }

            std::thread::park_timeout(Duration::from_micros(10));
        }
    }
}

#[inline]
fn change_modifer(modifer: &mut KeyboardModifier, modifier_state: ModifiersState) {
    let mut state = KeyboardModifier::NoModifier;
    if modifier_state.shift_key() {
        state = state.or(KeyboardModifier::ShiftModifier);
    }
    if modifier_state.control_key() {
        state = state.or(KeyboardModifier::ControlModifier);
    }
    if modifier_state.alt_key() {
        state = state.or(KeyboardModifier::AltModifier);
    }
    if modifier_state.super_key() {
        state = state.or(KeyboardModifier::MetaModifier);
    }
    *modifer = state;
}
