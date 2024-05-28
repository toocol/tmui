use log::debug;
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};
use tlib::{
    skia_safe::{
        canvas::SaveLayerRec, region::RegionOp, textlayout::TypefaceFontProvider, ClipOp, IRect,
    },
    typedef::SkiaRegion,
};
use tmui::{
    graphics::painter::Painter,
    prelude::*,
    skia_safe::{self, MaskFilter, Path},
    tlib::{
        object::{ObjectImpl, ObjectSubclass},
        typedef::{SkiaPaintStyle, SkiaRect},
    },
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
        self.set_render_styles(true);
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

        self.draw_varying_width_line(painter);

        self.draw_blur_border(painter);

        println!("cnt: {}", painter.save_count());
    }
}

impl SkiaPaint {
    fn draw_text(&self, painter: &mut Painter) {
        const TEXT: &str = "DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD";
        const REP: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#$%^&*()/\\";
        const FAMILY: &str = "Courier New";
        const FONT_SIZE: f32 = 12.;

        let mut font = Font::with_families(&[FAMILY]);
        font.set_size(FONT_SIZE);

        painter.set_font(font.clone());
        painter.set_color(Color::BLACK);

        let font = &font.to_skia_fonts()[0];

        // create font manager
        let typeface = font.typeface();
        let mut typeface_provider = TypefaceFontProvider::new();
        typeface_provider.register_typeface(typeface, Some(FAMILY));
        let mut font_collection = FontCollection::new();
        font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));

        // define text style
        let mut style = ParagraphStyle::new();
        let mut text_style = TextStyle::new();
        text_style.set_color(Color::BLACK);
        text_style.set_font_size(FONT_SIZE);
        text_style.set_font_families(&[FAMILY]);
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
        let canvas = painter.canvas_ref();
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
        painter.draw_text(TEXT, (0., 100.));

        painter.set_font(Font::with_families(&[FAMILY]));
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
        let rect = Rect::new(600, 0, 100, 40);
        painter.fill_round_rect(rect, 10., Color::CYAN);

        let (lt, rt, rb, lb) = rect.arc_points(10);
        let mut path = Path::new();
        path.move_to(lt.0);
        path.line_to(lt.1);
        path.move_to(rt.0);
        path.line_to(rt.1);
        path.move_to(rb.0);
        path.line_to(rb.1);
        path.move_to(lb.0);
        path.line_to(lb.1);

        let lt: SkiaRect = Rect::new(
            rect.left(),
            rect.top(),
            2 * 10,
            2 * 10,
        ).into();
        path.arc_to(lt, 180., 90., true);

        let rt: SkiaRect = Rect::new(
            rect.right() - 2 * 10,
            rect.top(),
            2 * 10,
            2 * 10,
        ).into();
        path.arc_to(rt, 270., 90., true);

        let rb: SkiaRect = Rect::new(
            rect.right() - 2 * 10,
            rect.bottom() - 2 * 10,
            2 * 10,
            2 * 10,
        ).into();
        path.arc_to(rb, 0., 90., true);

        let lb: SkiaRect = Rect::new(
            rect.left(),
            rect.bottom() - 2 * 10,
            2 * 10,
            2 * 10,
        ).into();
        path.arc_to(lb, 90., 90., true);

        painter.set_antialiasing(true);
        painter.set_style(SkiaPaintStyle::Stroke);
        painter.set_color(Color::BLACK);
        painter.draw_path(&path);
        painter.draw_rect(rect);

        let rect = Rect::new(600, 200, 100, 60);
        let (lt, rt, _, lb) = rect.arc_points(20);
        let oval: SkiaRect = Rect::new(
            rect.left(),
            rect.top(),
            2 * 20,
            2 * 20,
        ).into();
        painter.set_line_width(1.);
        painter.set_color(Color::RED);
        painter.draw_line(lb.1.x(), lb.1.y(), lt.0.x(), lt.0.y());
        // painter.draw_varying_arc_global(oval, 180., 90., 1., 4., 16);
        painter.draw_varying_arc_global(oval, 180., 45., 1., 2.5, 8);
        painter.set_color(Color::BLACK);
        painter.draw_varying_arc_global(oval, 225., 45., 2.5, 4., 8);
        painter.draw_line(lt.1.x(), lt.1.y(), rt.0.x(), rt.0.y());

        let rect = Rect::new(800, 200, 100, 100);
        painter.fill_round_rect(rect, 40., Color::CYAN);
        let (lt, rt, rb, lb) = rect.arc_points(40);
        painter.set_color(Color::BLACK);
        painter.set_line_width(1.);
        if rt.0.x() > lt.1.x() {
            painter.draw_line(lt.1.x(), lt.1.y(), rt.0.x(), rt.0.y());
        }
        if rb.0.y() > rt.1.y() {
            painter.draw_line(rt.1.x(), rt.1.y(), rb.0.x(), rb.0.y());
        }
        if lb.0.x() < rb.1.x() {
            painter.draw_line(rb.1.x(), rb.1.y(), lb.0.x(), lb.0.y());
        }
        if lt.0.y() < lb.1.y() {
            painter.draw_line(lb.1.x(), lb.1.y(), lt.0.x(), lt.0.y());
        }
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

    fn draw_varying_width_line(&mut self, painter: &mut Painter) {
        painter.set_color(Color::BLACK);
        painter.set_line_width(1.);
        painter.draw_line(900, 200, 950, 200);
        painter.set_line_width(10.);
        painter.draw_line(950, 204, 1000, 204);
    }

    fn draw_blur_border(&mut self, painter: &mut Painter) {
        let blur = MaskFilter::blur(skia_safe::BlurStyle::Normal, 2., None);
        let mut rect = FRect::new(1050., 200., 200., 80.);

        painter.set_color(Color::GREY_MEDIUM);
        painter.set_line_width(5.);
        painter.paint_mut().set_mask_filter(blur);
        painter.draw_rect(rect);

        painter.set_line_width(1.);
        painter.paint_mut().set_mask_filter(None);
        painter.fill_rect(rect, Color::WHITE);

        painter.set_color(Color::BLACK);
        rect.offset(-2.5, -2.5);
        rect.set_width(rect.width() + 5.);
        rect.set_height(rect.height() + 5.);
        painter.draw_rect(rect);
    }
}
