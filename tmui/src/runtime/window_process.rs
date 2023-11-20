use crate::{
    cursor::Cursor,
    platform::PlatformContext,
    primitive::Message,
    runtime::runtime_track::RuntimeTrack,
    winit::{
        self,
        event::{Event, WindowEvent},
        event_loop::EventLoop,
        window::Window,
    },
};
use log::{debug, info};
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
use tipc::{ipc_event::IpcEvent, ipc_master::IpcMaster, ipc_slave::IpcSlave, IpcNode, RwLock};
use tlib::{
    events::{DeltaType, EventType, KeyEvent, MouseEvent, ResizeEvent},
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
        ipc_master: Option<Arc<RwLock<IpcMaster<T, M>>>>,
        user_ipc_event_sender: Option<Sender<Vec<T>>>,
    ) {
        static mut RUNTIME_TRACK: Lazy<RuntimeTrack> = Lazy::new(|| RuntimeTrack::new());
        let runtime_track = unsafe { RUNTIME_TRACK.deref_mut() };

        event_loop.run(move |event, _, control_flow| {
            // Adjusting CPU usage based on vsync signals.
            if let Some(ins) = runtime_track.vsync_rec {
                if ins.elapsed().as_millis() >= 500 {
                    if runtime_track.update_cnt < 15 {
                        control_flow.set_wait_timeout(Duration::from_millis(10));
                        runtime_track.vsync_rec = None;
                    } else {
                        control_flow.set_poll();
                        runtime_track.vsync_rec = Some(Instant::now());
                    }
                    runtime_track.update_cnt = 0;
                }
            } else {
                control_flow.set_wait_timeout(Duration::from_millis(10));
            }

            if let Some(master) = ipc_master.clone() {
                let mut user_events = vec![];
                for evt in master.read().try_recv_vec() {
                    match evt {
                        IpcEvent::VSync(ins) => {
                            info!(
                                "Ipc vsync track: {}ms",
                                ins.elapsed().as_micros() as f64 / 1000.
                            );
                            control_flow.set_poll();
                            platform_context.request_redraw(&window);
                            if runtime_track.vsync_rec.is_none() {
                                runtime_track.vsync_rec = Some(Instant::now());
                            }
                            runtime_track.update_cnt += 1;
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

            let input_sender = platform_context.input_sender();
            match event {
                // Window resized event.
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    let evt = ResizeEvent::new(size.width as i32, size.height as i32);
                    input_sender.send(Message::Event(Box::new(evt))).unwrap();
                }

                // Window close event.
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("The close button was pressed; stopping");
                    if let Some(master) = ipc_master.clone() {
                        master.read().try_send(IpcEvent::Exit).unwrap();
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
                } => convert_modifier(&mut runtime_track.modifier, modifier.state()),

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
                        runtime_track.mouse_button_state(),
                        runtime_track.modifier,
                        0,
                        Point::default(),
                        DeltaType::default(),
                    );

                    let pos = evt.position();
                    runtime_track.mouse_position = (pos.0, pos.1);
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
                        (
                            runtime_track.mouse_position.0,
                            runtime_track.mouse_position.1,
                        ),
                        runtime_track.mouse_button_state(),
                        runtime_track.modifier,
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
                    let mouse_button: MouseButton = button.into();
                    runtime_track.receive_mouse_click(mouse_button);

                    let evt = MouseEvent::new(
                        EventType::MouseButtonPress,
                        (
                            runtime_track.mouse_position.0,
                            runtime_track.mouse_position.1,
                        ),
                        mouse_button,
                        runtime_track.modifier,
                        runtime_track.click_count(),
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
                    let button: MouseButton = button.into();
                    runtime_track.receive_mouse_release(button);

                    let evt = MouseEvent::new(
                        EventType::MouseButtonRelease,
                        (
                            runtime_track.mouse_position.0,
                            runtime_track.mouse_position.1,
                        ),
                        button,
                        runtime_track.modifier,
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
                    let evt =
                        KeyEvent::new(EventType::KeyPress, key_code, runtime_track.modifier, text);

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
                    let evt = KeyEvent::new(
                        EventType::KeyRelease,
                        key_code,
                        runtime_track.modifier,
                        text,
                    );

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
                    platform_context.request_redraw(&window);
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
        ipc_slave: Arc<RwLock<IpcSlave<T, M>>>,
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
                    // Send to master
                    ipc_slave_clone.read().try_send(message.into()).unwrap()
                }
            })
            .unwrap();

        'main: loop {
            let mut user_events = vec![];
            let ipc_slave = ipc_slave.read();
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
                    // Send events receiveed from master to the ui main thread.
                    sender.send(user_events).unwrap();
                }
            }

            std::thread::park_timeout(Duration::from_micros(10));
            // std::thread::yield_now();
        }
    }
}

#[inline]
fn convert_modifier(modifer: &mut KeyboardModifier, modifier_state: ModifiersState) {
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
