#![allow(dead_code)]
use std::ops::{Add, Sub};

use hex_color::HexColor;
use tlib::{
    types::StaticType,
    values::{FromBytes, FromValue, ToBytes, ToValue},
    Value,
};

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// Point
//////////////////////////////////////////////////////////////////////////////////////////////////////
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

impl StaticType for Point {
    fn static_type() -> tlib::Type {
        tlib::Type::from_name("Point")
    }

    fn bytes_len() -> usize {
        i32::bytes_len() + i32::bytes_len()
    }
}
impl ToBytes for Point {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.x.to_bytes());
        bytes.append(&mut self.y.to_bytes());
        bytes
    }
}
impl FromBytes for Point {
    fn from_bytes(data: &[u8], _len: usize) -> Self {
        let x = i32::from_bytes(&data[0..4], 4);
        let y = i32::from_bytes(&data[4..8], 4);
        Self { x, y }
    }
}
impl ToValue for Point {
    fn to_value(&self) -> tlib::Value {
        tlib::Value::new(self)
    }

    fn value_type(&self) -> tlib::Type {
        Self::static_type()
    }
}
impl FromValue for Point {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// Size
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Size {
    width: i32,
    height: i32,
}
impl Size {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}
impl From<(i32, i32)> for Size {
    fn from((width, height): (i32, i32)) -> Self {
        Self { width, height }
    }
}
impl Into<(i32, i32)> for Size {
    fn into(self) -> (i32, i32) {
        (self.width, self.height)
    }
}
impl Add for Size {
    type Output = Size;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}
impl Sub for Size {
    type Output = Size;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}

impl StaticType for Size {
    fn static_type() -> tlib::Type {
        tlib::Type::from_name("Point")
    }

    fn bytes_len() -> usize {
        i32::bytes_len() + i32::bytes_len()
    }
}
impl ToBytes for Size {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.width.to_bytes());
        bytes.append(&mut self.height.to_bytes());
        bytes
    }
}
impl FromBytes for Size {
    fn from_bytes(data: &[u8], _len: usize) -> Self {
        let width = i32::from_bytes(&data[0..4], 4);
        let height = i32::from_bytes(&data[4..8], 4);
        Self { width, height }
    }
}
impl ToValue for Size {
    fn to_value(&self) -> tlib::Value {
        tlib::Value::new(self)
    }

    fn value_type(&self) -> tlib::Type {
        Self::static_type()
    }
}
impl FromValue for Size {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// Rectangle
//////////////////////////////////////////////////////////////////////////////////////////////////////
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

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn top_left(&self) -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn top_right(&self) -> Point {
        Point {
            x: self.width,
            y: 0,
        }
    }

    pub fn bottom_left(&self) -> Point {
        Point {
            x: 0,
            y: self.height,
        }
    }

    pub fn bottom_right(&self) -> Point {
        Point {
            x: self.width,
            y: self.height,
        }
    }

    pub fn intersects(&self, rect: &Rect) -> bool {
        self.x.max(rect.x) <= self.width.min(rect.width)
            && self.y.max(rect.y) <= self.height.min(rect.height)
    }

    pub fn contains(&self, point: &Point) -> bool {
        point.x() >= self.x()
            && point.y() >= self.y()
            && point.x() <= self.width()
            && point.y() <= self.height()
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

impl StaticType for Rect {
    fn static_type() -> tlib::Type {
        tlib::Type::from_name("Rect")
    }

    fn bytes_len() -> usize {
        i32::bytes_len() * 4
    }
}
impl ToBytes for Rect {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.x.to_bytes());
        bytes.append(&mut self.y.to_bytes());
        bytes.append(&mut self.width.to_bytes());
        bytes.append(&mut self.height.to_bytes());
        bytes
    }
}
impl FromBytes for Rect {
    fn from_bytes(data: &[u8], _len: usize) -> Self {
        let x = i32::from_bytes(&data[0..4], 4);
        let y = i32::from_bytes(&data[4..8], 4);
        let width = i32::from_bytes(&data[8..12], 4);
        let height = i32::from_bytes(&data[12..16], 4);
        Self {
            x,
            y,
            width,
            height,
        }
    }
}
impl ToValue for Rect {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> tlib::Type {
        Self::static_type()
    }
}
impl FromValue for Rect {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// Color
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
    pub valid: bool,
}

impl Color {
    pub fn new() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
            valid: false,
        }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: 255,
            valid: true,
        }
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a,
            valid: true,
        }
    }

    pub fn from_hex(hex_code: &str) -> Self {
        let color = HexColor::parse(hex_code)
            .expect(format!("Parse hex color failed, {}", hex_code).as_str());
        Self {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
            valid: true,
        }
    }

    pub fn hexcode(&self) -> String {
        HexColor::rgba(self.r, self.g, self.b, self.a).to_string()
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

impl Into<(i32, i32, i32)> for &Color {
    fn into(self) -> (i32, i32, i32) {
        (self.r as i32, self.g as i32, self.b as i32)
    }
}

impl StaticType for Color {
    fn static_type() -> tlib::Type {
        tlib::Type::from_name("Color")
    }

    fn bytes_len() -> usize {
        u8::bytes_len() * 4
    }
}
impl ToBytes for Color {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.r.to_bytes());
        bytes.append(&mut self.g.to_bytes());
        bytes.append(&mut self.b.to_bytes());
        bytes.append(&mut self.a.to_bytes());
        bytes.append(&mut self.valid.to_bytes());
        bytes
    }
}
impl FromBytes for Color {
    fn from_bytes(data: &[u8], _len: usize) -> Self {
        let r = u8::from_bytes(&data[0..1], 1);
        let g = u8::from_bytes(&data[1..2], 1);
        let b = u8::from_bytes(&data[2..3], 1);
        let a = u8::from_bytes(&data[3..4], 1);
        let valid = data[4] == 1;
        Self { r, g, b, a, valid }
    }
}
impl ToValue for Color {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> tlib::Type {
        Self::static_type()
    }
}
impl FromValue for Color {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
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

        let val = p4.to_value();
        let get = val.get::<Point>();
        assert_eq!(20, get.x);
        assert_eq!(20, get.y);
    }

    #[test]
    fn test_size() {
        let size = Size::new(100, 100);
        let val = size.to_value();
        let get = val.get::<Size>();
        assert_eq!(size, get);
    }

    #[test]
    fn test_rect() {
        let rect = Rect::new(10, 10, 50, 50);
        let val = rect.to_value();
        let get = val.get::<Rect>();
        assert_eq!(rect, get)
    }

    #[test]
    fn test_color() {
        let color = Color::from_rgba(13, 13, 13, 13);
        let val = color.to_value();
        let get = val.get();
        assert_eq!(color, get);
        println!("{}", color.hexcode());
    }
}
