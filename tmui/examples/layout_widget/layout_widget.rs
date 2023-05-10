#![allow(dead_code)]
use std::time::{Duration, Instant};

use log::debug;
use tlib::{connect, object::ObjectSubclass, timer::Timer, disconnect};
use tmui::{
    graphics::figure::{FontTypeface, FontWidth},
    label::Label,
    prelude::*,
};

const TEXT: [&'static str; 4] = ["Hello", "World", "Hello", "You"];

#[extends(Widget, Layout(Stack))]
#[derive(Childrenable)]
pub struct LayoutWidget {
    #[children]
    label: Label,
    timer: Timer,
    idx: usize,
    stop_instant: Instant,
    instant: Instant,
}

impl Default for LayoutWidget {
    fn default() -> Self {
        Self {
            label: Default::default(),
            timer: Default::default(),
            idx: Default::default(),
            stop_instant: Instant::now(),
            instant: Instant::now(),
            container: Default::default(),
            children: Default::default(),
        }
    }
}

impl ObjectSubclass for LayoutWidget {
    const NAME: &'static str = "LayoutWidget";
}

impl ObjectImpl for LayoutWidget {
    fn construct(&mut self) {
        self.parent_construct();

        self.label.set_background(Color::CYAN);
        let mut font = self.label.font();
        font.set_typeface(
            FontTypeface::builder()
                .family("Consolas")
                .width(FontWidth::UltraCondensed)
                .italic(true)
                .build(),
        );
        font.set_size(20.);
        self.label.set_font(font);
        self.label.set_content_halign(Align::Center);
        self.label.set_content_valign(Align::Center);
        self.label.set_halign(Align::Center);
        self.label.set_valign(Align::Center);
        self.label.width_request(200);
        self.label.height_request(100);

        self.set_background(Color::RED);
        self.set_halign(Align::Center);
        self.set_valign(Align::Center);
        self.width_request(500);
        self.height_request(300);
    }

    fn initialize(&mut self) {
        connect!(self.label, text_changed(), self, text_changed(String:0, String:1));
        connect!(self.timer, timeout(), self, change_text());
        self.label.set_text("Hello World");
        self.timer.start(Duration::from_millis(10));
    }
}

impl WidgetImpl for LayoutWidget {}

impl LayoutWidget {
    pub fn new() -> Self {
        Object::new(&[])
    }

    pub fn text_changed(&self, old: String, new: String) {
        debug!("label text changed, old: {}, new: {}", old, new)
    }

    pub fn change_text(&mut self) {
        if self.idx >= 4 {
            self.idx = 0;
        }
        debug!("Timeout change text. duration = {}ms", self.instant.elapsed().as_micros() as f32 / 1000.);
        self.instant = Instant::now();
        self.label.set_text(TEXT[self.idx]);
        self.idx += 1;

        if self.stop_instant.elapsed().as_secs() >= 15 {
            self.timer.stop();
            disconnect!(self.timer, timeout(), self, null);
        }
    }
}