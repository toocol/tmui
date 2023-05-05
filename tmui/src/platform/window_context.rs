use std::sync::mpsc::{Receiver, Sender};
use winit::{
    event_loop::{EventLoop, EventLoopProxy},
    window::Window,
};
use super::Message;

pub(crate) enum WindowContext {
    Ipc(Receiver<Message>, Option<OutputSender>),
    NonIpc(Window, EventLoop<Message>, Option<OutputSender>),
}

pub(crate) enum OutputSender {
    Sender(Sender<Message>),
    EventLoopProxy(EventLoopProxy<Message>),
}