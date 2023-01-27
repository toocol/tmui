use crate::{
    graphics::painter::Painter,
    prelude::*,
    widget::{WidgetImpl, WidgetSignals},
};
use log::debug;
use tlib::object::{ObjectImpl, ObjectSubclass};
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

impl ObjectImpl for Label {}

impl WidgetImpl for Label {
    fn size_hint(&mut self) -> Size {
        let width = self.get_property("width-request").unwrap().get::<i32>();
        let height = self.get_property("height-request").unwrap().get::<i32>();
        Size::new(width, height)
    }

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

        let draw_point = content_rect.bottom_left();
        debug!(
            "Paint label, contents rect = {:?}, draw point = {:?}",
            content_rect, draw_point
        );
        painter.draw_text(&text, draw_point);
    }

    fn font_changed(&mut self) {
        debug!("`Label` font changed.");
        let font = self.font();

        let mut widths = vec![0f32; self.label.len()];
        font.get_widths(&self.label, &mut widths);
        let width: f32 = widths.iter().sum();
        let height = font.metrics().1.cap_height;

        let size = self.size();

        if width > size.width() as f32 || height > size.height() as f32 {
            self.width_request(width as i32);
            self.height_request(height as i32 + 2);
        }
    }
}

impl Label {
    pub fn new(text: Option<&str>) -> Self {
        let mut label: Label = Object::new(&[]);
        if let Some(text) = text {
            label.label = U16String::from_str(text).as_slice().to_vec();

            label.font_changed();
        }
        label
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color
    }

    pub fn set_size(&mut self, size: i32) {
        let mut font = self.font();
        font.set_size(size as f32);
        self.set_font(font);
    }
}
