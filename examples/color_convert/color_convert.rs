use std::time::Duration;

use tlib::{color_convert::ColorFormat, connect, timer::Timer};
use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
pub struct ColorConvert {
    timer: Timer,
    front: Box<Vec<u8>>,
    end: Box<Vec<u8>>,
    flag: bool,
}

impl ObjectSubclass for ColorConvert {
    const NAME: &'static str = "ColorConvert";
}

impl ObjectImpl for ColorConvert {
    fn construct(&mut self) {
        self.parent_construct();

        self.front = Box::new(vec![0u8; 2560 * 1440 * 4]);
        self.end = Box::new(vec![0u8; 2560 * 1440 * 4]);
        connect!(self.timer, timeout(), self, color_convert());
        self.timer.start(Duration::from_millis(16));
    }
}

impl WidgetImpl for ColorConvert {}

impl ColorConvert {
    pub fn color_convert(&mut self) {
        if self.flag {
            tlib::color_convert::ColorConvert::convert(
                &mut self.front,
                ColorFormat::Rgba8888,
                ColorFormat::Argb8888,
            );
        } else {
            tlib::color_convert::ColorConvert::convert(
                &mut self.end,
                ColorFormat::Rgba8888,
                ColorFormat::Argb8888,
            );
        }

        self.flag = !self.flag;
    }
}
