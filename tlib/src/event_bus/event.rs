use std::fmt::Debug;
use std::hash::Hash;

pub trait IEvent {
    type EventType: IEventType;

    fn ty(&self) -> Self::EventType;
}

pub trait IEventType: Hash + Debug {}
