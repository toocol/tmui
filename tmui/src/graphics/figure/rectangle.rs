use super::{point::Point, Size};
use tlib::{
    types::StaticType,
    values::{FromBytes, FromValue, ToBytes, ToValue},
    Value,
};

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// Rectangle
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rect {
    #[inline]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn from_point_size(point: Point, size: Size) -> Self {
        Self {
            x: point.x(),
            y: point.y(),
            width: point.x() + size.width(),
            height: point.y() + size.height(),
        }
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
    pub fn width(&self) -> f32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.height
    }

    #[inline]
    pub fn set_width(&mut self, width: f32) {
        self.width = width
    }

    #[inline]
    pub fn set_height(&mut self, height: f32) {
        self.height = height
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
    pub fn top_left(&self) -> Point {
        Point::new(self.x, self.y)
    }

    #[inline]
    pub fn top_right(&self) -> Point {
        Point::new(self.x + self.width, self.y)
    }

    #[inline]
    pub fn bottom_left(&self) -> Point {
        Point::new(self.x, self.y + self.height)
    }

    #[inline]
    pub fn bottom_right(&self) -> Point {
        Point::new(self.x + self.width, self.y + self.height)
    }

    #[inline]
    pub fn left(&self) -> f32 {
        self.x
    }

    #[inline]
    pub fn top(&self) -> f32 {
        self.y
    }

    #[inline]
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    #[inline]
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    #[inline]
    pub fn move_left(&mut self, pos: f32) {
        self.x = pos;
    }

    #[inline]
    pub fn move_top(&mut self, pos: f32) {
        self.y = pos;
    }

    #[inline]
    pub fn move_right(&mut self, pos: f32) {
        self.x += pos - self.right();
    }

    #[inline]
    pub fn move_bottom(&mut self, pos: f32) {
        self.y += pos - self.bottom();
    }

    #[inline]
    pub fn move_top_left(&mut self, point: &Point) {
        self.move_left(point.x());
        self.move_top(point.y());
    }

    #[inline]
    pub fn move_bottom_right(&mut self, point: &Point) {
        self.move_right(point.x());
        self.move_bottom(point.y());
    }

    #[inline]
    pub fn move_top_right(&mut self, point: &Point) {
        self.move_right(point.x());
        self.move_top(point.y());
    }

    #[inline]
    pub fn move_bottom_left(&mut self, point: &Point) {
        self.move_left(point.x());
        self.move_bottom(point.y());
    }

    #[inline]
    pub fn move_center(&mut self, point: &Point) {
        self.x = point.x() - self.width / 2.;
        self.y = point.y() - self.height / 2.;
    }

    #[inline]
    pub fn set_left(&mut self, pos: f32) {
        self.x = pos;
    }

    #[inline]
    pub fn set_top(&mut self, pos: f32) {
        self.y = pos;
    }

    #[inline]
    pub fn set_right(&mut self, pos: f32) {
        self.width = pos - self.x;
    }

    #[inline]
    pub fn set_bottom(&mut self, pos: f32) {
        self.height = pos - self.y;
    }

    #[inline]
    pub fn intersects(&self, rect: &Rect) -> bool {
        self.x.max(rect.x) <= self.width.min(rect.width)
            && self.y.max(rect.y) <= self.height.min(rect.height)
    }

    #[inline]
    pub fn contains(&self, point: &Point) -> bool {
        point.x() >= self.x()
            && point.y() >= self.y()
            && point.x() <= self.width()
            && point.y() <= self.height()
    }

    #[inline]
    pub fn adjusted(&self, xp1: f32, yp1: f32, xp2: f32, yp2: f32) -> Rect {
        Rect {
            x: self.x + xp1,
            y: self.y + yp1,
            width: self.width + xp2 - xp1,
            height: self.height + yp2 - yp1,
        }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.width >= 0. && self.height >= 0.
    }
}

impl From<(i32, i32, i32, i32)> for Rect {
    fn from((x, y, width, height): (i32, i32, i32, i32)) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            width: width as f32,
            height: height as f32,
        }
    }
}

impl From<(f32, f32, f32, f32)> for Rect {
    fn from((x, y, width, height): (f32, f32, f32, f32)) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl Into<(f32, f32, f32, f32)> for Rect {
    fn into(self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}

impl Into<skia_safe::Rect> for Rect {
    fn into(self) -> skia_safe::Rect {
        skia_safe::Rect::from_xywh(
            self.x as f32,
            self.y as f32,
            self.width as f32,
            self.height as f32,
        )
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
        let x = f32::from_bytes(&data[0..4], 4);
        let y = f32::from_bytes(&data[4..8], 4);
        let width = f32::from_bytes(&data[8..12], 4);
        let height = f32::from_bytes(&data[12..16], 4);
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
