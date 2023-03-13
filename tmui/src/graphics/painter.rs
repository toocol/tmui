#![allow(dead_code)]
use super::figure::{Color, FRect, Transform};
use crate::{widget::WidgetImpl, util::skia_font_clone};
use log::error;
use skia_safe::{Canvas, Font, Paint, Path, Point};
use std::cell::RefMut;

pub struct Painter<'a> {
    canvas: RefMut<'a, Canvas>,
    path: Path,
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

    transform: Transform,
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
            color: None,
            line_width: None,
            saved_font: None,
            saved_color: None,
            saved_line_width: None,
            width: rect.width(),
            height: rect.height(),
            x_offset: rect.x(),
            y_offset: rect.y(),
            transform: Transform::new(),
        }
    }

    #[inline]
    pub fn set_transform(&mut self, transform: Transform, combined: bool) {
        if combined {
            self.transform = self.transform * transform;
        } else {
            self.transform = transform
        }
    }

    /// Save the canvas status.
    #[inline]
    pub fn save(&mut self) {
        self.canvas.save();
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
        self.color = Some(color);
        self.paint.set_color(color);
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

    /// Stroke and fill the specified Rect with offset.
    #[inline]
    pub fn fill_rect<T: Into<skia_safe::Rect>>(&mut self, rect: T, color: Color) {
        self.paint.set_color(color);
        self.paint.set_style(skia_safe::PaintStyle::StrokeAndFill);

        let mut rect: skia_safe::Rect = rect.into();
        rect.offset((self.x_offset, self.y_offset));

        self.canvas.draw_rect(rect, &self.paint);
    }

    /// Stroke the specified Rect with offset.
    #[inline]
    pub fn draw_rect<T: Into<skia_safe::Rect>>(&mut self, rect: T) {
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
        self.draw_line_f(x1 as f32, y1 as f32, x2 as f32, y2 as f32)
    }

    /// Draw a line from (x1, y1) to (x2, y2) with offset.
    #[inline]
    pub fn draw_line_f(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let mut p1: Point = (x1, y1).into();
        let mut p2: Point = (x2, y2).into();
        p1.offset((self.x_offset, self.y_offset));
        p2.offset((self.x_offset, self.y_offset));

        self.canvas.draw_line(p1, p2, &self.paint);
    }

    /// Draw arc
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

    /// Draw arc
    #[inline]
    pub fn draw_arc_f(&mut self, x: f32, y: f32, w: f32, h: f32, a: f32, alen: f32) {
        let rect: FRect = (x, y, w, h).into();
        let mut rect: skia_safe::Rect = rect.into();
        rect.offset((self.x_offset, self.y_offset));

        self.canvas.draw_arc(rect, a, alen, true, &self.paint);
    }

    /// Draw a point at (x, y) with offset.
    #[inline]
    pub fn draw_point(&mut self, x: i32, y: i32) {
        self.draw_point_f(x as f32, y as f32)
    }

    /// Draw a point at (x, y) with offset.
    #[inline]
    pub fn draw_point_f(&mut self, x: f32, y: f32) {
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
