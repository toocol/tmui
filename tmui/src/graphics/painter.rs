#![allow(dead_code)]
use super::figure::Color;
use crate::{prelude::Rect, widget::WidgetImpl};
use log::error;
use skia_safe::{Canvas, Font, Paint, Path, Point};
use std::cell::RefMut;

pub struct Painter<'a> {
    canvas: RefMut<'a, Canvas>,
    path: Path,
    paint: Paint,
    font: Option<Font>,

    width: i32,
    height: i32,
    x_offset: i32,
    y_offset: i32,
}

impl<'a> Painter<'a> {
    /// The constructer to build the Painter.
    pub fn new(canvas: RefMut<'a, Canvas>, widget: &dyn WidgetImpl) -> Painter<'a> {
        let rect = widget.rect();
        Painter {
            canvas,
            path: Path::default(),
            paint: Paint::default(),
            font: None,
            width: rect.width(),
            height: rect.height(),
            x_offset: rect.x(),
            y_offset: rect.y(),
        }
    }

    /// Save the canvas status.
    #[inline]
    pub fn save(&mut self) {
        self.canvas.save();
    }

    /// Restore the canvas status.
    #[inline]
    pub fn restore(&mut self) {
        self.canvas.restore();
    }

    /// Reset the painter to Initial state.
    #[inline]
    pub fn reset(&mut self) {
        self.canvas.reset_matrix();
        self.paint.reset();
        self.path.reset();
    }

    /// Set the antialiasing to true.
    #[inline]
    pub fn set_antialiasing(&mut self) {
        self.paint.set_anti_alias(true);
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
        self.font = Some(font);
    }

    /// Set the color of painter.
    #[inline]
    pub fn set_color(&mut self, color: Color) {
        self.paint.set_color(color);
    }

    /// Set the stroke width of painter.
    #[inline]
    pub fn set_line_width(&mut self, width: f32) {
        self.paint.set_stroke_width(width);
    }

    /// Stroke and fill the specified Rect with offset.
    #[inline]
    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        self.paint.set_color(color);
        self.paint.set_style(skia_safe::PaintStyle::StrokeAndFill);

        let mut rect: skia_safe::Rect = rect.into();
        rect.offset((self.x_offset, self.y_offset));

        self.canvas.draw_rect(rect, &self.paint);
    }

    /// Stroke the specified Rect with offset.
    #[inline]
    pub fn draw_rect(&mut self, rect: Rect) {
        self.paint.set_style(skia_safe::PaintStyle::Stroke);
        let rect: skia_safe::Rect = rect.into();
        self.canvas.draw_rect(rect, &self.paint);
    }

    /// Draw text at specified position `origin` with offset.
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

    /// Draw a line from (x1, y1) to (x2, y2) with offset.
    #[inline]
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let mut p1: Point = (x1, y1).into();
        let mut p2: Point = (x2, y2).into();
        p1.offset((self.x_offset, self.y_offset));
        p2.offset((self.x_offset, self.y_offset));

        self.canvas.draw_line(p1, p2, &self.paint);
    }

    /// Draw arc
    #[inline]
    pub fn draw_arc(&mut self, x: i32, y: i32, w: i32, h: i32, a: i32, alen: i32) {
        let rect: Rect = (x, y, w, h).into();
        let mut rect: skia_safe::Rect = rect.into();
        rect.offset((self.x_offset, self.y_offset));

        self.canvas
            .draw_arc(rect, a as f32, alen as f32, true, &self.paint);
    }

    /// Draw a point at (x, y) with offset.
    #[inline]
    pub fn draw_point(&mut self, x: i32, y: i32) {
        let mut point: Point = (x, y).into();
        point.offset((self.x_offset, self.y_offset));

        self.canvas.draw_point(point, &self.paint);
    }

    /// Draw a pixmap.
    #[inline]
    pub fn draw_pixmap(&mut self) {
        todo!()
    }
}
