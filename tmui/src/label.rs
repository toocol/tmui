use crate::{
    graphics::painter::Painter, layout::ContentAlignment, prelude::*, skia_safe, widget::WidgetImpl,
};
use log::debug;
use tlib::{
    emit,
    object::{ObjectImpl, ObjectSubclass},
    signals,
    skia_safe::textlayout::{
        FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle, TypefaceFontProvider,
    },
};

#[extends(Widget)]
pub struct Label {
    label: String,
    content_halign: Align,
    content_valign: Align,
    #[derivative(Default(value = "Color::BLACK"))]
    color: Color,
    letter_spacing: f32,
    auto_wrap: bool,

    paragraph_width: f32,
    paragraph_height: f32,
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
    fn paint(&mut self, painter: &mut Painter) {
        let content_rect: FRect = self.contents_rect(Some(Coordinate::Widget)).into();

        painter.reset();
        painter.set_antialiasing(false);
        painter.set_color(self.color);

        let mut draw_point = content_rect.top_left();
        match self.content_halign {
            Align::Start => {}
            Align::Center => {
                let offset = (content_rect.width() - self.paragraph_width) / 2.;
                if offset > 0. {
                    draw_point.set_x(draw_point.x() + offset);
                }
            }
            Align::End => {
                let offset = content_rect.width() - self.paragraph_width;
                if offset > 0. {
                    draw_point.set_x(draw_point.x() + offset);
                }
            }
        };
        match self.content_valign {
            Align::Start => {}
            Align::Center => {
                let offset = (content_rect.height() - self.paragraph_height) / 2.;
                if offset > 0. {
                    draw_point.set_y(draw_point.y() + offset);
                }
            }
            Align::End => {
                let offset = content_rect.height() - self.paragraph_height;
                if offset > 0. {
                    draw_point.set_y(draw_point.y() + offset);
                }
            }
        };
        debug!(
            "Paint label(Widget coordinate) contents rect = {:?}, draw point = {:?}, text = {}",
            content_rect, draw_point, &self.label
        );

        let (lines, ellipsis) = if self.auto_wrap {
            (None, false)
        } else {
            (Some(1), true)
        };

        painter.save();
        painter.clip_rect(content_rect, tlib::skia_safe::ClipOp::Intersect);
        painter.draw_paragraph(
            &self.label,
            draw_point,
            self.letter_spacing,
            content_rect.width(),
            lines,
            ellipsis,
        );
        painter.restore();
    }

    fn font_changed(&mut self) {
        let font: skia_safe::Font = self.font().to_skia_font();
        let typeface = font.typeface();
        if typeface.is_none() {
            return;
        }
        let typeface = typeface.unwrap();

        let mut typeface_provider = TypefaceFontProvider::new();
        let family = typeface.family_name();
        typeface_provider.register_typeface(typeface, Some(family.clone()));

        let mut font_collection = FontCollection::new();
        font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));

        // define text style
        let mut style = ParagraphStyle::new();
        let mut text_style = TextStyle::new();
        text_style.set_font_size(font.size());
        text_style.set_font_families(&vec![family]);
        text_style.set_letter_spacing(self.letter_spacing);
        style.set_text_style(&text_style);
        if self.auto_wrap {
            style.set_max_lines(None);
            style.set_ellipsis("");
        } else {
            style.set_max_lines(Some(1));
            style.set_ellipsis("\u{2026}");
        };

        // layout the paragraph
        let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
        paragraph_builder.add_text(self.text());
        let mut paragraph = paragraph_builder.build();
        paragraph.layout(f32::MAX);

        self.paragraph_width = paragraph.max_intrinsic_width();
        self.paragraph_height = paragraph.height();

        let size = self.size();

        if size.width() == 0 || size.height() == 0 {
            self.set_fixed_width(self.paragraph_width.round() as i32);
            self.set_fixed_height(self.paragraph_height.round() as i32);
        }
        if self.window().initialized() {
            self.window().layout_change(self);
        }
    }
}

impl Label {
    #[inline]
    pub fn new(text: Option<&str>) -> Box<Self> {
        let mut label: Box<Self> = Object::new(&[]);
        if let Some(text) = text {
            label.label = text.to_string();

            label.font_changed();
        }
        label
    }

    #[inline]
    pub fn text(&self) -> &str {
        &self.label
    }

    #[inline]
    pub fn set_text(&mut self, text: &str) {
        let old = self.label.clone();
        self.label = text.to_string();
        emit!(Label::set_text => self.text_changed(), old, text);
        self.font_changed();
        self.set_rerender_styles(true);
        self.update()
    }

    #[inline]
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
        self.set_rerender_styles(true);
        self.update();
    }

    #[inline]
    pub fn set_size(&mut self, size: i32) {
        let font = self.font_mut();
        font.set_size(size as f32);
        self.font_changed();
        self.set_rerender_styles(true);
        self.update();
    }

    #[inline]
    pub fn set_letter_spacing(&mut self, letter_spacing: f32) {
        self.letter_spacing = letter_spacing;
    }

    #[inline]
    pub fn letter_spacing(&self) -> f32 {
        self.letter_spacing
    }

    #[inline]
    pub fn set_auto_wrap(&mut self, auto_wrap: bool) {
        self.auto_wrap = auto_wrap;
    }

    #[inline]
    pub fn auto_wrap(&self) -> bool {
        self.auto_wrap
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
