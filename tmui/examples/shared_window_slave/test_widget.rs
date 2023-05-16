use std::time::{Duration, Instant};

use tlib::{object::{ObjectImpl, ObjectSubclass}, timer::Timer, connect, disconnect};
use tmui::{prelude::*, widget::WidgetImpl, graphics::figure::Color, application::Application};
use log::debug;
use lazy_static::lazy_static;

use crate::{UserEvent, Request};

lazy_static! {
    static ref COLORS: [Color; 3] = [Color::RED, Color::GREEN, Color::BLUE];
}

#[extends(Widget)]
#[derive(Default)]
pub struct TestWidget {
    idx: usize,
    timer: Timer,
    cnt: i32,
    ins: Option<Instant>,
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
        self.ins = Some(Instant::now());

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
            let rec = Instant::now();
            let resp = Application::<UserEvent, Request>::send_request(Request::Request).unwrap();
            if let Request::Response(a) = resp {
                assert_eq!(100, a);
            }
            debug!("Request time: {}ms", rec.elapsed().as_micros() as f64 / 1000.);
        }
        self.set_background(COLORS[self.idx]);
        self.update();

        Application::<UserEvent, Request>::send_user_event(UserEvent::TestEvent(self.cnt, Instant::now()));
        self.cnt += 1;

        if self.ins.as_ref().unwrap().elapsed().as_secs() >= 10 {
            self.timer.stop();
            disconnect!(self.timer, timeout(), self, null);
        }
    }
}