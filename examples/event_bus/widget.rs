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

        self.set_background(Color::RED);
        self.set_borders(1., 1., 1., 1.);
        self.width_request(100);
        self.height_request(100);
    }

    fn on_drop(&mut self) {
        info!("widget drop");
        EventBus::remove(self);
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
            Events::Test => info!("Test events received. id = {}", self.id()),
        }
    }
}

impl WidgetEvents {
    #[inline]
    pub fn new() -> Tr<Self> {
        Self::new_alloc()
    }
}
