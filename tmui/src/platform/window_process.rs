use super::{Message, PlatformContext};
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

    pub fn event_handle(
        &self,
        platform_context: &'static mut dyn PlatformContext,
        window: Window,
        event_loop: EventLoop<Message>,
    ) {
        event_loop.run(move |event, _, control_flow| {
            // ControlFlow::Wait pauses the event loop if no events are available to process.
            // This is ideal for non-game applications that only update in response to user
            // input, and uses significantly less power/CPU time than ControlFlow::Poll.
            control_flow.set_wait();

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("The close button was pressed; stopping");
                    control_flow.set_exit();
                }
                Event::UserEvent(Message::VSync) => {
                    window.request_redraw();
                }
                Event::MainEventsCleared => {}
                Event::UserEvent(Message::SetCursorShape(cursor)) => {
                    match cursor {
                        SystemCursorShape::BlankCursor => window.set_cursor_visible(false),
                        _ => {
                            window.set_cursor_visible(true);
                            window.set_cursor_icon(cursor.into())
                        }
                    }
                }
                Event::RedrawRequested(_) => {
                    // Redraw the application.
                    platform_context.redraw();
                }
                _ => (),
            }
        });
    }
}
