use std::ops::{Add, Sub};
use tlib::{
    types::StaticType,
    values::{FromBytes, FromValue, ToBytes, ToValue},
    Type, Value,
};

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// Point
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.x
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.y
    }

    #[inline]
    pub fn set_x(&mut self, x: f32) {
        self.x = x
    }

    #[inline]
    pub fn set_y(&mut self, y: f32) {
        self.y = y
    }

    #[inline]
    pub fn to_skia_ipoint(self) -> skia_safe::IPoint {
        skia_safe::IPoint::from(self)
    }

    #[inline]
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
        Self { x: x as f32, y: y as f32}
    }
}

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y}
    }
}

impl From<Point> for skia_safe::IPoint {
    fn from(p: Point) -> Self {
        p.into()
    }
}

impl From<Point> for skia_safe::Point {
    fn from(p: Point) -> Self {
        skia_safe::Point::from(skia_safe::IPoint::from(p))
    }
}

impl Into<(f32, f32)> for Point {
    fn into(self) -> (f32, f32) {
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
        let x = f32::from_bytes(&data[0..4], 4);
        let y = f32::from_bytes(&data[4..8], 4);
        Self { x, y }
    }
}

impl ToValue for Point {
    fn to_value(&self) -> Value {
        tlib::Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl FromValue for Point {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}
