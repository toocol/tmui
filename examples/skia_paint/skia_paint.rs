use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};
use tlib::skia_safe::textlayout::TypefaceFontProvider;
use tmui::{
    prelude::*,
    skia_safe::{self},
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
pub struct SkiaPaint {}

impl ObjectSubclass for SkiaPaint {
    const NAME: &'static str = "SkiaPaint";
}

impl ObjectImpl for SkiaPaint {}

impl WidgetImpl for SkiaPaint {
    fn paint(&mut self, mut painter: tmui::graphics::painter::Painter) {
        const TEXT: &'static str = "DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD";
        const REP: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#$%^&*()/\\";
        const FAMILY: &'static str = "Courier New";
        const FONT_SIZE: f32 = 12.;

        let mut font = Font::with_family(FAMILY).to_skia_font();
        font.set_size(FONT_SIZE);

        painter.set_color(Color::BLACK);

        // create font manager
        let typeface = font.typeface().unwrap();
        let mut typeface_provider = TypefaceFontProvider::new();
        typeface_provider.register_typeface(typeface.into(), Some(FAMILY));
        let mut font_collection = FontCollection::new();
        font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));

        // define text style
        let mut style = ParagraphStyle::new();
        let mut text_style = TextStyle::new();
        text_style.set_color(Color::BLACK);
        text_style.set_font_size(FONT_SIZE);
        text_style.set_font_families(&vec![FAMILY]);
        text_style.set_letter_spacing(0.);
        style.set_text_style(&text_style);

        // layout the paragraph
        let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection.clone());
        paragraph_builder.add_text(TEXT);
        let mut paragraph = paragraph_builder.build();
        paragraph.layout(1024.0);

        // Calculate the width base on font:
        let glyphs: Vec<u16> = TEXT.encode_utf16().collect();
        let mut widths = vec![0.; TEXT.len()];
        font.get_widths(&glyphs, &mut widths);

        let measure = font.measure_str(REP, None);
        let cal_width: f32 = widths.iter().sum();
        let cal_height: f32 = measure.1.height();

        let width = paragraph.min_intrinsic_width();
        let height = paragraph.height();
        let metrics = &paragraph.get_line_metrics()[0];
        println!("{}, {}, {}", metrics.ascent, metrics.descent, metrics.baseline);

        painter.draw_rect(Rect::new(0, 0, cal_width as i32, metrics.baseline as i32));
        painter.draw_rect(Rect::new(0, 0, width as i32, height as i32));
        painter.draw_rect(Rect::new(0, 100 - cal_height as i32, cal_width as i32, cal_height as i32));

        // draw text
        let canvas = painter.canvas_mut();
        let point = skia_safe::Point::new(0.0, 0.0);
        paragraph.paint(canvas, point);

        // draw another text
        let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
        paragraph_builder.add_text(REP);
        let mut paragraph = paragraph_builder.build();
        paragraph.layout(1024.0);
        let point = skia_safe::Point::new(0.0, 40.0);
        paragraph.paint(canvas, point);

        painter.set_antialiasing(true);
        painter.set_font(font);
        painter.draw_text(TEXT, (0., 100.));

        painter.set_font(Font::with_family(FAMILY).to_skia_font());
        painter.draw_text(TEXT, (0., 200.));
    }
}
