use super::event::{IEvent, IEventType};

pub trait EventHandle {
    type EventType: IEventType;
    type Event: IEvent;

    fn listen(&self) -> Vec<Self::EventType>;

    fn handle(&mut self, evt: &Self::Event);
}
