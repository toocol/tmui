use crate::{
    application::{self, Application},
    cursor::Cursor,
    platform::{
        ipc_window::IpcWindow,
        physical_window::PhysicalWindow,
    },
    primitive::{cpu_balance::CpuBalance, Message},
    runtime::{
        runtime_track::RuntimeTrack,
        window_context::{OutputReceiver, PhysicalWindowContext},
    },
    winit::{
        self,
        event::{Event, WindowEvent},
    },
};
use log::{debug, info};
use once_cell::sync::Lazy;
use std::{
    marker::PhantomData,
    ops::DerefMut,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Instant,
};
use tipc::{ipc_event::IpcEvent, IpcNode};
use tlib::{
    events::{DeltaType, EventType, KeyEvent, MouseEvent, ResizeEvent},
    figure::Point,
    global::to_static,
    namespace::{KeyCode, KeyboardModifier, MouseButton},
    payload::PayloadWeight,
    prelude::SystemCursorShape,
    winit::{
        event::{ElementState, MouseScrollDelta},
        keyboard::{Key, ModifiersState, PhysicalKey},
    },
};

pub(crate) struct WindowsProcess<T: 'static + Copy + Send + Sync, M: 'static + Copy + Send + Sync> {
    _holdt: PhantomData<T>,
    _holdm: PhantomData<M>,
}

impl<T: 'static + Copy + Send + Sync, M: 'static + Copy + Send + Sync> WindowsProcess<T, M> {
    #[inline]
    pub fn new() -> Self {
        Self {
            _holdt: PhantomData::default(),
            _holdm: PhantomData::default(),
        }
    }

    pub fn process(&self, physical_window: PhysicalWindow<T, M>) {
        let window = match physical_window {
            PhysicalWindow::Ipc(window) => {
                self.event_handle_ipc(window);
                None
            }

            #[cfg(windows_platform)]
            PhysicalWindow::Win32(window) => Some(window),
        };

        if let Some(mut window) = window {
            static mut RUNTIME_TRACK: Lazy<RuntimeTrack> = Lazy::new(|| RuntimeTrack::new());
            let runtime_track = unsafe { RUNTIME_TRACK.deref_mut() };

            let (winit_window, event_loop, input_sender) = match window.context.take().unwrap() {
                PhysicalWindowContext::Default(a, b, c) => {
                    match b {
                        OutputReceiver::EventLoop(d) => (a, d, c.0),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            };

            event_loop
                .run(move |event, target| {
                    if let Some(master) = window.master.clone() {
                        let mut user_events = vec![];
                        for evt in master.read().try_recv_vec() {
                            match evt {
                                IpcEvent::VSync(ins) => {
                                    info!(
                                        "Ipc vsync track: {}ms",
                                        ins.elapsed().as_micros() as f64 / 1000.
                                    );
                                    window.request_redraw(&winit_window);
                                    if runtime_track.vsync_rec.is_none() {
                                        runtime_track.vsync_rec = Some(Instant::now());
                                    }
                                    runtime_track.update_cnt += 1;
                                }
                                IpcEvent::SetCursorShape(cursor) => match cursor {
                                    SystemCursorShape::BlankCursor => {
                                        winit_window.set_cursor_visible(false)
                                    }
                                    _ => {
                                        winit_window.set_cursor_visible(true);
                                        winit_window.set_cursor_icon(cursor.into())
                                    }
                                },
                                IpcEvent::UserEvent(evt, _timestamp) => user_events.push(evt),
                                _ => {}
                            }
                        }
                        if user_events.len() > 0 {
                            if let Some(ref sender) = window.user_ipc_event_sender {
                                sender.send(user_events).unwrap();
                            }
                        }
                    }

                    // let input_sender = platform_context.input_sender();
                    match event {
                        // Window resized event.
                        Event::WindowEvent {
                            event: WindowEvent::Resized(size),
                            ..
                        } => {
                            if !Application::<T, M>::is_app_started() {
                                return;
                            }
                            let evt = ResizeEvent::new(size.width as i32, size.height as i32);
                            input_sender.send(Message::Event(Box::new(evt))).unwrap();
                        }

                        // Window close event.
                        Event::WindowEvent {
                            event: WindowEvent::CloseRequested,
                            ..
                        } => {
                            println!("The close button was pressed; stopping");
                            if let Some(master) = window.master.clone() {
                                let master_guard = master.read();
                                master_guard.try_send(IpcEvent::Exit).unwrap();
                                master_guard.wait();
                            }
                            target.exit()
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
                            let mut key_code = KeyCode::default();
                            match physical_key {
                                PhysicalKey::Code(physical_key) => key_code = physical_key.into(),
                                _ => {}
                            }
                            let text = match logical_key {
                                Key::Character(str) => to_static(str.to_string()),
                                _ => "",
                            };
                            let evt = KeyEvent::new(
                                EventType::KeyPress,
                                key_code,
                                runtime_track.modifier,
                                text,
                            );

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
                            let mut key_code = KeyCode::default();
                            match physical_key {
                                PhysicalKey::Code(physical_key) => key_code = physical_key.into(),
                                _ => {}
                            }
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

                        // VSync event.
                        Event::UserEvent(Message::VSync(ins)) => {
                            debug!(
                                "vscyn track: {}ms",
                                ins.elapsed().as_micros() as f64 / 1000.
                            );
                            window.request_redraw(&winit_window);
                        }

                        // SetCursorShape event.
                        Event::UserEvent(Message::SetCursorShape(cursor)) => match cursor {
                            SystemCursorShape::BlankCursor => winit_window.set_cursor_visible(false),
                            _ => {
                                winit_window.set_cursor_visible(true);
                                winit_window.set_cursor_icon(cursor.into())
                            }
                        },

                        // Redraw event.
                        Event::WindowEvent {
                            event: WindowEvent::RedrawRequested { .. },
                            ..
                        } => {
                            window.redraw();
                        }

                        _ => (),
                    }
                })
                .unwrap();
        }
    }

    pub fn event_handle_ipc(&self, window: IpcWindow<T, M>) {
        let (output_receiver, input_sender) = match window.context {
            PhysicalWindowContext::Ipc(a, b) => match a {
                OutputReceiver::Receiver(c) => (c, b.0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        static mut EXIT: AtomicBool = AtomicBool::new(false);

        let ipc_slave_clone = window.slave.clone();
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

        let mut cpu_balance = CpuBalance::new();

        'main: loop {
            cpu_balance.loop_start();

            let mut user_events = vec![];
            let ipc_slave = window.slave.read();

            while ipc_slave.has_event() {
                let evt = ipc_slave.try_recv().unwrap();

                cpu_balance.add_payload(evt.payload_wieght());

                match evt {
                    IpcEvent::Exit => {
                        unsafe { EXIT.store(true, Ordering::SeqCst) };
                        break 'main;
                    }
                    IpcEvent::UserEvent(evt, _timestamp) => user_events.push(evt),
                    evt => input_sender.send(Message::Event(evt.into())).unwrap(),
                }
            }

            if user_events.len() > 0 {
                // Send events receiveed from master to the ui main thread.
                window.user_ipc_event_sender.send(user_events).unwrap();
            }

            cpu_balance.payload_check();
            if application::is_high_load() {
                cpu_balance.request_high_load();
            }
            cpu_balance.sleep(false);
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
