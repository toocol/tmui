#![allow(dead_code)]
use crate::{
    font::Font,
    skia_safe::{self, Canvas, Matrix, Paint, Path, Point},
    tlib,
    widget::WidgetImpl,
};
use ::tlib::{
    namespace::BlendMode,
    typedef::{SkiaBlendMode, SkiaFont, SkiaImage, SkiaPoint, SkiaRect},
};
use log::{error, warn};
use std::ffi::c_uint;
use tlib::{
    figure::{Color, FRect, Rect},
    skia_safe::{
        canvas::{SaveLayerRec, SrcRectConstraint},
        textlayout::{
            FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextStyle,
            TypefaceFontProvider,
        },
        IPoint,
    },
};

pub struct Painter<'a> {
    name: &'a str,
    canvas: &'a Canvas,
    paint: Paint,
    font: Option<Font>,
    color: Option<Color>,
    line_width: Option<f32>,

    skia_fonts: Vec<SkiaFont>,

    saved_font: Option<Font>,
    saved_color: Option<Color>,
    saved_line_width: Option<f32>,

    width: i32,
    height: i32,
    x_offset: i32,
    y_offset: i32,

    transform: Matrix,

    paragraph: Option<Paragraph>,
    text_style: TextStyle,
    paragraph_style: ParagraphStyle,
}

impl<'a> Painter<'a> {
    /// The constructer to build the Painter.
    #[inline]
    pub fn new(name: &'a str, canvas: &'a Canvas, widget: &dyn WidgetImpl) -> Painter<'a> {
        let rect = widget.rect();
        let mut paint = Paint::default();
        paint.set_blend_mode(SkiaBlendMode::Src);

        Painter {
            name,
            canvas,
            paint,
            font: None,
            color: None,
            line_width: None,
            skia_fonts: vec![],
            saved_font: None,
            saved_color: None,
            saved_line_width: None,
            width: rect.width(),
            height: rect.height(),
            x_offset: rect.x(),
            y_offset: rect.y(),
            transform: Matrix::new_identity(),
            paragraph: None,
            text_style: TextStyle::new(),
            paragraph_style: ParagraphStyle::new(),
        }
    }

    #[inline]
    pub fn paint_ref(&self) -> &Paint {
        &self.paint
    }

    #[inline]
    pub fn paint_mut(&mut self) -> &mut Paint {
        &mut self.paint
    }

    #[inline]
    pub fn canvas_ref(&self) -> &Canvas {
        self.canvas
    }

    #[inline]
    pub fn offset_rect(&self, rect: &mut Rect) {
        rect.offset(self.x_offset, self.y_offset)
    }

    #[inline]
    pub fn set_transform(&mut self, transform: Matrix, combined: bool) {
        if combined {
            self.transform = self.transform * transform;
        } else {
            self.transform = transform
        }
        self.canvas.set_matrix(&self.transform.into());
    }

    /// Save the canvas status.
    ///
    /// Return the save count.
    #[inline]
    pub fn save(&self) -> usize {
        self.canvas.save()
    }

    /// Get canvas save count.
    #[inline]
    pub fn save_count(&self) -> usize {
        self.canvas.save_count()
    }

    #[inline]
    pub fn set_blend_mode(&mut self, blend_mode: BlendMode) {
        self.paint.set_blend_mode(blend_mode.into());
    }

    /// Save the pen status: Color, Font, line width etc...
    #[inline]
    pub fn save_pen(&mut self) {
        if let Some(ref font) = self.font {
            self.saved_font = Some(font.clone())
        }
        self.saved_color = self.color;
        self.saved_line_width = self.line_width;
    }

    /// Restore the pen status
    #[inline]
    pub fn restore_pen(&mut self) {
        if let Some(font) = self.saved_font.take() {
            self.set_font(font)
        }
        if let Some(color) = self.saved_color.take() {
            self.set_color(color)
        }
        if let Some(line_width) = self.saved_line_width.take() {
            self.set_line_width(line_width)
        }
    }

    /// Restore the canvas status.
    #[inline]
    pub fn restore(&self) {
        self.canvas.restore();
    }

    #[inline]
    pub fn restore_to_count(&self, count: usize) {
        self.canvas.restore_to_count(count);
    }

    #[inline]
    pub fn save_layer(&self, layer: &SaveLayerRec) -> usize {
        self.canvas.save_layer(layer)
    }

    #[inline]
    pub fn save_layer_alpha<T: Into<Option<skia_safe::Rect>>>(
        &self,
        layer: T,
        alpha: u8,
    ) -> usize {
        self.canvas.save_layer_alpha(layer, alpha as c_uint)
    }

    /// Reset the painter to Initial state.
    #[inline]
    pub fn reset(&mut self) {
        self.paint.reset();
    }

    #[inline]
    pub fn reset_matrix(&self) {
        self.canvas.reset_matrix();
    }

    /// Set the antialiasing to true.
    #[inline]
    pub fn set_antialiasing(&mut self, anti_alias: bool) {
        self.paint.set_anti_alias(anti_alias);
    }

    /// Set the global transform of this painter.
    #[inline]
    pub fn scale(&self, sx: i32, sy: i32) {
        self.canvas.scale((sx as f32, sy as f32));
    }

    #[inline]
    pub fn translate(&self, dx: f32, dy: f32) {
        self.canvas.translate((dx, dy));
    }

    /// Set the font of painter.
    #[inline]
    pub fn set_font(&mut self, font: Font) {
        let mut families = vec![];
        font.typefaces().iter().for_each(|tf| {
            families.push(tf.family());
        });

        self.text_style.set_font_families(&families);
        self.text_style.set_font_size(font.size());
        self.text_style.set_font_style(font.get_skia_font_style());

        self.skia_fonts = font.to_skia_fonts();
        self.font = Some(font);
    }

    /// Set the color of painter.
    #[inline]
    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
        self.paint.set_color(color);
        self.text_style.set_color(color);
    }

    #[inline]
    pub fn set_style(&mut self, style: crate::skia_safe::paint::Style) {
        self.paint.set_style(style);
    }

    /// Get the stroke width of painter.
    #[inline]
    pub fn line_width(&self) -> f32 {
        self.paint.stroke_width()
    }

    /// Set the stroke width of painter.
    #[inline]
    pub fn set_line_width(&mut self, width: f32) {
        self.line_width = Some(width);
        self.paint.set_stroke_width(width);
    }

    /// Set the stroke cap.
    #[inline]
    pub fn set_stroke_cap(&mut self, cap: crate::skia_safe::PaintCap) {
        self.paint.set_stroke_cap(cap);
    }

    /// Clear the canvas with the specified color.
    #[inline]
    pub fn clear(&self, color: Color) {
        self.canvas.clear(color);
    }

    /// Stroke and fill the specified Rect with offset. <br>
    ///
    /// the point of `Rect`'s coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn fill_rect<T: Into<SkiaRect>>(&mut self, rect: T, color: Color) {
        let mut rect: SkiaRect = rect.into();
        rect.offset((self.x_offset, self.y_offset));

        self.fill_rect_global(rect, color)
    }

    /// Stroke and fill the specified Rect without offset. <br>
    ///
    /// the point of `Rect`'s coordinate must be [`Coordinate::World`](tlib::namespace::Coordinate::World)
    #[inline]
    pub fn fill_rect_global<T: Into<SkiaRect>>(&mut self, rect: T, color: Color) {
        self.paint.set_color(color);
        self.paint.set_style(crate::skia_safe::PaintStyle::Fill);

        let rect: SkiaRect = rect.into();

        self.canvas.draw_rect(rect, &self.paint);
        if let Some(color) = self.color {
            self.paint.set_color(color);
        }
    }

    /// Stroke and Fill the specified rect with border radius and offset. <br>
    ///
    /// the point of `Rect`'s coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn fill_round_rect<T: Into<SkiaRect>>(
        &mut self,
        rect: T,
        border_radius: f32,
        color: Color,
    ) {
        let mut rect: SkiaRect = rect.into();
        rect.offset((self.x_offset, self.y_offset));

        self.fill_round_rect_global(rect, border_radius, color);
    }

    /// Stroke and Fill the specified rect with border radius. <br>
    ///
    /// the point of `Rect`'s coordinate must be [`Coordinate::World`](tlib::namespace::Coordinate::World)
    #[inline]
    pub fn fill_round_rect_global<T: Into<SkiaRect>>(
        &mut self,
        rect: T,
        border_radius: f32,
        color: Color,
    ) {
        self.paint.set_color(color);
        self.paint.set_style(crate::skia_safe::PaintStyle::Fill);

        let rect: SkiaRect = rect.into();

        let rrect = crate::skia_safe::RRect::new_rect_xy(rect, border_radius, border_radius);
        self.canvas.draw_rrect(rrect, &self.paint);
        if let Some(color) = self.color {
            self.paint.set_color(color);
        }
    }

    /// Stroke the specified rect with offset. <br>
    ///
    /// the point of `Rect`'s coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_rect<T: Into<SkiaRect>>(&mut self, rect: T) {
        let mut rect: SkiaRect = rect.into();
        rect.offset((self.x_offset, self.y_offset));

        self.draw_rect_global(rect);
    }

    /// Stroke the specified rect without offset. <br>
    ///
    /// the point of `Rect`'s coordinate must be [`Coordinate::World`](tlib::namespace::Coordinate::World)
    #[inline]
    pub fn draw_rect_global<T: Into<SkiaRect>>(&mut self, rect: T) {
        self.paint.set_style(crate::skia_safe::PaintStyle::Stroke);
        let rect: SkiaRect = rect.into();

        self.canvas.draw_rect(rect, &self.paint);
        self.paint.set_style(crate::skia_safe::PaintStyle::Fill);
    }

    /// Stroke the specified rect with border radius and offset. <br>
    ///
    /// the point of `Rect`'s coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_round_rect<T: Into<SkiaRect>>(&mut self, rect: T, border_radius: f32) {
        let mut rect: SkiaRect = rect.into();
        rect.offset((self.x_offset, self.y_offset));

        self.draw_round_rect_global(rect, border_radius);
    }

    /// Stroke the specified rect with border radius. <br>
    ///
    /// the point of `Rect`'s coordinate must be [`Coordinate::World`](tlib::namespace::Coordinate::World)
    #[inline]
    pub fn draw_round_rect_global<T: Into<SkiaRect>>(&mut self, rect: T, border_radius: f32) {
        self.paint.set_style(crate::skia_safe::PaintStyle::Stroke);
        let rect: SkiaRect = rect.into();

        let rrect = crate::skia_safe::RRect::new_rect_xy(rect, border_radius, border_radius);
        self.canvas.draw_rrect(rrect, &self.paint);
        self.paint.set_style(crate::skia_safe::PaintStyle::Fill);
    }

    /// Stroke and Fill the specified region with the specified color. <br>
    ///
    /// the point of region's coordinate must be [`Coordinate::World`](tlib::namespace::Coordinate::World)
    #[inline]
    pub fn fill_region(&mut self, region: &skia_safe::Region, color: Color) {
        self.paint.set_color(color);
        self.paint
            .set_style(crate::skia_safe::PaintStyle::StrokeAndFill);

        self.canvas.draw_region(region, &self.paint);
        if let Some(color) = self.color {
            self.paint.set_color(color);
        }
    }

    /// Stroke the specified region. <br>
    ///
    /// the point of region's coordinate must be [`Coordinate::World`](tlib::namespace::Coordinate::World)
    #[inline]
    pub fn draw_region(&mut self, region: &skia_safe::Region) {
        self.paint.set_style(crate::skia_safe::PaintStyle::Stroke);

        self.canvas.draw_region(region, &self.paint);
        self.paint.set_style(crate::skia_safe::PaintStyle::Fill);
    }

    /// Draw text paragraph at specified position `origin` with offset. <br>
    ///
    /// letter_spacing: The spacing betweeen characters.
    /// width_layout: The specified width of a text paragraph.
    ///
    /// the origin point's coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_paragraph<T: Into<Point> + Copy>(
        &mut self,
        text: &str,
        origin: T,
        letter_spacing: f32,
        width_layout: f32,
        max_lines: Option<usize>,
        ellipsis: bool,
    ) {
        self.prepare_paragraph(text, letter_spacing, width_layout, max_lines, ellipsis);

        self.draw_paragraph_prepared(origin);
    }

    /// Draw text paragraph at specified position `origin` without offset. <br>
    ///
    /// letter_spacing: The spacing betweeen characters.
    /// width_layout: The specified width of a text paragraph.
    ///
    /// the origin point's coordinate must be [`Coordinate::World`](tlib::namespace::Coordinate::World)
    #[inline]
    pub fn draw_paragraph_global<T: Into<Point> + Copy>(
        &mut self,
        text: &str,
        origin: T,
        letter_spacing: f32,
        width_layout: f32,
        max_lines: Option<usize>,
        ellipsis: bool,
    ) {
        self.prepare_paragraph(text, letter_spacing, width_layout, max_lines, ellipsis);

        self.draw_paragrah_prepared_global(origin);
    }

    /// Prepare the paragraph with out renderering.
    #[inline]
    pub fn prepare_paragraph(
        &mut self,
        text: &str,
        letter_spacing: f32,
        width_layout: f32,
        max_lines: Option<usize>,
        ellipsis: bool,
    ) {
        if self.font.is_some() {
            // create font manager
            let mut typeface_provider = TypefaceFontProvider::new();

            // Register the font typefaces, and calculate the baseline shift
            self.skia_fonts.iter().for_each(|sf| {
                let typeface = sf.typeface();
                let family = typeface.family_name();
                typeface_provider.register_typeface(typeface, Some(family.as_str()));
            });

            let mut font_collection = FontCollection::new();
            font_collection.set_asset_font_manager(Some(typeface_provider.into()));

            // set text style
            self.text_style.set_letter_spacing(letter_spacing);
            self.paragraph_style.set_text_style(&self.text_style);
            self.paragraph_style.set_max_lines(max_lines);
            if ellipsis {
                self.paragraph_style.set_ellipsis("\u{2026}");
            } else {
                self.paragraph_style.set_ellipsis("");
            }

            // layout the paragraph
            let mut paragraph_builder =
                ParagraphBuilder::new(&self.paragraph_style, font_collection);
            paragraph_builder.add_text(text);
            let mut paragraph = paragraph_builder.build();
            paragraph.layout(width_layout);

            self.paragraph = Some(paragraph);
        } else {
            warn!(
                "The `font` of `Painter` was None in widget `{}`.",
                self.name
            )
        }
    }

    /// Render the prepared paragraph.
    ///
    /// the origin point's coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_paragraph_prepared<T: Into<Point>>(&mut self, origin: T) {
        let mut origin: Point = origin.into();
        origin.offset((self.x_offset, self.y_offset));

        self.draw_paragrah_prepared_global(origin);
    }

    /// Render the prepared paragraph.
    ///
    /// the origin point's coordinate must be [`Coordinate::World`](tlib::namespace::Coordinate::World)
    #[inline]
    pub fn draw_paragrah_prepared_global<T: Into<Point>>(&mut self, origin: T) {
        if let Some(paragraph) = self.paragraph.take() {
            paragraph.paint(self.canvas, origin);
        } else {
            warn!("Widget `{}` has no paragraph prepared.", self.name)
        }
    }

    #[inline]
    pub fn get_paragraph(&self) -> Option<&Paragraph> {
        self.paragraph.as_ref()
    }

    /// Draw simple text at specified position `origin` with offset. <br>
    ///
    /// the point's coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_text<T: Into<Point>>(&self, text: &str, origin: T) {
        if let Some(font) = self.font.as_ref() {
            let mut origin: Point = origin.into();
            origin.offset((self.x_offset, self.y_offset));

            self.canvas
                .draw_str(text, origin, &font.to_skia_fonts()[0], &self.paint);
        } else {
            error!("The `font` of `Painter` is None.")
        }
    }

    #[inline]
    pub fn draw_glyphs<T: Into<Point>>(
        &self,
        glyphs: &[u16],
        positions: &[Point],
        clusters: &[u32],
        origin: T,
        utf8_text: &str,
    ) {
        if let Some(font) = self.font.as_ref() {
            let mut origin: Point = origin.into();
            origin.offset((self.x_offset, self.y_offset));

            self.canvas.draw_glyphs_utf8(
                glyphs,
                positions,
                clusters,
                utf8_text,
                origin,
                &font.to_skia_fonts()[0],
                &self.paint,
            )
        } else {
            error!("The `font` of `Painter` is None.")
        }
    }

    /// Draw a line from (x1, y1) to (x2, y2) with offset. <br>
    ///
    /// the point's coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.draw_line_f(x1 as f32, y1 as f32, x2 as f32, y2 as f32)
    }

    /// Draw a line from (x1, y1) to (x2, y2) with offset with the float numbers. <br>
    ///
    /// the point's coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_line_f(&self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let mut p1: Point = (x1, y1).into();
        let mut p2: Point = (x2, y2).into();
        p1.offset((self.x_offset, self.y_offset));
        p2.offset((self.x_offset, self.y_offset));

        self.canvas.draw_line(p1, p2, &self.paint);
    }

    /// Draw a line from (x1, y1) to (x2, y2). <br>
    #[inline]
    pub fn draw_line_global(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.draw_line_f_global(x1 as f32, y1 as f32, x2 as f32, y2 as f32)
    }

    /// Draw a line from (x1, y1) to (x2, y2) with the float numbers. <br>
    #[inline]
    pub fn draw_line_f_global(&self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let p1: Point = (x1, y1).into();
        let p2: Point = (x2, y2).into();

        self.canvas.draw_line(p1, p2, &self.paint);
    }

    /// Draw the arc. <br>
    ///
    /// the point's coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn draw_arc(
        &self,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        angle: i32,
        sweep_angle: i32,
        use_center: bool,
    ) {
        self.draw_arc_f(
            x as f32,
            y as f32,
            w as f32,
            h as f32,
            angle as f32,
            sweep_angle as f32,
            use_center,
        )
    }

    /// Draw the arc with float numbers. <br>
    ///
    /// the point's coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn draw_arc_f(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        angle: f32,
        sweep_angle: f32,
        use_center: bool,
    ) {
        let rect: FRect = (x, y, w, h).into();
        let mut rect: SkiaRect = rect.into();
        rect.offset((self.x_offset, self.y_offset));

        self.canvas
            .draw_arc(rect, angle, sweep_angle, use_center, &self.paint);
    }

    /// Draw a point at (x, y) with offset.
    ///
    /// the point's coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_point(self, x: i32, y: i32) {
        self.draw_point_f(x as f32, y as f32)
    }

    /// Draw a point at (x, y) with offset. <br>
    ///
    /// the point's coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_point_f(&self, x: f32, y: f32) {
        let mut point: Point = (x, y).into();
        point.offset((self.x_offset, self.y_offset));

        self.canvas.draw_point(point, &self.paint);
    }

    #[inline]
    pub fn clip_rect<T: Into<skia_safe::Rect>>(&self, rect: T, op: skia_safe::ClipOp) {
        let mut rect: skia_safe::Rect = rect.into();
        rect.offset((self.x_offset, self.y_offset));
        self.canvas.clip_rect(rect, op, false);
    }

    #[inline]
    pub fn clip_rect_global<T: Into<skia_safe::Rect>>(&self, rect: T, op: skia_safe::ClipOp) {
        let rect: skia_safe::Rect = rect.into();
        self.canvas.clip_rect(rect, op, false);
    }

    /// Clip the region to draw.
    #[inline]
    pub fn clip_region_global(&self, region: skia_safe::Region, op: skia_safe::ClipOp) {
        self.canvas.clip_region(&region, Some(op));
    }

    /// Draw the path tho canvas.
    #[inline]
    pub fn draw_path(&self, path: &Path) {
        self.canvas.draw_path(path, &self.paint);
    }

    #[inline]
    pub fn draw_paint(&mut self, paint: &Paint) {
        self.canvas.draw_paint(paint);
    }

    /// Draw the original image with the actual size to the canvas. <br>
    ///
    /// point (x, y) represent the left-top point to display. <br>
    /// the point's coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_image<T: Into<SkiaPoint>>(&self, image: impl AsRef<SkiaImage>, point: T) {
        let mut point: SkiaPoint = point.into();
        point.offset((self.x_offset, self.y_offset));

        self.canvas.draw_image(image, point, Some(&self.paint));
    }

    /// Draw the original image with the given rect to the canvas. <br>
    ///
    /// the point of `Rect`'s coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_image_rect(&self, image: &impl AsRef<SkiaImage>, from: Option<Rect>, dst: Rect) {
        let mut from_rect: skia_safe::Rect;
        let from = if let Some(from) = from {
            from_rect = from.into();
            from_rect.offset((self.x_offset, self.y_offset));
            Some((&from_rect, SrcRectConstraint::Strict))
        } else {
            None
        };
        let mut dst: skia_safe::Rect = dst.into();
        dst.offset((self.x_offset, self.y_offset));

        self.canvas.draw_image_rect(image, from, dst, &self.paint);
    }

    #[inline]
    pub fn draw_pixels<T: Into<tlib::figure::Point>>(
        &self,
        info: &skia_safe::ImageInfo,
        pixels: &[u8],
        row_bytes: usize,
        offset: T,
    ) {
        let mut offset: tlib::figure::Point = offset.into();
        offset.offset(self.x_offset, self.y_offset);
        let offset: IPoint = offset.into();

        let _ = self.canvas.write_pixels(info, pixels, row_bytes, offset);
    }

    #[cfg(svg)]
    #[inline]
    pub fn draw_dom(&self, dom: &tlib::typedef::SkiaSvgDom) {
        dom.render(self.canvas)
    }
}
