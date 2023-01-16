use super::point::Point;
use tlib::{
    types::StaticType,
    values::{FromBytes, FromValue, ToBytes, ToValue},
    Value,
};

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
    #[inline]
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
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
    pub fn width(&self) -> i32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.height
    }

    #[inline]
    pub fn set_width(&mut self, width: i32) {
        self.width = width
    }

    #[inline]
    pub fn set_height(&mut self, height: i32) {
        self.height = height
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
    pub fn left(&self) -> i32 {
        self.x
    }

    #[inline]
    pub fn top(&self) -> i32 {
        self.y
    }

    #[inline]
    pub fn right(&self) -> i32 {
        self.x + self.width
    }

    #[inline]
    pub fn bottom(&self) -> i32 {
        self.y + self.height
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
    pub fn asjusted(&self, xp1: i32, yp1: i32, xp2: i32, yp2: i32) -> Rect {
        Rect {
            x: self.x + xp1,
            y: self.y + yp1,
            width: self.width + xp2 - xp1,
            height: self.height + yp2 - yp1,
        }
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
