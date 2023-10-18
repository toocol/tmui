use crate::{
    graphics::painter::Painter, layout::ContentAlignment, prelude::*, skia_safe, widget::WidgetImpl,
};
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
        LabelSignal:

        /// Emitted when text was changed.
        /// @param old(String)
        /// @param new(String)
        text_changed();
    }
}
impl LabelSignal for Label {}

impl WidgetImpl for Label {
    fn paint(&mut self, mut painter: Painter) {
        let content_rect = self.contents_rect(Some(Coordinate::Widget));

        let font: skia_safe::Font = self.font().to_skia_font();
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
        painter.set_antialiasing(true);
        painter.set_color(self.color);

        let measure = font.measure_str(&text, Some(painter.paint_ref())).1;
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
                let offset = content_rect.height() - measure.height() as i32 - 2;
                draw_point.set_y(draw_point.y() - offset);
            }
            Align::Center => {
                let offset = (content_rect.height() - measure.height() as i32) / 2;
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
        let font: skia_safe::Font = self.font().to_skia_font();

        let mut widths = vec![0f32; self.label.len()];
        font.get_widths(&self.label, &mut widths);
        let width: f32 = widths.into_iter().sum();
        let height = if self.label.len() == 0 {
            font.metrics().1.cap_height
        } else {
            let text = U16String::from_vec(&self.label[..])
                .to_string()
                .expect("`Label` encode u16 string to utf-8 string failed.");
            font.measure_str(&text, None).1.height()
        };

        let size = self.size();

        if width > size.width() as f32 || height > size.height() as f32 {
            self.set_fixed_width(width as i32 + 10);
            self.set_fixed_height(height as i32 + 4);
        }
    }
}

impl Label {
    pub fn new(text: Option<&str>) -> Box<Self> {
        let mut label: Box<Self> = Object::new(&[]);
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
        emit!(Label::set_text => self.text_changed(), old, text);
        self.font_changed();
        self.set_rerender_styles(true);
        self.update()
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
        self.set_rerender_styles(true);
        self.update();
    }

    pub fn set_size(&mut self, size: i32) {
        let font = self.font_mut();
        font.set_size(size as f32);
        self.font_changed();
        self.set_rerender_styles(true);
        self.update();
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
