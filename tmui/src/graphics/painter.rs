#![allow(dead_code)]
use crate::{
    application_window::ApplicationWindow,
    skia_safe::{self, Canvas, Font, Matrix, Paint, Path, Point},
    widget::WidgetImpl,
};
use log::{error, warn};
use std::{cell::RefMut, ffi::c_uint};
use tlib::{
    figure::{Color, FRect, ImageBuf, Rect},
    global::skia_font_clone,
    skia_safe::{
        canvas::{SaveLayerRec, SrcRectConstraint},
        textlayout::{
            FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle, TypefaceFontProvider,
        },
    },
};

pub struct Painter<'a> {
    canvas: RefMut<'a, Canvas>,
    paint: Paint,
    font: Option<Font>,
    color: Option<Color>,
    line_width: Option<f32>,

    saved_font: Option<Font>,
    saved_color: Option<Color>,
    saved_line_width: Option<f32>,

    width: i32,
    height: i32,
    x_offset: i32,
    y_offset: i32,

    transform: Matrix,

    text_style: TextStyle,
    paragraph_style: ParagraphStyle,
}

impl<'a> Painter<'a> {
    /// The constructer to build the Painter.
    #[inline]
    pub fn new(canvas: RefMut<'a, Canvas>, widget: &dyn WidgetImpl) -> Painter<'a> {
        let base_offset = ApplicationWindow::window_of(widget.window_id()).base_offset();
        let rect = widget.rect();
        Painter {
            canvas,
            paint: Paint::default(),
            font: None,
            color: None,
            line_width: None,
            saved_font: None,
            saved_color: None,
            saved_line_width: None,
            width: rect.width(),
            height: rect.height(),
            x_offset: rect.x() + base_offset.x(),
            y_offset: rect.y() + base_offset.y(),
            transform: Matrix::new_identity(),
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
        &self.canvas
    }

    #[inline]
    pub fn canvas_mut(&mut self) -> &mut Canvas {
        &mut self.canvas
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
    pub fn save(&mut self) -> usize {
        self.canvas.save()
    }

    /// Get canvas save count.
    #[inline]
    pub fn save_count(&self) -> usize {
        self.canvas.save_count()
    }

    /// Save the pen status: Color, Font, line width etc...
    #[inline]
    pub fn save_pen(&mut self) {
        if let Some(ref font) = self.font {
            self.saved_font = Some(skia_font_clone(font))
        }
        self.saved_color = self.color.clone();
        self.saved_line_width = self.line_width.clone();
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
    pub fn restore(&mut self) {
        self.canvas.restore();
    }

    #[inline]
    pub fn restore_to_count(&mut self, count: usize) {
        self.canvas.restore_to_count(count);
    }

    #[inline]
    pub fn save_layer(&mut self, layer: &SaveLayerRec) -> usize {
        self.canvas.save_layer(layer)
    }

    #[inline]
    pub fn save_layer_alpha<T: Into<Option<skia_safe::Rect>>>(
        &mut self,
        layer: T,
        alpha: u8,
    ) -> usize {
        self.canvas.save_layer_alpha(layer, alpha as c_uint)
    }

    /// Reset the painter to Initial state.
    #[inline]
    pub fn reset(&mut self) {
        self.canvas.reset_matrix();
        self.paint.reset();
    }

    /// Set the antialiasing to true.
    #[inline]
    pub fn set_antialiasing(&mut self, anti_alias: bool) {
        self.paint.set_anti_alias(anti_alias);
    }

    /// Set the global transform of this painter.
    #[inline]
    pub fn scale(&mut self, sx: i32, sy: i32) {
        self.canvas.scale((sx as f32, sy as f32));
    }

    #[inline]
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.canvas.translate((dx, dy));
    }

    /// Set the font of painter.
    #[inline]
    pub fn set_font(&mut self, font: Font) {
        self.text_style.set_font_size(font.size());
        if let Some(typeface) = font.typeface() {
            self.text_style
                .set_font_families(&vec![typeface.family_name()]);
        }
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
    pub fn clear(&mut self, color: Color) {
        self.canvas.clear(color);
    }

    /// Stroke and fill the specified Rect with offset. <br>
    ///
    /// the point of `Rect`'s coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn fill_rect<T: Into<crate::skia_safe::Rect>>(&mut self, rect: T, color: Color) {
        self.paint.set_color(color);
        self.paint
            .set_style(crate::skia_safe::PaintStyle::Fill);

        let mut rect: crate::skia_safe::Rect = rect.into();
        rect.offset((self.x_offset, self.y_offset));

        self.canvas.draw_rect(rect, &self.paint);
        if let Some(color) = self.color {
            self.paint.set_color(color);
        }
    }

    #[inline]
    pub fn fill_round_rect<T: Into<crate::skia_safe::Rect>>(
        &mut self,
        rect: T,
        border_radius: f32,
        color: Color,
    ) {
        self.paint.set_color(color);
        self.paint
            .set_style(crate::skia_safe::PaintStyle::Fill);

        let mut rect: crate::skia_safe::Rect = rect.into();
        rect.offset((self.x_offset, self.y_offset));

        let rrect = crate::skia_safe::RRect::new_rect_xy(rect, border_radius, border_radius);
        self.canvas.draw_rrect(rrect, &self.paint);
        if let Some(color) = self.color {
            self.paint.set_color(color);
        }
    }

    /// Stroke the specified Rect with offset. <br>
    ///
    /// the point of `Rect`'s coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_rect<T: Into<crate::skia_safe::Rect>>(&mut self, rect: T) {
        self.paint.set_style(crate::skia_safe::PaintStyle::Stroke);
        let mut rect: crate::skia_safe::Rect = rect.into();
        rect.offset((self.x_offset, self.y_offset));

        self.canvas.draw_rect(rect, &self.paint);
        self.paint.set_style(crate::skia_safe::PaintStyle::Fill);
    }

    /// Strike the specified rect with border radius and offset.
    ///
    /// the point of `Rect`'s coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_round_rect<T: Into<crate::skia_safe::Rect>>(
        &mut self,
        rect: T,
        border_radius: f32,
    ) {
        self.paint.set_style(crate::skia_safe::PaintStyle::Stroke);
        let mut rect: crate::skia_safe::Rect = rect.into();
        rect.offset((self.x_offset, self.y_offset));

        let rrect = crate::skia_safe::RRect::new_rect_xy(rect, border_radius, border_radius);
        self.canvas.draw_rrect(rrect, &self.paint);
        self.paint.set_style(crate::skia_safe::PaintStyle::Fill);
    }

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
    /// the point's coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_paragraph<T: Into<Point>>(
        &mut self,
        text: &str,
        origin: T,
        letter_spacing: f32,
        width_layout: f32,
        max_lines: Option<usize>,
        ellipsis: bool,
    ) {
        if let Some(font) = self.font.as_ref() {
            // create font manager
            let mut typeface_provider = TypefaceFontProvider::new();
            if let Some(typeface) = font.typeface() {
                let family = typeface.family_name();
                typeface_provider.register_typeface(typeface, Some(family));
            } else {
                warn!("The typeface of font not specified.");
                return;
            }
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

            let mut origin: Point = origin.into();
            origin.offset((self.x_offset, self.y_offset));

            paragraph.paint(&mut self.canvas, origin);
        } else {
            error!("The `font` of `Painter` is None.")
        }
    }

    /// Draw simple text at specified position `origin` with offset. <br>
    ///
    /// the point's coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_text<T: Into<Point>>(&mut self, text: &str, origin: T) {
        if let Some(font) = self.font.as_ref() {
            let mut origin: Point = origin.into();
            origin.offset((self.x_offset, self.y_offset));

            self.canvas.draw_str(text, origin, &font, &self.paint);
        } else {
            error!("The `font` of `Painter` is None.")
        }
    }

    #[inline]
    pub fn draw_glyphs<T: Into<Point>>(
        &mut self,
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
                font,
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
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.draw_line_f(x1 as f32, y1 as f32, x2 as f32, y2 as f32)
    }

    /// Draw a line from (x1, y1) to (x2, y2) with offset with the float numbers. <br>
    ///
    /// the point's coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_line_f(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let mut p1: Point = (x1, y1).into();
        let mut p2: Point = (x2, y2).into();
        p1.offset((self.x_offset, self.y_offset));
        p2.offset((self.x_offset, self.y_offset));

        self.canvas.draw_line(p1, p2, &self.paint);
    }

    /// Draw the arc. <br>
    ///
    /// the point's coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_arc(&mut self, x: i32, y: i32, w: i32, h: i32, a: i32, alen: i32) {
        self.draw_arc_f(
            x as f32,
            y as f32,
            w as f32,
            h as f32,
            a as f32,
            alen as f32,
        )
    }

    /// Draw the arc with float numbers. <br>
    ///
    /// the point's coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_arc_f(&mut self, x: f32, y: f32, w: f32, h: f32, a: f32, alen: f32) {
        let rect: FRect = (x, y, w, h).into();
        let mut rect: crate::skia_safe::Rect = rect.into();
        rect.offset((self.x_offset, self.y_offset));

        self.canvas.draw_arc(rect, a, alen, true, &self.paint);
    }

    /// Draw a point at (x, y) with offset.
    ///
    /// the point's coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_point(&mut self, x: i32, y: i32) {
        self.draw_point_f(x as f32, y as f32)
    }

    /// Draw a point at (x, y) with offset. <br>
    ///
    /// the point's coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_point_f(&mut self, x: f32, y: f32) {
        let mut point: Point = (x, y).into();
        point.offset((self.x_offset, self.y_offset));

        self.canvas.draw_point(point, &self.paint);
    }

    #[inline]
    pub fn clip_rect<T: Into<skia_safe::Rect>>(&mut self, rect: T, op: skia_safe::ClipOp) {
        let mut rect: skia_safe::Rect = rect.into();
        rect.offset((self.x_offset, self.y_offset));
        self.canvas.clip_rect(rect, op, false);
    }

    /// Clip the region to draw.
    #[inline]
    pub fn clip_region(&mut self, region: skia_safe::Region, op: skia_safe::ClipOp) {
        self.canvas.clip_region(&region, Some(op));
    }

    /// Draw the path tho canvas.
    #[inline]
    pub fn draw_path(&mut self, path: &mut Path) {
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
    pub fn draw_image(&mut self, image: &ImageBuf, x: i32, y: i32) {
        let mut point: Point = (x, y).into();
        point.offset((self.x_offset, self.y_offset));

        self.canvas.draw_image(image, point, Some(&self.paint));
    }

    /// Draw the original image with the given rect to the canvas. <br>
    ///
    /// the point of `Rect`'s coordinate must be [`Coordinate::Widget`](tlib::namespace::Coordinate::Widget)
    #[inline]
    pub fn draw_image_rect(&mut self, image: &ImageBuf, from: Option<Rect>, dst: Rect) {
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
}
