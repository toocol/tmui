use crate::{
    primitive::Message,
    winit::{
        event_loop::{EventLoop, EventLoopProxy},
        window::Window,
    },
};
use std::sync::mpsc::{Receiver, Sender};

pub(crate) enum PhysicalWindowContext {
    Default(Window, OutputReceiver, InputSender),
    Ipc(OutputReceiver, InputSender),
}

pub(crate) struct LogicWindowContext {
    pub output_sender: OutputSender,
    pub input_receiver: InputReceiver,
}

// pub(crate) enum WindowContext {
//     Default(Window, EventLoop<Message>, Option<OutputSender>),
//     Ipc(OutputReceiver, Option<OutputSender>),
// }

pub(crate) enum OutputSender {
    EventLoopProxy(EventLoopProxy<Message>),
    Sender(Sender<Message>),
}
pub(crate) enum OutputReceiver { 
    EventLoop(EventLoop<Message>),
    Receiver(Receiver<Message>),
}

pub(crate) struct InputSender(pub Sender<Message>);
pub(crate) struct InputReceiver(pub Receiver<Message>);
