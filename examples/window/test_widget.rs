use lazy_static::lazy_static;
use log::debug;
use std::time::Duration;
use tlib::{
    connect, disconnect,
    object::{ObjectImpl, ObjectSubclass},
    timer::Timer,
};
use tmui::{animation::frame_animator::FrameAnimator, prelude::*, primitive::frame::Frame, tlib::{figure::Color, frame_animator}, widget::WidgetImpl};

lazy_static! {
    static ref COLORS: [Color; 3] = [Color::RED, Color::GREEN, Color::BLUE];
}

#[extends(Widget)]
#[frame_animator]
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

        connect!(self.timer, timeout(), self, timeout());
        self.timer.start(Duration::from_secs(1));
    }
}

impl WidgetImpl for TestWidget {
    fn on_mouse_pressed(&mut self, event: &tlib::events::MouseEvent) {
        println!("Mouse pressed {:?}", event.position())
    }

    fn on_mouse_enter(&mut self, _: &tlib::events::MouseEvent) {
        println!("Mouse enter.")
    }

    fn on_mouse_leave(&mut self, _: &tlib::events::MouseEvent) {
        println!("Mouse leave.")
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
            self.timer.stop();
            disconnect!(self.timer, timeout(), null, null);
        }
        self.set_background(COLORS[self.idx]);
        self.update();

        async_do!({ debug!("timeout async") });
    }
}

impl FrameAnimator for TestWidget {
    #[inline]
    fn on_frame(&mut self, frame: Frame){
        println!("widget on frame: {:?}", frame)
    }
}