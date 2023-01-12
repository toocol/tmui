#![allow(dead_code)]
use skia_safe::{Path, Paint, Font, Canvas};
use tlib::prelude::Align;
use crate::prelude::Rect;
use super::figure::Color;

pub struct Painter<'a> {
    drawing_context: &'a Canvas,
    path: &'a Path,
    paint: &'a Paint,
    font: Option<Font>,

    width: i32,
    height: i32,
    x_offset: i32,
    y_offset: i32,
}

impl<'a> Painter<'a> {
    pub fn save(&mut self) {

    }

    pub fn restore(&mut self) {

    }

    pub fn set_render_hints(&mut self) {

    }

    pub fn set_world_transform(&mut self) {

    }

    pub fn set_font(&mut self, font: Font) {
        self.font = Some(font);
    }

    pub fn set_layout_direction(&mut self) {

    }

    pub fn fill_rect(&mut self, rect: Rect, color: Color) {

    }

    pub fn draw_rect(&mut self, rect: Rect) {

    }

    pub fn draw_pixmap(&mut self) {

    }

    pub fn draw_text(&mut self, align: Align, text: &str) {

    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {

    }

    pub fn draw_point(&mut self, x: i32, y: i32) {

    }
}
