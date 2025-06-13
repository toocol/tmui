use super::event::{IEvent, IEventType};

pub trait EventHandle {
    type EventType: IEventType;
    type Event: IEvent;

    fn listen(&self) -> Vec<Self::EventType>;

    fn handle_evt(&mut self, evt: &Self::Event);
}
