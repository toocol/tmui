use crate::{
    primitive::Message,
    winit::event_loop::{EventLoop, EventLoopProxy},
};
use std::sync::mpsc::{Receiver, Sender};

pub(crate) struct PhysicalWindowContext(pub(crate) OutputReceiver, pub(crate) InputSender);

pub(crate) struct LogicWindowContext {
    pub output_sender: OutputSender,
    pub input_receiver: InputReceiver,
}

pub(crate) enum OutputSender {
    EventLoopProxy(EventLoopProxy<Message>),
    Sender(Sender<Message>),
}
pub(crate) enum OutputReceiver {
    EventLoop(Option<EventLoop<Message>>),
    Receiver(Receiver<Message>),
}

pub(crate) struct InputSender(pub Sender<Message>);
pub(crate) struct InputReceiver(pub Receiver<Message>);
