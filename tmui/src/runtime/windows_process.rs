use crate::{
    application::{self, Application, APP_STOPPED},
    cursor::Cursor,
    opti::tracker::Tracker,
    platform::{
        ipc_window::IpcWindow,
        physical_window::{PhysWindow, PhysicalWindow},
        PlatformContext,
    },
    primitive::{cpu_balance::CpuBalance, Message},
    runtime::{runtime_track::RuntimeTrack, window_context::OutputReceiver},
    winit::{
        self,
        event::{Event, WindowEvent},
    },
};
use log::error;
use std::{
    collections::HashMap,
    marker::PhantomData,
    ops::Add,
    sync::atomic::Ordering,
    thread,
    time::{Duration, Instant},
};
use tipc::{ipc_event::IpcEvent, raw_sync::Timeout, IpcNode};
use tlib::{
    events::{DeltaType, EventType, KeyEvent, MouseEvent, ResizeEvent, WindowMaximized, WindowMinimized, WindowRestored},
    figure::Point,
    global::to_static,
    namespace::{KeyCode, KeyboardModifier, MouseButton},
    payload::PayloadWeight,
    prelude::SystemCursorShape,
    winit::{
        event::{ElementState, MouseScrollDelta},
        event_loop::{ControlFlow, EventLoopProxy, EventLoopWindowTarget},
        keyboard::{Key, ModifiersState, NamedKey, PhysicalKey},
        window::WindowId,
    },
};

pub(crate) struct WindowsProcess<
    'a,
    T: 'static + Copy + Send + Sync,
    M: 'static + Copy + Send + Sync,
> {
    _holdt: PhantomData<T>,
    _holdm: PhantomData<M>,

    ui_stack_size: usize,
    platform_context: &'a dyn PlatformContext<T, M>,

    windows: HashMap<WindowId, PhysWindow<T, M>>,
    window_extremed: HashMap<WindowId, bool>,
    main_window_id: Option<WindowId>,
    proxy: Option<EventLoopProxy<Message>>,
}

impl<'a, T: 'static + Copy + Send + Sync, M: 'static + Copy + Send + Sync>
    WindowsProcess<'a, T, M>
{
    #[inline]
    pub fn new(ui_stack_size: usize, platform_context: &'a dyn PlatformContext<T, M>) -> Self {
        Self {
            _holdt: PhantomData,
            _holdm: PhantomData,
            ui_stack_size,
            platform_context,
            windows: HashMap::new(),
            window_extremed: HashMap::new(),
            main_window_id: None,
            proxy: None,
        }
    }

    #[inline]
    fn main_window_id(&self) -> WindowId {
        self.main_window_id.unwrap()
    }

    #[inline]
    fn proxy(&self) -> EventLoopProxy<Message> {
        self.proxy.as_ref().unwrap().clone()
    }

    pub fn process(&mut self, physical_window: PhysicalWindow<T, M>) {
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

    pub fn event_handle(&mut self, mut window: PhysWindow<T, M>) {
        let mut runtime_track = RuntimeTrack::new();
        let mut ui_joins = vec![];

        let event_loop = window.take_event_loop();
        let user_ipc_event_sender = window.user_ipc_event_sender.take();

        let proxy = event_loop.create_proxy();
        let join = window.master.clone().map(|master| {
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
                        if !user_events.is_empty() {
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
                .unwrap()
        });

        self.main_window_id = Some(window.window_id());
        self.proxy = Some(event_loop.create_proxy());
        self.windows.insert(window.window_id(), window);

        event_loop
            .run(|event, target| {
                // Adjusting CPU usage.
                if application::is_high_load() {
                    target.set_control_flow(ControlFlow::Poll)
                } else {
                    target.set_control_flow(ControlFlow::WaitUntil(
                        Instant::now().add(Duration::from_millis(10)),
                    ));
                }

                #[inline]
                fn close_window<
                    T: 'static + Copy + Send + Sync,
                    M: 'static + Copy + Send + Sync,
                >(
                    window_id: WindowId,
                    main_window_id: WindowId,
                    window: &mut PhysWindow<T, M>,
                    target: &EventLoopWindowTarget<Message>,
                ) {
                    if window_id != main_window_id {
                        window.send_input(Message::WindowClosed);
                        let _ = window.take_winit_window();
                        return;
                    }

                    if let Some(ref master) = window.master {
                        let master_guard = master.read();
                        master_guard.try_send(IpcEvent::Exit).unwrap();
                        master_guard.wait(Timeout::Val(Duration::from_secs(1)));
                    }

                    target.exit();

                    APP_STOPPED.store(true, Ordering::SeqCst);
                }

                match event {
                    Event::WindowEvent { window_id, event } => {
                        let main_window_id = self.main_window_id();
                        let window = self.windows.get_mut(&window_id).unwrap_or_else(|| {
                            panic!("Can not find window with id {:?}", window_id)
                        });

                        target.set_control_flow(ControlFlow::Poll);

                        match event {
                            // Window redraw event.
                            WindowEvent::RedrawRequested => {
                                let _track = Tracker::start("physical_window_redraw");
                                window.redraw();
                            }

                            // Window resized event.
                            WindowEvent::Resized(size) => {
                                if !Application::<T, M>::is_app_started() {
                                    return;
                                }

                                if window.winit_window().is_maximized() {
                                    self.window_extremed.insert(window_id, true);
                                    window.send_input(Message::Event(Box::new(WindowMaximized::new())));
                                } else if window.winit_window().is_minimized().unwrap_or_default() {
                                    self.window_extremed.insert(window_id, true);
                                    window.send_input(Message::Event(Box::new(WindowMinimized::new())));
                                } else {
                                    let is_extremed = self.window_extremed.get(&window_id).copied().unwrap_or_default();
                                    if is_extremed {
                                        self.window_extremed.insert(window_id, false);
                                        window.send_input(Message::Event(Box::new(WindowRestored::new())));
                                    }
                                }

                                let evt = ResizeEvent::new(size.width as i32, size.height as i32);
                                window.send_input(Message::Event(Box::new(evt)));

                                application::request_high_load(true);
                            }

                            // Window close requested event.
                            WindowEvent::CloseRequested => {
                                close_window(window_id, main_window_id, window, target);
                            }

                            // Window destroy event.
                            WindowEvent::Destroyed => {
                                self.windows.remove(&window_id);
                            }

                            // Modifier change event.
                            WindowEvent::ModifiersChanged(modifier) => {
                                convert_modifier(&mut runtime_track.modifier, modifier.state())
                            }

                            // Mouse enter window event.
                            WindowEvent::CursorEntered { .. } => {}

                            // Mouse leave window event.
                            WindowEvent::CursorLeft { .. } => {}

                            // Mouse moved event.
                            WindowEvent::CursorMoved { position, .. } => {
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

                                window.send_input(Message::Event(Box::new(evt)));
                            }

                            // Mouse wheel event.
                            WindowEvent::MouseWheel { delta, .. } => {
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

                                window.send_input(Message::Event(Box::new(evt)));
                            }

                            // Mouse pressed event.
                            WindowEvent::MouseInput {
                                state: ElementState::Pressed,
                                button,
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

                                window.send_input(Message::Event(Box::new(evt)));
                            }

                            // Mouse release event.
                            WindowEvent::MouseInput {
                                state: ElementState::Released,
                                button,
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

                                window.send_input(Message::Event(Box::new(evt)));
                            }

                            // Key pressed event.
                            WindowEvent::KeyboardInput {
                                event:
                                    winit::event::KeyEvent {
                                        physical_key,
                                        logical_key,
                                        state: ElementState::Pressed,
                                        ..
                                    },
                                ..
                            } => {
                                let mut key_code = KeyCode::default();
                                if let PhysicalKey::Code(physical_key) = physical_key {
                                    key_code = physical_key.into()
                                }
                                let text = match logical_key {
                                    Key::Character(str) => to_static(str.to_string()),
                                    Key::Named(NamedKey::Space) => " ",
                                    _ => "",
                                };
                                let evt = KeyEvent::new(
                                    EventType::KeyPress,
                                    key_code,
                                    runtime_track.modifier,
                                    text,
                                );

                                window.send_input(Message::Event(Box::new(evt)));
                            }

                            // Key released event.
                            WindowEvent::KeyboardInput {
                                event:
                                    winit::event::KeyEvent {
                                        physical_key,
                                        logical_key,
                                        state: ElementState::Released,
                                        ..
                                    },
                                ..
                            } => {
                                let mut key_code = KeyCode::default();
                                if let PhysicalKey::Code(physical_key) = physical_key {
                                    key_code = physical_key.into()
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

                                window.send_input(Message::Event(Box::new(evt)));
                            }

                            _ => {}
                        }
                    }

                    // VSync event.
                    Event::UserEvent(Message::VSync(window_id, _)) => {
                        let window = self.windows.get_mut(&window_id).unwrap_or_else(|| {
                            panic!("Can not find window with id {:?}", window_id)
                        });

                        window.request_redraw();
                        target.set_control_flow(ControlFlow::Poll)
                    }

                    // SetCursorShape event.
                    Event::UserEvent(evt) => {
                        let main_window_id = self.main_window_id();

                        match evt {
                            Message::SetCursorShape(cursor) => {
                                let window_id = self.main_window_id();
                                let window =
                                    self.windows.get_mut(&window_id).unwrap_or_else(|| {
                                        panic!("Can not find window with id {:?}", window_id)
                                    });

                                match cursor {
                                    SystemCursorShape::BlankCursor => {
                                        window.winit_window().set_cursor_visible(false)
                                    }
                                    _ => {
                                        window.winit_window().set_cursor_visible(true);
                                        window.winit_window().set_cursor_icon(cursor.into())
                                    }
                                }
                            }

                            Message::CreateWindow(mut win) => {
                                let (mut logic_window, physical_window) =
                                    self.platform_context.create_window(
                                        win.take_config(),
                                        Some(target),
                                        Some(self.proxy()),
                                    );

                                logic_window.on_activate = win.take_on_activate();

                                let phys_window = physical_window.into_phys_window();

                                self.windows.insert(phys_window.window_id(), phys_window);

                                ui_joins.push(super::start_ui_runtime(
                                    win.index(),
                                    self.ui_stack_size,
                                    logic_window,
                                ));
                            }

                            Message::WindowCloseRequest(window_id) => {
                                let window =
                                    self.windows.get_mut(&window_id).unwrap_or_else(|| {
                                        panic!("Can not find window with id {:?}", window_id)
                                    });
                                close_window(window_id, main_window_id, window, target)
                            }

                            Message::WindowMaximizeRequest(window_id) => {
                                let window =
                                    self.windows.get_mut(&window_id).unwrap_or_else(|| {
                                        panic!("Can not find window with id {:?}", window_id)
                                    });
                                window.winit_window().set_maximized(true);
                                window.winit_window().set_minimized(false);
                            }

                            Message::WindowMinimizeRequest(window_id) => {
                                let window =
                                    self.windows.get_mut(&window_id).unwrap_or_else(|| {
                                        panic!("Can not find window with id {:?}", window_id)
                                    });
                                window.winit_window().set_maximized(false);
                                window.winit_window().set_minimized(true);
                            }

                            Message::WindowRestoreRequest(window_id) => {
                                let window =
                                    self.windows.get_mut(&window_id).unwrap_or_else(|| {
                                        panic!("Can not find window with id {:?}", window_id)
                                    });
                                window.winit_window().set_maximized(false);
                                window.winit_window().set_minimized(false);
                            }

                            _ => {}
                        }
                    }

                    _ => (),
                }
            })
            .unwrap();

        if let Some(join) = join {
            join.join().unwrap()
        }
        for join in ui_joins.into_iter() {
            join.join().unwrap()
        }
    }

    pub fn event_handle_ipc(&self, window: IpcWindow<T, M>) {
        let (output_receiver, input_sender) = (
            match window.context.0 {
                OutputReceiver::Receiver(c) => c,
                _ => unreachable!(),
            },
            window.context.1 .0,
        );

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
                    evt => input_sender
                        .send(Message::Event(evt.into()))
                        .unwrap_or_else(|_| {
                            error!("Error sending Message: The UI thread may have been closed.")
                        }),
                }
            }

            if !user_events.is_empty() {
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
