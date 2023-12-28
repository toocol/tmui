use crate::{
    application::{self, Application, APP_STOPPED},
    cursor::Cursor,
    platform::{
        ipc_window::IpcWindow,
        physical_window::{PhysWindow, PhysicalWindow},
    },
    primitive::{
        cpu_balance::CpuBalance,
        Message,
    },
    runtime::{
        runtime_track::RuntimeTrack,
        window_context::{OutputReceiver, PhysicalWindowContext},
    },
    winit::{
        self,
        event::{Event, WindowEvent},
    },
};
use log::debug;
use std::{
    marker::PhantomData,
    ops::Add,
    sync::atomic::Ordering,
    thread,
    time::{Duration, Instant},
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
        event_loop::ControlFlow,
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
        match physical_window {
            PhysicalWindow::Ipc(window) => {
                self.event_handle_ipc(window);
            }

            #[cfg(windows_platform)]
            PhysicalWindow::Win32(window) => self.event_handle(window),

            #[cfg(macos_platform)]
            PhysicalWindow::Macos(window) => self.event_handle(window),

            #[cfg(x11_platform)]
            PhysicalWindow::X11(window) => self.event_handle(window),

            #[cfg(wayland_platform)]
            PhysicalWindow::Wayland(window) => self.event_handle(window),
        };
    }

    pub fn event_handle(&self, mut window: PhysWindow<T, M>) {
        let mut runtime_track = RuntimeTrack::new();

        let (winit_window, event_loop, input_sender) = match window.context.take().unwrap() {
            PhysicalWindowContext::Default(a, b, c) => match b {
                OutputReceiver::EventLoop(d) => (a, d, c.0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        let user_ipc_event_sender = window.user_ipc_event_sender.take();

        let proxy = event_loop.create_proxy();
        let join = if let Some(master) = window.master.clone() {
            Some(
                thread::Builder::new()
                    .name("ipc-thread".to_string())
                    .spawn(move || {
                        let mut cpu_balance = CpuBalance::new();

                        loop {
                            if APP_STOPPED.load(Ordering::Acquire) {
                                break;
                            }
                            cpu_balance.loop_start();

                            let mut user_events = vec![];
                            for evt in master.read().try_recv_vec() {
                                cpu_balance.add_payload(evt.payload_wieght());

                                match evt {
                                    IpcEvent::SetCursorShape(cursor) => {
                                        proxy.send_event(Message::SetCursorShape(cursor)).unwrap();
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

                            cpu_balance.payload_check();
                            if application::is_high_load() {
                                cpu_balance.request_high_load();
                            }
                            cpu_balance.sleep(false);
                        }
                    })
                    .unwrap(),
            )
        } else {
            None
        };

        event_loop
            .run(move |event, target| {
                // Adjusting CPU usage based.
                if application::is_high_load() {
                    target.set_control_flow(ControlFlow::Poll)
                } else {
                    target.set_control_flow(ControlFlow::WaitUntil(
                        Instant::now().add(Duration::from_millis(10)),
                    ));
                }

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
                        target.set_control_flow(ControlFlow::Poll);
                        application::request_high_load(true);
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

                        target.exit();

                        APP_STOPPED.store(true, Ordering::SeqCst);
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

                        target.set_control_flow(ControlFlow::Poll)
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
                        target.set_control_flow(ControlFlow::Poll)
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
                        target.set_control_flow(ControlFlow::Poll)
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
                        target.set_control_flow(ControlFlow::Poll)
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
                        target.set_control_flow(ControlFlow::Poll)
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
                        target.set_control_flow(ControlFlow::Poll)
                    }

                    // VSync event.
                    Event::UserEvent(Message::VSync(ins)) => {
                        debug!(
                            "vscyn track: {}ms",
                            ins.elapsed().as_micros() as f64 / 1000.
                        );
                        window.request_redraw(&winit_window);
                        target.set_control_flow(ControlFlow::Poll)
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

        if let Some(join) = join {
            join.join().unwrap()
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

        let ipc_slave_clone = window.slave.clone();
        thread::Builder::new()
            .name("ipc-thread".to_string())
            .spawn(move || {
                while let Ok(message) = output_receiver.recv() {
                    if APP_STOPPED.load(Ordering::SeqCst) {
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
                        APP_STOPPED.store(true, Ordering::Release);
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

        window.slave.read().signal();
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
