use std::time::Duration;

use tlib::{object::{ObjectImpl, ObjectSubclass}, timer::Timer, connect};
use tmui::{prelude::*, widget::WidgetImpl, graphics::figure::Color};
use log::debug;
use lazy_static::lazy_static;

lazy_static! {
    static ref COLORS: [Color; 3] = [Color::RED, Color::GREEN, Color::BLUE];
}

#[extends_widget]
#[derive(Default)]
pub struct TestWidget {
    idx: usize,
    timer: Timer,
}

impl ObjectSubclass for TestWidget {
    const NAME: &'static str = "TestWidget";

    type Type = TestWidget;

    type ParentType = Widget;
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

impl WidgetImpl for TestWidget {}

impl TestWidget {
    pub fn new() -> Self {
        Object::new(&[])
    }

    pub fn timeout(&mut self) {
        self.idx += 1;
        if self.idx >= 3 {
            self.idx = 0;
        }
        self.set_background(COLORS[self.idx]);
        self.update();
    }
}