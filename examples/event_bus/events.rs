use tmui::tlib::{event_bus::event::{IEvent, IEventType}, event_bus_init};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EventType {
    Test = 0,
}
impl IEventType for EventType {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Events {
    Test,
}
impl IEvent for Events {
    type EventType = EventType;

    #[inline]
    fn ty(&self) -> EventType {
        match self {
            Self::Test => EventType::Test,
        }
    }
}

event_bus_init!(Events, EventType);
