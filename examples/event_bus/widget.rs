use crate::events::{EventBus, EventType, Events};
use log::info;
use tlib::{event_bus::event_handle::EventHandle, iter_executor};
use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{IterExecutor, WidgetImpl},
};

#[extends(Widget)]
#[iter_executor]
pub struct WidgetEvents {}

impl ObjectSubclass for WidgetEvents {
    const NAME: &'static str = "WidgetEvents";
}

impl ObjectImpl for WidgetEvents {
    fn initialize(&mut self) {
        EventBus::register(self);
    }
}

impl WidgetImpl for WidgetEvents {}

impl IterExecutor for WidgetEvents {
    fn iter_execute(&mut self) {
        EventBus::process();
    }
}

impl EventHandle for WidgetEvents {
    type EventType = EventType;
    type Event = Events;

    #[inline]
    fn listen(&self) -> Vec<Self::EventType> {
        vec![EventType::Test]
    }

    fn handle(&mut self, evt: &Self::Event) {
        match evt {
            Events::Test => info!("Test events received."),
        }
    }
}

impl WidgetEvents {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
