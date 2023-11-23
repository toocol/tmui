use lazy_static::lazy_static;
use log::info;
use std::time::{Duration, Instant};
use tlib::{
    connect, disconnect,
    object::{ObjectImpl, ObjectSubclass},
    timer::Timer,
};
use tmui::{application::Application, prelude::*, tlib::figure::Color, widget::WidgetImpl};

use crate::{Request, UserEvent};

lazy_static! {
    static ref COLORS: [Color; 3] = [Color::RED, Color::GREEN, Color::BLUE];
}

#[extends(Widget)]
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

        connect!(self.timer, timeout(), self, timeout());
        self.timer.start(Duration::from_secs(1));
        self.ins = Some(Instant::now());
    }
}

impl WidgetImpl for TestWidget {
    fn on_mouse_pressed(&mut self, event: &tlib::events::MouseEvent) {
        println!("Receive mouse pressed event, {:?}", event)
    }
}

impl TestWidget {
    pub fn new() -> Box<Self> {
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
            info!(
                "Request time: {}ms",
                rec.elapsed().as_micros() as f64 / 1000.
            );
        }
        self.set_background(COLORS[self.idx]);
        self.update();

        Application::<UserEvent, Request>::send_user_event(UserEvent::TestEvent(
            self.cnt,
            Instant::now(),
        ));

        self.cnt += 1;

        if self.ins.as_ref().unwrap().elapsed().as_secs() >= 10 {
            self.timer.stop();
            disconnect!(self.timer, timeout(), self, null);
        }
    }
}
