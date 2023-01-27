use std::time::Duration;

use tlib::{object::{ObjectImpl, ObjectSubclass}, timer::Timer, connect};
use tmui::{prelude::*, widget::WidgetImpl, graphics::{painter::Painter, figure::Color}};
use log::debug;

#[extends_widget]
#[derive(Default)]
pub struct TestWidget {
    num: i32,
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
        self.num = 100;
    }

    fn initialize(&mut self) {
        connect!(self.timer, timeout(), self, timeout());
        self.timer.start(Duration::from_secs(1));

        debug!("Initialize the `TestWidget`");
    }
}

impl WidgetImpl for TestWidget {
    fn paint(&mut self, mut painter: Painter) {
        debug!("Paint test widget. self rect = {:?}", self.rect());
        painter.set_antialiasing();
    }
}

impl TestWidget {
    pub fn new() -> Self {
        Object::new(&[])
    }

    pub fn timeout(&mut self) {
        self.num += 1;
    }
}