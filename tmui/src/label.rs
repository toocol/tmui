use crate::{graphics::painter::Painter, layout::ContentAlignment, prelude::*, widget::WidgetImpl};
use log::debug;
use tlib::{
    emit,
    object::{ObjectImpl, ObjectSubclass},
    signals,
};
use widestring::U16String;

#[extends(Widget)]
pub struct Label {
    label: Vec<u16>,
    content_halign: Align,
    content_valign: Align,
    #[derivative(Default(value = "Color::BLACK"))]
    color: Color,
}

impl ObjectSubclass for Label {
    const NAME: &'static str = "Label";
}

impl ObjectImpl for Label {
    fn construct(&mut self) {
        self.parent_construct();

        self.font_changed();
    }

    fn type_register(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<Label, ReflectContentAlignment>();
    }
}

pub trait LabelSignal: ActionExt {
    signals! {
        /// Emitted when text was changed.
        /// @param old(String)
        /// @param new(String)
        text_changed();
    }
}
impl LabelSignal for Label {}

impl WidgetImpl for Label {
    fn paint(&mut self, mut painter: Painter) {
        debug!("Paint label.");
        let content_rect = self.contents_rect(Some(Coordinate::Widget));

        let font = self.font();
        let metrics = font.metrics().1;
        let mut widths = vec![0f32; self.label.len()];
        font.get_widths(&self.label, &mut widths);
        let mut text_width = 0;
        let mut idx = 0;
        for i in 0..widths.len() {
            let width = widths[i] as i32;
            text_width += width;
            if text_width > content_rect.width() {
                idx = i - 1;
                text_width -= width;
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

        painter.reset();
        painter.set_antialiasing();
        painter.set_color(self.color);
        painter.set_font(font);

        let mut draw_point = content_rect.bottom_left();
        match self.content_halign {
            Align::Start => {}
            Align::Center => {
                let offset = (content_rect.width() - text_width) / 2;
                draw_point.set_x(draw_point.x() + offset);
            }
            Align::End => {
                let offset = content_rect.width() - text_width;
                draw_point.set_x(draw_point.x() + offset);
            }
        };
        match self.content_valign {
            Align::Start => {
                let offset = content_rect.height() - metrics.cap_height as i32 - 2;
                draw_point.set_y(draw_point.y() - offset);
            }
            Align::Center => {
                let offset = (content_rect.height() - metrics.cap_height as i32) / 2;
                draw_point.set_y(draw_point.y() - offset);
            }
            Align::End => {}
        };
        debug!(
            "Paint label(Widget coordinate) contents rect = {:?}, draw point = {:?}, text = {}",
            content_rect, draw_point, &text
        );
        painter.draw_text(&text, draw_point);
    }

    fn font_changed(&mut self) {
        debug!("`Label` font changed.");
        let font = self.font();

        let mut widths = vec![0f32; self.label.len()];
        font.get_widths(&self.label, &mut widths);
        let width: f32 = widths.into_iter().sum();
        let height = font.metrics().1.cap_height;

        let size = self.size();

        if width > size.width() as f32 || height > size.height() as f32 {
            self.width_request(width as i32 + 10);
            self.height_request(height as i32 + 4);
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

    pub fn text(&self) -> String {
        U16String::from_vec(self.label.clone()).to_string().unwrap()
    }

    pub fn set_text(&mut self, text: &str) {
        let old = U16String::from_vec(&self.label[..])
            .to_string()
            .expect("`Label` encode u16 string to utf-8 string failed.");
        self.label = U16String::from_str(text).as_slice().to_vec();
        emit!(self.text_changed(), old, text);
        self.font_changed();
        self.update()
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

impl ContentAlignment for Label {
    #[inline]
    fn homogeneous(&self) -> bool {
        true
    }

    #[inline]
    fn set_homogeneous(&mut self, _: bool) {}

    #[inline]
    fn content_halign(&self) -> Align {
        self.content_halign
    }

    #[inline]
    fn content_valign(&self) -> Align {
        self.content_valign
    }

    #[inline]
    fn set_content_halign(&mut self, halign: Align) {
        self.content_halign = halign
    }

    #[inline]
    fn set_content_valign(&mut self, valign: Align) {
        self.content_valign = valign
    }
}
