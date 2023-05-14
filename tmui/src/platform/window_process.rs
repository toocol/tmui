use std::{
    sync::{mpsc::Receiver, Arc},
    thread,
    time::Duration,
};
use super::{Message, PlatformContext};
use log::debug;
use tipc::{
    ipc_event::IpcEvent,
    ipc_master::IpcMaster,
    ipc_slave::IpcSlave,
    IpcNode,
};
use tlib::prelude::SystemCursorShape;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
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
    ) {
        let _input_sender = platform_context.input_sender();

        event_loop.run(move |event, _, control_flow| {
            // ControlFlow::Wait pauses the event loop if no events are available to process.
            // This is ideal for non-game applications that only update in response to user
            // input, and uses significantly less power/CPU time than ControlFlow::Poll.
            control_flow.set_wait_timeout(Duration::from_millis(10));

            if let Some(master) = ipc_master.clone() {
                for evt in master.try_recv_vec() {
                    match evt {
                        IpcEvent::VSync(ins) => {
                            println!("Ipc vsync track: {}ms", ins.elapsed().as_micros() as f64 / 1000.);
                            window.request_redraw();
                        }
                        _ => {}
                    }
                }
            }

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("The close button was pressed; stopping");
                    control_flow.set_exit();
                }
                Event::UserEvent(Message::VSync(ins)) => {
                    debug!(
                        "vscyn track: {}ms",
                        ins.elapsed().as_micros() as f64 / 1000.
                    );
                    window.request_redraw();
                }
                Event::MainEventsCleared => {}
                Event::UserEvent(Message::SetCursorShape(cursor)) => match cursor {
                    SystemCursorShape::BlankCursor => window.set_cursor_visible(false),
                    _ => {
                        window.set_cursor_visible(true);
                        window.set_cursor_icon(cursor.into())
                    }
                },
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
    ) {
        let _input_sender = platform_context.input_sender();

        let ipc_slave_clone = ipc_slave.clone();
        thread::Builder::new()
            .name("ipc-thread".to_string())
            .spawn(move || {
                while let Ok(message) = output_receiver.recv() {
                    ipc_slave_clone.try_send(message.into()).unwrap()
                }
            })
            .unwrap();

        'main: loop {
            while ipc_slave.has_event() {
                let evt = ipc_slave.try_recv().unwrap();
                match evt {
                    IpcEvent::Exit => {
                        break 'main;
                    }
                    _ => {}
                }
            }

            std::thread::park_timeout(Duration::from_micros(10));
        }
    }
}
