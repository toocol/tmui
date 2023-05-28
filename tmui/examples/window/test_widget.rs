use std::time::Duration;

use lazy_static::lazy_static;
use log::debug;
use tlib::{
    connect, disconnect,
    object::{ObjectImpl, ObjectSubclass},
    timer::Timer,
};
use tmui::{prelude::*, tlib::figure::Color, widget::WidgetImpl};

lazy_static! {
    static ref COLORS: [Color; 3] = [Color::RED, Color::GREEN, Color::BLUE];
}

#[extends(Widget)]
#[derive(Default)]
pub struct TestWidget {
    idx: usize,
    timer: Timer,
}

impl ObjectSubclass for TestWidget {
    const NAME: &'static str = "TestWidget";
}

impl ObjectImpl for TestWidget {
    fn construct(&mut self) {
        self.parent_construct();
        self.idx = 0;
    }

    fn initialize(&mut self) {
        connect!(self.timer, timeout(), self, timeout());
        self.timer.start(Duration::from_secs(1));

        debug!("Initialize the `TestWidget`");
    }
}

impl WidgetImpl for TestWidget {
    fn mouse_pressed(&mut self, mouse_event: &tlib::events::MouseEvent) {
        println!("Mouse pressed {:?}", mouse_event.position())
    }
}

impl TestWidget {
    pub fn new() -> Self {
        Object::new(&[])
    }

    pub fn timeout(&mut self) {
        self.idx += 1;
        if self.idx >= 3 {
            self.idx = 0;
            self.timer.stop();
            disconnect!(self.timer, timeout(), null, null);
        }
        self.set_background(COLORS[self.idx]);
        self.update();

        tasync!({ debug!("timeout async") });
    }
}
