use log::debug;
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};
use tlib::{skia_safe::{canvas::SaveLayerRec, region::RegionOp, textlayout::TypefaceFontProvider, ClipOp, IRect}, typedef::SkiaRegion};
use tmui::{
    graphics::painter::Painter,
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

impl ObjectImpl for SkiaPaint {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_vexpand(true);
        self.set_hexpand(true);
    }
}

impl WidgetImpl for SkiaPaint {
    fn on_mouse_pressed(&mut self, _event: &tlib::events::MouseEvent) {
        self.set_rerender_styles(true);
        self.update()
    }

    fn paint(&mut self, painter: &mut tmui::graphics::painter::Painter) {
        self.draw_text(painter);

        self.draw_region_1(painter);

        self.draw_region_2(painter);

        self.draw_region_3(painter);

        self.draw_layer(painter);

        self.draw_round_rect(painter);

        self.draw_with_clip_difference(painter);

        println!("cnt: {}", painter.save_count());
    }
}

impl SkiaPaint {
    fn draw_text(&self, painter: &mut Painter) {
        const TEXT: &'static str =
            "DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD";
        const REP: &'static str =
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#$%^&*()/\\";
        const FAMILY: &'static str = "Courier New";
        const FONT_SIZE: f32 = 12.;

        let mut font = Font::with_family(FAMILY);
        font.set_size(FONT_SIZE);

        painter.set_font(font.to_skia_font());
        painter.set_color(Color::BLACK);

        let font = font.to_skia_font();

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
        debug!(
            "{}, {}, {}",
            metrics.ascent, metrics.descent, metrics.baseline
        );

        painter.draw_rect(Rect::new(0, 0, cal_width as i32, metrics.baseline as i32));
        painter.draw_rect(Rect::new(0, 0, width as i32, height as i32));
        painter.draw_rect(Rect::new(
            0,
            100 - cal_height as i32,
            cal_width as i32,
            cal_height as i32,
        ));

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

        painter.draw_paragraph(REP, (0., 300.), 0., 100., None, true);
        painter.draw_paragraph(REP, (0., 325.), 0., 100., Some(1), false);
        painter.set_color(Color::RED);
        painter.draw_paragraph(REP, (0., 350.), 5., 1024., None, true);
    }

    fn draw_region_1(&mut self, painter: &mut Painter) {
        let rect1 = skia_safe::IRect::new(0, 400, 200, 600);
        let rect2 = skia_safe::IRect::new(0, 400, 300, 700);
        let mut region = skia_safe::Region::new();
        region.op_rect(rect2, RegionOp::Union);
        region.op_rect(rect1, RegionOp::Difference);

        // painter.fill_region(&region, Color::BLACK);
        painter.save();
        painter.fill_rect(rect2, Color::BLUE);
        painter.clip_region_global(region, skia_safe::ClipOp::Intersect);
        painter.fill_rect(rect2, Color::BLACK);
        painter.restore();
    }

    fn draw_region_2(&mut self, painter: &mut Painter) {
        let rect1 = skia_safe::IRect::new(400, 400, 600, 600);
        let rect2 = skia_safe::IRect::new(450, 450, 650, 650);
        let mut region = skia_safe::Region::new();
        region.op_rect(rect2, RegionOp::Union);
        region.op_rect(rect1, RegionOp::Difference);

        let intersects: skia_safe::IRect = {
            let rect1: Rect = rect1.into();
            let rect2: Rect = rect2.into();
            rect1
                .intersects(&rect2)
                .unwrap_or((0, 0, 0, 0).into())
                .into()
        };

        let mut region_to_remove = skia_safe::Region::new();
        region_to_remove.op_rect(rect1, RegionOp::Union);
        region_to_remove.op_rect(intersects, RegionOp::Difference);

        let rect = skia_safe::IRect::new(400, 400, 650, 650);
        painter.save();
        painter.fill_rect(rect1, Color::BLUE);
        painter.clip_region_global(region, skia_safe::ClipOp::Intersect);
        painter.fill_rect(rect, Color::BLACK);
        painter.restore();

        painter.save();
        painter.clip_region_global(region_to_remove, skia_safe::ClipOp::Intersect);
        painter.fill_rect(rect, Color::RED);
        painter.restore();
    }

    fn draw_region_3(&mut self, painter: &mut Painter) {
        let rect1 = skia_safe::IRect::new(700, 400, 950, 650);
        let rect2 = skia_safe::IRect::new(700, 400, 900, 600);

        let mut region = skia_safe::Region::new();
        region.op_rect(rect2, RegionOp::Union);
        region.op_rect(rect1, RegionOp::Difference);

        painter.save();
        painter.clip_region_global(region, skia_safe::ClipOp::Intersect);
        painter.fill_rect(rect2, Color::BLACK);
        painter.restore();
    }

    fn draw_layer(&mut self, painter: &mut Painter) {
        let rect: skia_safe::Rect = Rect::new(0, 0, 100, 100).into();
        let mut layer = SaveLayerRec::default();
        layer = layer.bounds(&rect);

        painter.save_layer(&layer);

        painter.fill_rect(rect, Color::RED);
        painter.clear(Color::TRANSPARENT);
        painter.restore();
    }

    fn draw_round_rect(&mut self, painter: &mut Painter) {
        painter.fill_round_rect(Rect::new(600, 0, 100, 40), 10., Color::CYAN);
    }

    fn draw_with_clip_difference(&mut self, painter: &mut Painter) {
        painter.save();

        let mut region = SkiaRegion::new();
        let diff = Rect::new(810, 10, 40, 20);
        let rect: IRect = diff.into();
        region.op_rect(rect, RegionOp::Union);

        let diff = Rect::new(860, 10, 40, 20);
        let rect: IRect = diff.into();
        region.op_rect(rect, RegionOp::Union);

        painter.clip_region_global(region, ClipOp::Difference);

        painter.fill_rect(Rect::new(800, 0, 200, 80), Color::MAGENTA);

        painter.restore();
    }
}
