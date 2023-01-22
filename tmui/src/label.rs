use crate::{
    graphics::painter::Painter,
    prelude::*,
    widget::{WidgetImpl, WidgetSignals},
};
use log::debug;
use tlib::{
    connect,
    object::{ObjectImpl, ObjectSubclass},
};
use widestring::U16String;

#[extends_widget]
pub struct Label {
    label: Vec<u16>,
    content_halign: Align,
    content_valign: Align,
    color: Color,
}

impl Default for Label {
    fn default() -> Self {
        Self {
            label: Default::default(),
            content_halign: Default::default(),
            content_valign: Default::default(),
            color: Color::BLACK,
            widget: Default::default(),
        }
    }
}

impl ObjectSubclass for Label {
    const NAME: &'static str = "Label";

    type Type = Label;

    type ParentType = Widget;
}

impl ObjectImpl for Label {
    fn initialize(&mut self) {
        connect!(self, font_changed(), self, on_font_change());
    }
}

impl WidgetImpl for Label {
    fn paint(&mut self, mut painter: Painter) {
        let content_rect = self.contents_rect(Some(Coordinate::Widget));

        let font = self.font();
        let mut widths = vec![0f32; self.label.len()];
        font.get_widths(&self.label, &mut widths);
        let mut text_width = 0;
        let mut idx = 0;
        for i in 0..widths.len() {
            let width = widths[i] as i32;
            text_width += width;
            if text_width > content_rect.width() {
                idx = i - 1;
                break;
            } else if text_width == content_rect.width() {
                idx = i;
                break;
            } else {
                idx = i;
            }
        }
        let text = U16String::from_vec(&self.label[0..idx + 1])
            .to_string()
            .expect("`Label` encode u16 string to utf-8 string failed.");

        // TODO: deal with the content align.

        painter.reset();
        painter.set_antialiasing();
        painter.set_color(self.color);
        painter.set_font(font);

        let mut draw_point = content_rect.top_left();
        draw_point.set_y(content_rect.height());
        debug!(
            "Paint label, contents rect = {:?}, draw point = {:?}",
            content_rect, draw_point
        );
        painter.draw_text(&text, draw_point);
    }
}

impl Label {
    pub fn new(text: Option<&str>) -> Self {
        let mut label: Label = Object::new(&[]);
        if let Some(text) = text {
            label.label = U16String::from_str(text).as_slice().to_vec();

            let font = label.font();
            let mut widths = vec![0f32; label.label.len()];
            font.get_widths(&label.label, &mut widths);
            let width: f32 = widths.iter().sum();
            let height = font.metrics().1.cap_height;

            debug!("Label construct, width = {}, height = {}", width, height);
            label.width_request(width as i32 + 1);
            label.height_request(height as i32 + 1);
        }
        label
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color
    }

    fn on_font_change(&mut self) {
        debug!("`Label` font changed!")
    }
}
