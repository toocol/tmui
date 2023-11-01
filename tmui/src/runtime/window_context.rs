use std::sync::mpsc::{Receiver, Sender};
use crate::{winit::{
    event_loop::{EventLoop, EventLoopProxy},
    window::Window,
}, primitive::Message};

pub(crate) enum WindowContext {
    Default(Window, EventLoop<Message>, Option<OutputSender>),
    Ipc(Receiver<Message>, Option<OutputSender>),
}

pub(crate) enum OutputSender {
    EventLoopProxy(EventLoopProxy<Message>),
    Sender(Sender<Message>),
}
