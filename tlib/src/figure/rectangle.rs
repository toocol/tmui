use super::{point::Point, FPoint, FSize, Size};
use crate::{
    types::StaticType,
    values::{FromBytes, FromValue, ToBytes, ToValue},
    Value,
};
use std::sync::atomic::{AtomicI32, Ordering};

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// Rect
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
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

    pub fn from_point_size(point: Point, size: Size) -> Self {
        Self {
            x: point.x(),
            y: point.y(),
            width: point.x() + size.width(),
            height: point.y() + size.height(),
        }
    }

    #[inline]
    pub fn offset(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    #[inline]
    pub fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    #[inline]
    pub fn point(&self) -> Point {
        Point::new(self.x, self.y)
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
    pub fn move_left(&mut self, pos: i32) {
        self.x = pos;
    }

    #[inline]
    pub fn move_top(&mut self, pos: i32) {
        self.y = pos;
    }

    #[inline]
    pub fn move_right(&mut self, pos: i32) {
        self.x += pos - self.right();
    }

    #[inline]
    pub fn move_bottom(&mut self, pos: i32) {
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
        self.x = point.x() - self.width / 2;
        self.y = point.y() - self.height / 2;
    }

    #[inline]
    pub fn set_left(&mut self, pos: i32) {
        self.x = pos;
    }

    #[inline]
    pub fn set_top(&mut self, pos: i32) {
        self.y = pos;
    }

    #[inline]
    pub fn set_right(&mut self, pos: i32) {
        self.width = pos - self.x;
    }

    #[inline]
    pub fn set_bottom(&mut self, pos: i32) {
        self.height = pos - self.y;
    }

    #[inline]
    pub fn is_intersects(&self, rect: &Rect) -> bool {
        self.x.max(rect.x) < (self.x + self.width).min(rect.x + rect.width)
            && self.y.max(rect.y) < (self.y + self.height).min(rect.y + rect.height)
    }

    #[inline]
    pub fn intersects(&self, rect: &Rect) -> Option<Rect> {
        if !self.is_intersects(rect) {
            None
        } else {
            let left = self.x.max(rect.x);
            let top = self.y.max(rect.y);
            let right = (self.x + self.width).min(rect.x + rect.width);
            let bottom = (self.y + self.height).min(rect.y + rect.height);

            let x = left;
            let y = top;
            let width = right - left;
            let height = bottom - top;

            Some(Rect {
                x,
                y,
                width,
                height,
            })
        }
    }

    #[inline]
    pub fn union(&self, rect: &Rect) -> Rect {
        let left = self.x.min(rect.x);
        let right = (self.x + self.width).max(rect.x + rect.width);
        let top = self.y.min(rect.y);
        let bottom = (self.y + self.height).max(rect.y + rect.height);

        Rect {
            x: left,
            y: top,
            width: right - left,
            height: bottom - top,
        }
    }

    #[inline]
    pub fn subtracted(&self, other: &Rect) -> Option<Vec<Rect>> {
        let left = self.x.max(other.x);
        let top = self.y.max(other.y);
        let right = (self.x + self.width).min(other.x + other.width);
        let bottom = (self.y + self.height).min(other.y + other.height);

        if left >= right || top >= bottom {
            return None;
        }
        let mut result = vec![];

        if self.x < left {
            result.push(Rect {
                x: self.x,
                y: self.y,
                width: left - self.x,
                height: self.height,
            });
        }

        if self.y < top {
            result.push(Rect {
                x: self.x,
                y: self.y,
                width: self.width,
                height: top - self.y,
            });
        }

        if self.x + self.width > right {
            result.push(Rect {
                x: right,
                y: self.y,
                width: self.x + self.width - right,
                height: self.height,
            });
        }

        if self.y + self.height > bottom {
            result.push(Rect {
                x: self.x,
                y: bottom,
                width: self.width,
                height: self.y + self.height - bottom,
            });
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    #[inline]
    pub fn contains(&self, point: &Point) -> bool {
        point.x() >= self.x()
            && point.y() >= self.y()
            && point.x() <= self.x() + self.width()
            && point.y() <= self.y() + self.height()
    }

    #[inline]
    pub fn adjusted(&self, xp1: i32, yp1: i32, xp2: i32, yp2: i32) -> Rect {
        Rect {
            x: self.x + xp1,
            y: self.y + yp1,
            width: self.width + xp2 - xp1,
            height: self.height + yp2 - yp1,
        }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.width >= 0 && self.height >= 0
    }

    #[inline]
    pub fn and(&mut self, other: &Rect) {
        self.x &= other.x;
        self.y &= other.y;
        self.width &= other.width;
        self.height &= other.height;
    }

    #[inline]
    pub fn or(&mut self, other: &Rect) {
        self.x |= other.x;
        self.y |= other.y;
        self.width |= other.width;
        self.height |= other.height;
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

impl Into<skia_safe::IRect> for Rect {
    fn into(self) -> skia_safe::IRect {
        skia_safe::IRect::from_xywh(self.x, self.y, self.width, self.height)
    }
}

impl Into<FRect> for Rect {
    fn into(self) -> FRect {
        FRect {
            x: self.x as f32,
            y: self.y as f32,
            width: self.width as f32,
            height: self.height as f32,
        }
    }
}

impl StaticType for Rect {
    fn static_type() -> crate::Type {
        crate::Type::from_name("Rect")
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

    fn value_type(&self) -> crate::Type {
        Self::static_type()
    }
}

impl FromValue for Rect {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// FRect
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct FRect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl FRect {
    #[inline]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn from_point_size(point: FPoint, size: FSize) -> Self {
        Self {
            x: point.x(),
            y: point.y(),
            width: point.x() + size.width(),
            height: point.y() + size.height(),
        }
    }

    #[inline]
    pub fn size(&self) -> FSize {
        FSize::new(self.width, self.height)
    }

    #[inline]
    pub fn point(&self) -> FPoint {
        FPoint::new(self.x, self.y)
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
    pub fn top_left(&self) -> FPoint {
        FPoint::new(self.x, self.y)
    }

    #[inline]
    pub fn top_right(&self) -> FPoint {
        FPoint::new(self.x + self.width, self.y)
    }

    #[inline]
    pub fn bottom_left(&self) -> FPoint {
        FPoint::new(self.x, self.y + self.height)
    }

    #[inline]
    pub fn bottom_right(&self) -> FPoint {
        FPoint::new(self.x + self.width, self.y + self.height)
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
    pub fn move_top_left(&mut self, point: &FPoint) {
        self.move_left(point.x());
        self.move_top(point.y());
    }

    #[inline]
    pub fn move_bottom_right(&mut self, point: &FPoint) {
        self.move_right(point.x());
        self.move_bottom(point.y());
    }

    #[inline]
    pub fn move_top_right(&mut self, point: &FPoint) {
        self.move_right(point.x());
        self.move_top(point.y());
    }

    #[inline]
    pub fn move_bottom_left(&mut self, point: &FPoint) {
        self.move_left(point.x());
        self.move_bottom(point.y());
    }

    #[inline]
    pub fn move_center(&mut self, point: &FPoint) {
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
    pub fn is_intersects(&self, rect: &FRect) -> bool {
        self.x.max(rect.x) <= self.width.min(rect.width)
            && self.y.max(rect.y) <= self.height.min(rect.height)
    }

    #[inline]
    pub fn intersects(&self, rect: &FRect) -> Option<FRect> {
        if !self.is_intersects(rect) {
            None
        } else {
            let left = self.x.max(rect.x);
            let top = self.y.max(rect.y);
            let right = (self.x + self.width).min(rect.x + rect.width);
            let bottom = (self.y + self.height).min(rect.y + rect.height);

            let x = left;
            let y = top;
            let width = right - left;
            let height = bottom - top;

            Some(FRect {
                x,
                y,
                width,
                height,
            })
        }
    }

    #[inline]
    pub fn union(&self, rect: &FRect) -> FRect {
        let left = self.x.min(rect.x);
        let right = (self.x + self.width).max(rect.x + rect.width);
        let top = self.y.min(rect.y);
        let bottom = (self.y + self.height).max(rect.y + rect.height);

        FRect {
            x: left,
            y: top,
            width: right - left,
            height: bottom - top,
        }
    }

    #[inline]
    pub fn subtracted(&self, other: &FRect) -> Option<Vec<FRect>> {
        let left = self.x.max(other.x);
        let top = self.y.max(other.y);
        let right = (self.x + self.width).min(other.x + other.width);
        let bottom = (self.y + self.height).min(other.y + other.height);

        if left >= right || top >= bottom {
            return None;
        }
        let mut result = vec![];

        if self.x < left {
            result.push(FRect {
                x: self.x,
                y: self.y,
                width: left - self.x,
                height: self.height,
            });
        }

        if self.y < top {
            result.push(FRect {
                x: self.x,
                y: self.y,
                width: self.width,
                height: top - self.y,
            });
        }

        if self.x + self.width > right {
            result.push(FRect {
                x: right,
                y: self.y,
                width: self.x + self.width - right,
                height: self.height,
            });
        }

        if self.y + self.height > bottom {
            result.push(FRect {
                x: self.x,
                y: bottom,
                width: self.width,
                height: self.y + self.height - bottom,
            });
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    #[inline]
    pub fn contains(&self, point: &FPoint) -> bool {
        point.x() >= self.x()
            && point.y() >= self.y()
            && point.x() <= self.width()
            && point.y() <= self.height()
    }

    #[inline]
    pub fn adjusted(&self, xp1: f32, yp1: f32, xp2: f32, yp2: f32) -> Self {
        Self {
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

impl From<(f32, f32, f32, f32)> for FRect {
    fn from((x, y, width, height): (f32, f32, f32, f32)) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl Into<(f32, f32, f32, f32)> for FRect {
    fn into(self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}

impl Into<skia_safe::Rect> for FRect {
    fn into(self) -> skia_safe::Rect {
        skia_safe::Rect::from_xywh(
            self.x as f32,
            self.y as f32,
            self.width as f32,
            self.height as f32,
        )
    }
}

impl Into<Rect> for FRect {
    fn into(self) -> Rect {
        Rect {
            x: self.x as i32,
            y: self.y as i32,
            width: self.width as i32,
            height: self.height as i32,
        }
    }
}

impl StaticType for FRect {
    fn static_type() -> crate::Type {
        crate::Type::from_name("FRect")
    }

    fn bytes_len() -> usize {
        f32::bytes_len() * 4
    }
}

impl ToBytes for FRect {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.x.to_bytes());
        bytes.append(&mut self.y.to_bytes());
        bytes.append(&mut self.width.to_bytes());
        bytes.append(&mut self.height.to_bytes());
        bytes
    }
}

impl FromBytes for FRect {
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

impl ToValue for FRect {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> crate::Type {
        Self::static_type()
    }
}

impl FromValue for FRect {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// AtomicRect
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default)]
pub struct AtomicRect {
    x: AtomicI32,
    y: AtomicI32,
    width: AtomicI32,
    height: AtomicI32,
}

impl AtomicRect {
    /// Construct to create new `AtomicRect`
    #[inline]
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x: AtomicI32::new(x),
            y: AtomicI32::new(y),
            width: AtomicI32::new(width),
            height: AtomicI32::new(height),
        }
    }

    #[inline]
    pub fn offset(&mut self, x: i32, y: i32) {
        self.x.fetch_add(x, Ordering::Release);
        self.y.fetch_add(y, Ordering::Release);
    }

    #[inline]
    pub fn size(&self) -> Size {
        Size::new(
            self.width.load(Ordering::Acquire),
            self.height.load(Ordering::Acquire),
        )
    }

    #[inline]
    pub fn point(&self) -> Point {
        Point::new(
            self.x.load(Ordering::Acquire),
            self.y.load(Ordering::Acquire),
        )
    }

    #[inline]
    pub fn x(&self) -> i32 {
        self.x.load(Ordering::Acquire)
    }

    #[inline]
    pub fn y(&self) -> i32 {
        self.y.load(Ordering::Acquire)
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.width.load(Ordering::Acquire)
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.height.load(Ordering::Acquire)
    }

    #[inline]
    pub fn set_width(&mut self, width: i32) {
        self.width.store(width, Ordering::Release)
    }

    #[inline]
    pub fn set_height(&mut self, height: i32) {
        self.height.store(height, Ordering::Release)
    }

    #[inline]
    pub fn set_x(&mut self, x: i32) {
        self.x.store(x, Ordering::Release)
    }

    #[inline]
    pub fn set_y(&mut self, y: i32) {
        self.y.store(y, Ordering::Release)
    }

    #[inline]
    pub fn top_left(&self) -> Point {
        Point::new(
            self.x.load(Ordering::Acquire),
            self.y.load(Ordering::Acquire),
        )
    }

    #[inline]
    pub fn top_right(&self) -> Point {
        Point::new(
            self.x.load(Ordering::Acquire) + self.width.load(Ordering::Acquire),
            self.y.load(Ordering::Acquire),
        )
    }

    #[inline]
    pub fn bottom_left(&self) -> Point {
        Point::new(
            self.x.load(Ordering::Acquire),
            self.y.load(Ordering::Acquire) + self.height.load(Ordering::Acquire),
        )
    }

    #[inline]
    pub fn bottom_right(&self) -> Point {
        Point::new(
            self.x.load(Ordering::Acquire) + self.width.load(Ordering::Acquire),
            self.y.load(Ordering::Acquire) + self.height.load(Ordering::Acquire),
        )
    }

    #[inline]
    pub fn left(&self) -> i32 {
        self.x.load(Ordering::Acquire)
    }

    #[inline]
    pub fn top(&self) -> i32 {
        self.y.load(Ordering::Acquire)
    }

    #[inline]
    pub fn right(&self) -> i32 {
        self.x.load(Ordering::Acquire) + self.width.load(Ordering::Acquire)
    }

    #[inline]
    pub fn bottom(&self) -> i32 {
        self.y.load(Ordering::Acquire) + self.height.load(Ordering::Acquire)
    }

    #[inline]
    pub fn move_left(&mut self, pos: i32) {
        self.x.store(pos, Ordering::Release);
    }

    #[inline]
    pub fn move_top(&mut self, pos: i32) {
        self.y.store(pos, Ordering::Release);
    }

    #[inline]
    pub fn move_right(&mut self, pos: i32) {
        self.x.fetch_add(pos - self.right(), Ordering::Release);
    }

    #[inline]
    pub fn move_bottom(&mut self, pos: i32) {
        self.y.fetch_add(pos - self.bottom(), Ordering::Release);
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
        self.x
            .store(point.x() - self.width() / 2, Ordering::Release);
        self.y
            .store(point.y() - self.height() / 2, Ordering::Release);
    }

    #[inline]
    pub fn set_left(&mut self, pos: i32) {
        self.x.store(pos, Ordering::Release);
    }

    #[inline]
    pub fn set_top(&mut self, pos: i32) {
        self.y.store(pos, Ordering::Release);
    }

    #[inline]
    pub fn set_right(&mut self, pos: i32) {
        self.width.store(pos - self.x(), Ordering::Release);
    }

    #[inline]
    pub fn set_bottom(&mut self, pos: i32) {
        self.height.store(pos - self.y(), Ordering::Release);
    }

    #[inline]
    pub fn equals(&self, other: &Self) -> bool {
        self.x() == other.x()
            && self.y() == other.y()
            && self.width() == other.width()
            && self.height() == other.height()
    }
}

impl StaticType for AtomicRect {
    fn static_type() -> crate::Type {
        crate::Type::from_name("AtomicRect")
    }

    fn bytes_len() -> usize {
        i32::bytes_len() * 4
    }
}
impl ToBytes for AtomicRect {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.x().to_bytes();
        bytes.append(&mut self.y().to_bytes());
        bytes.append(&mut self.width().to_bytes());
        bytes.append(&mut self.height().to_bytes());
        bytes
    }
}
impl ToValue for AtomicRect {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> crate::Type {
        Self::static_type()
    }
}
impl FromBytes for AtomicRect {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        let i32_len = i32::bytes_len();
        let mut idx = 0;

        let x = i32::from_bytes(&data[idx..idx + i32_len], i32_len);
        idx += i32_len;

        let y = i32::from_bytes(&data[idx..idx + i32_len], i32_len);
        idx += i32_len;

        let width = i32::from_bytes(&data[idx..idx + i32_len], i32_len);
        idx += i32_len;

        let height = i32::from_bytes(&data[idx..idx + i32_len], i32_len);

        Self::new(x, y, width, height)
    }
}
impl FromValue for AtomicRect {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values() {
        let rect = Rect::new(20, 25, 300, 400);
        let val = rect.to_value();
        assert_eq!(rect, val.get::<Rect>());

        let rect = FRect::new(20., 25., 300., 400.);
        let val = rect.to_value();
        assert_eq!(rect, val.get::<FRect>());

        let rect = AtomicRect::new(20, 25, 300, 400);
        let val = rect.to_value();
        assert!(val.get::<AtomicRect>().equals(&rect));
    }
}
