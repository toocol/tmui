use crate::{
    namespace::Coordinate, typedef::{SkiaIPoint, SkiaPoint}, types::StaticType, values::{FromBytes, FromValue, ToBytes, ToValue}, Type, Value
};
use std::ops::{Add, Div, Mul, Sub};

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// Point
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn x(&self) -> i32 {
        self.x
    }

    #[inline]
    pub fn y(&self) -> i32 {
        self.y
    }

    #[inline]
    pub fn set_x(&mut self, x: i32) {
        self.x = x
    }

    #[inline]
    pub fn set_y(&mut self, y: i32) {
        self.y = y
    }

    #[inline]
    pub fn x_mut(&mut self) -> &mut i32 {
        &mut self.x
    }

    #[inline]
    pub fn y_mut(&mut self) -> &mut i32 {
        &mut self.y
    }

    #[inline]
    pub fn offset(&mut self, x_off: i32, y_off: i32) {
        self.x += x_off;
        self.y += y_off;
    }
}

impl Add for Point {
    type Output = Point;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Point {
    type Output = Point;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div for Point {
    type Output = Point;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl From<(i32, i32)> for Point {
    #[inline]
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<Point> for (i32, i32) {
    #[inline]
    fn from(value: Point) -> Self {
        (value.x, value.y)
    }
}

impl From<FPoint> for Point {
    #[inline]
    fn from(value: FPoint) -> Self {
        Self { x: value.x() as i32, y: value.y() as i32 }
    }
}

impl From<Point> for SkiaPoint {
    #[inline]
    fn from(value: Point) -> Self {
        SkiaPoint {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl From<Point> for Option<SkiaPoint> {
    #[inline]
    fn from(value: Point) -> Self {
        Some(SkiaPoint::from(value))
    }
}

impl From<Point> for SkiaIPoint {
    #[inline]
    fn from(value: Point) -> Self {
        SkiaIPoint {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<SkiaPoint> for Point {
    #[inline]
    fn from(value: SkiaPoint) -> Self {
        Point {
            x: value.x as i32,
            y: value.y as i32,
        }
    }
}

impl From<SkiaIPoint> for Point {
    #[inline]
    fn from(value: SkiaIPoint) -> Self {
        Point {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<Point> for FPoint {
    #[inline]
    fn from(value: Point) -> Self {
        FPoint {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl StaticType for Point {
    #[inline]
    fn static_type() -> crate::Type {
        crate::Type::from_name("Point")
    }

    #[inline]
    fn bytes_len() -> usize {
        i32::bytes_len() + i32::bytes_len()
    }
}

impl ToBytes for Point {
    #[inline]
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.x.to_bytes());
        bytes.append(&mut self.y.to_bytes());
        bytes
    }
}

impl FromBytes for Point {
    #[inline]
    fn from_bytes(data: &[u8], _len: usize) -> Self {
        let x = i32::from_bytes(&data[0..4], 4);
        let y = i32::from_bytes(&data[4..8], 4);
        Self { x, y }
    }
}

impl ToValue for Point {
    #[inline]
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    #[inline]
    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl FromValue for Point {
    #[inline]
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// FPoint
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct FPoint {
    x: f32,
    y: f32,
}

impl FPoint {
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
    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }

    #[inline]
    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }

    #[inline]
    pub fn offset(&mut self, x_off: f32, y_off: f32) {
        self.x += x_off;
        self.y += y_off;
    }
}

impl Add for FPoint {
    type Output = FPoint;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for FPoint {
    type Output = FPoint;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for FPoint {
    type Output = FPoint;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div for FPoint {
    type Output = FPoint;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl From<(i32, i32)> for FPoint {
    #[inline]
    fn from((x, y): (i32, i32)) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
        }
    }
}

impl From<(f32, f32)> for FPoint {
    #[inline]
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

impl From<FPoint> for (f32, f32) {
    #[inline]
    fn from(value: FPoint) -> Self {
        (value.x, value.y)
    }
}

impl From<FPoint> for SkiaPoint {
    #[inline]
    fn from(value: FPoint) -> Self {
        SkiaPoint {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<FPoint> for SkiaIPoint {
    #[inline]
    fn from(value: FPoint) -> Self {
        SkiaIPoint {
            x: value.x as i32,
            y: value.y as i32,
        }
    }
}

impl From<SkiaPoint> for FPoint {
    #[inline]
    fn from(value: SkiaPoint) -> Self {
        FPoint {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<SkiaIPoint> for FPoint {
    #[inline]
    fn from(value: SkiaIPoint) -> Self {
        FPoint {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl StaticType for FPoint {
    fn static_type() -> crate::Type {
        crate::Type::from_name("FPoint")
    }

    fn bytes_len() -> usize {
        f32::bytes_len() + f32::bytes_len()
    }
}

impl ToBytes for FPoint {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.x.to_bytes());
        bytes.append(&mut self.y.to_bytes());
        bytes
    }
}

impl FromBytes for FPoint {
    fn from_bytes(data: &[u8], _len: usize) -> Self {
        let x = f32::from_bytes(&data[0..4], 4);
        let y = f32::from_bytes(&data[4..8], 4);
        Self { x, y }
    }
}

impl ToValue for FPoint {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl FromValue for FPoint {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// CoordPoint
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct CoordPoint(FPoint, Coordinate);
impl CoordPoint {
    #[inline]
    pub fn new<T: Into<FPoint>>(point: T, coord: Coordinate) -> Self {
        Self(point.into(), coord)
    }

    #[inline]
    pub fn point(&self) -> FPoint {
        self.0
    }

    #[inline]
    pub fn coord(&self) -> Coordinate {
        self.1
    }
}

impl StaticType for CoordPoint {
    fn static_type() -> crate::Type {
        crate::Type::from_name("CoordPoint")
    }

    fn bytes_len() -> usize {
        FPoint::bytes_len() + Coordinate::bytes_len()
    }
}

impl ToBytes for CoordPoint {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.0.to_bytes());
        bytes.append(&mut self.1.to_bytes());
        bytes
    }
}

impl FromBytes for CoordPoint {
    fn from_bytes(data: &[u8], _len: usize) -> Self {
        let point = FPoint::from_bytes(&data[0..8], 8);
        let coord = Coordinate::from_bytes(&data[8..9], 1);
        Self(point, coord)
    }
}

impl ToValue for CoordPoint {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl FromValue for CoordPoint {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

#[cfg(test)]
mod tests {
    use crate::{namespace::Coordinate, prelude::ToValue};

    use super::{Point, CoordPoint};

    #[test]
    fn test_mut_ref() {
        let mut pos: Point = (0, 0).into();
        *pos.x_mut() += 1;
        assert_eq!(pos.x(), 1);
    }

    #[test]
    fn test_coord_point_value() {
        let p = CoordPoint::new((10, 10), Coordinate::Widget);
        let val = p.to_value();
        assert_eq!(p, val.get::<CoordPoint>())
    }
}
