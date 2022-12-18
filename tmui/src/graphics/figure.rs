#![allow(dead_code)]

use std::ops::{Add, Sub};

use hex_color::HexColor;
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn to_skia_ipoint(self) -> skia_safe::IPoint {
        skia_safe::IPoint::from(self)
    }

    pub fn to_skia_point(self) -> skia_safe::Point {
        skia_safe::Point::from(self)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<Point> for skia_safe::IPoint {
    fn from(p: Point) -> Self {
        skia_safe::IPoint::from((p.x, p.y))
    }
}

impl From<Point> for skia_safe::Point {
    fn from(p: Point) -> Self {
        skia_safe::Point::from(skia_safe::IPoint::from(p))
    }
}

impl Into<(i32, i32)> for Point {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn top_left(&self) -> Point {
        Point { x: 0, y: 0 }
    }

    fn top_right(&self) -> Point {
        Point {
            x: self.width,
            y: 0,
        }
    }

    fn bottom_left(&self) -> Point {
        Point {
            x: 0,
            y: self.height,
        }
    }

    fn bottom_right(&self) -> Point {
        Point {
            x: self.width,
            y: self.height,
        }
    }

    fn intersects(&self, rect: &Rect) -> bool {
        self.x.max(rect.x) <= self.width.min(rect.width)
            && self.y.max(rect.y) <= self.height.min(rect.height)
    }
}

impl From<(i32, i32, i32, i32)> for Rect {
    fn from((x, y, width, height): (i32, i32, i32, i32)) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl Into<(i32, i32, i32, i32)> for Rect {
    fn into(self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.width, self.height)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_hex(hex_code: &str) -> Self {
        let color = HexColor::parse(hex_code)
            .expect(format!("Parse hex color failed, {}", hex_code).as_str());
        Self {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::from_rgb(r, g, b)
    }
}

impl From<(i32, i32, i32)> for Color {
    fn from((r, g, b): (i32, i32, i32)) -> Self {
        Self::from_rgb(r as u8, g as u8, b as u8)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self::from_rgba(r, g, b, a)
    }
}

impl From<(i32, i32, i32, i32)> for Color {
    fn from((r, g, b, a): (i32, i32, i32, i32)) -> Self {
        Self::from_rgba(r as u8, g as u8, b as u8, a as u8)
    }
}

impl From<String> for Color {
    fn from(hex: String) -> Self {
        Self::from_hex(hex.as_str())
    }
}

impl From<&str> for Color {
    fn from(hex: &str) -> Self {
        Self::from_hex(hex)
    }
}

impl Into<(i32, i32, i32)> for Color {
    fn into(self) -> (i32, i32, i32) {
        (self.r as i32, self.g as i32, self.b as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(20, 20);
        let p3 = p1 + p2;
        assert_eq!(30, p3.x);
        assert_eq!(30, p3.y);
        let p4 = p3 - p1;
        assert_eq!(20, p4.x);
        assert_eq!(20, p4.y);
    }
}
