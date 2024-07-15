use super::{point::Point, CoordPoint, FPoint, FSize, Size};
use crate::{
    namespace::Coordinate,
    typedef::{SkiaIRect, SkiaRect},
    types::StaticType,
    values::{FromBytes, FromValue, ToBytes, ToValue},
    Type, Value,
};
use std::{
    ops::{Add, Div, Mul, Sub},
    sync::atomic::{AtomicI32, Ordering},
};

/// (left-top, right-top, right-bottom, left-bottom)
pub type ArcPoints = (
    (Point, Point),
    (Point, Point),
    (Point, Point),
    (Point, Point),
);

/// (left-top, right-top, right-bottom, left-bottom)
pub type ArcFPoints = (
    (FPoint, FPoint),
    (FPoint, FPoint),
    (FPoint, FPoint),
    (FPoint, FPoint),
);

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

    #[inline]
    pub fn from_point_size(point: Point, size: Size) -> Self {
        Self {
            x: point.x(),
            y: point.y(),
            width: size.width(),
            height: size.height(),
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
    pub fn set_point(&mut self, point: &Point) {
        self.x = point.x();
        self.y = point.y();
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
    pub fn width_mut(&mut self) -> &mut i32 {
        &mut self.width
    }

    #[inline]
    pub fn height_mut(&mut self) -> &mut i32 {
        &mut self.height
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
    pub fn set_coords(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.set_left(x1);
        self.set_top(y1);
        self.set_right(x2);
        self.set_bottom(y2);
    }

    #[inline]
    pub fn is_intersects(&self, rect: &Rect) -> bool {
        let left = self.x.max(rect.x);
        let top = self.y.max(rect.y);
        let right = (self.x + self.width).min(rect.x + rect.width);
        let bottom = (self.y + self.height).min(rect.y + rect.height);

        left <= right && top <= bottom
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
        if !self.is_valid() {
            return false;
        }
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
        self.width > 0 && self.height > 0
    }

    #[inline]
    pub fn invalidate(&mut self) {
        self.x = 0;
        self.y = 0;
        self.width = 0;
        self.height = 0;
    }

    #[inline]
    pub fn and(&mut self, other: &Rect) {
        if !other.is_valid() {
            return;
        }
        if !self.is_valid() {
            self.x = other.x;
            self.y = other.y;
            self.width = other.width;
            self.height = other.height;
            return;
        }
        match self.intersects(other) {
            Some(intersect) => *self = intersect,
            None => self.invalidate(),
        }
    }

    #[inline]
    pub fn or(&mut self, other: &Rect) {
        if !other.is_valid() {
            return;
        }
        if !self.is_valid() {
            self.x = other.x;
            self.y = other.y;
            self.width = other.width;
            self.height = other.height;
            return;
        }
        *self = self.union(other);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.x = 0;
        self.y = 0;
        self.width = 0;
        self.height = 0;
    }

    /// (left-top, right-top, right-bottom, left-bottom)
    ///
    /// Each points are ordered by clockwise.
    #[inline]
    pub fn arc_points(&self, radius: i32) -> ArcPoints {
        (
            (
                Point::new(self.left(), self.top() + radius),
                Point::new(self.left() + radius, self.top()),
            ),
            (
                Point::new(self.right() - radius, self.top()),
                Point::new(self.right(), self.top() + radius),
            ),
            (
                Point::new(self.right(), self.bottom() - radius),
                Point::new(self.right() - radius, self.bottom()),
            ),
            (
                Point::new(self.left() + radius, self.bottom()),
                Point::new(self.left(), self.bottom() - radius),
            ),
        )
    }

    pub fn shrink(&self, distance: i32) -> Self {
        let new_width = if self.width > 2 * distance {
            self.width - 2 * distance
        } else {
            0
        };

        let new_height = if self.height > 2 * distance {
            self.height - 2 * distance
        } else {
            0
        };

        let new_x = self.x + distance;
        let new_y = self.y + distance;

        Self {
            x: new_x,
            y: new_y,
            width: new_width,
            height: new_height,
        }
    }

    pub fn enlarge(&self, distance: i32) -> Self {
        let new_width = self.width + 2 * distance;
        let new_height = self.height + 2 * distance;

        let new_x = self.x - distance;
        let new_y = self.y - distance;

        Self {
            x: new_x,
            y: new_y,
            width: new_width - 1,
            height: new_height - 1,
        }
    }
}

impl From<(i32, i32, i32, i32)> for Rect {
    #[inline]
    fn from((x, y, width, height): (i32, i32, i32, i32)) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl From<Rect> for (i32, i32, i32, i32) {
    #[inline]
    fn from(value: Rect) -> Self {
        (value.x, value.y, value.width, value.height)
    }
}

impl From<Rect> for SkiaRect {
    #[inline]
    fn from(value: Rect) -> Self {
        SkiaRect::from_xywh(
            value.x as f32,
            value.y as f32,
            value.width as f32,
            value.height as f32,
        )
    }
}

impl From<SkiaRect> for Rect {
    #[inline]
    fn from(value: SkiaRect) -> Self {
        Rect::new(
            value.x() as i32,
            value.y() as i32,
            value.width() as i32,
            value.height() as i32,
        )
    }
}

impl From<Rect> for SkiaIRect {
    #[inline]
    fn from(value: Rect) -> Self {
        SkiaIRect::from_xywh(value.x, value.y, value.width, value.height)
    }
}

impl From<SkiaIRect> for Rect {
    #[inline]
    fn from(value: SkiaIRect) -> Self {
        Rect::new(value.x(), value.y(), value.width(), value.height())
    }
}

impl From<Rect> for FRect {
    #[inline]
    fn from(value: Rect) -> Self {
        FRect {
            x: value.x as f32,
            y: value.y as f32,
            width: value.width as f32,
            height: value.height as f32,
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

impl Add for Rect {
    type Output = Rect;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.width + rhs.width,
            self.height + rhs.height,
        )
    }
}

impl Sub for Rect {
    type Output = Rect;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.width - rhs.width,
            self.height - rhs.height,
        )
    }
}

impl Mul for Rect {
    type Output = Rect;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.width * rhs.width,
            self.height * rhs.height,
        )
    }
}

impl Div for Rect {
    type Output = Rect;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x / rhs.x,
            self.y / rhs.y,
            self.width / rhs.width,
            self.height / rhs.height,
        )
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
            width: size.width(),
            height: size.height(),
        }
    }

    #[inline]
    pub fn offset(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    #[inline]
    pub fn size(&self) -> FSize {
        FSize::new(self.width, self.height)
    }

    #[inline]
    pub fn set_point(&mut self, point: &FPoint) {
        self.x = point.x();
        self.y = point.y();
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
    pub fn width_mut(&mut self) -> &mut f32 {
        &mut self.width
    }

    #[inline]
    pub fn height_mut(&mut self) -> &mut f32 {
        &mut self.height
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
    pub fn set_coords(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.set_left(x1);
        self.set_top(y1);
        self.set_right(x2);
        self.set_bottom(y2);
    }

    #[inline]
    pub fn is_intersects(&self, rect: &FRect) -> bool {
        let left = self.x.max(rect.x);
        let top = self.y.max(rect.y);
        let right = (self.x + self.width).min(rect.x + rect.width);
        let bottom = (self.y + self.height).min(rect.y + rect.height);

        left <= right && top <= bottom
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
        if !self.is_valid() {
            return false;
        }
        point.x() >= self.x()
            && point.y() >= self.y()
            && point.x() <= self.x() + self.width()
            && point.y() <= self.y() + self.height()
    }

    #[inline]
    pub fn contains_point(&self, point: &Point) -> bool {
        if !self.is_valid() {
            return false;
        }
        point.x() as f32 >= self.x()
            && point.y() as f32 >= self.y()
            && point.x() as f32 <= self.x() + self.width()
            && point.y() as f32 <= self.y() + self.height()
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
        self.width > 0. && self.height > 0.
    }

    #[inline]
    pub fn invalidate(&mut self) {
        self.x = 0.;
        self.y = 0.;
        self.width = 0.;
        self.height = 0.;
    }

    #[inline]
    pub fn and(&mut self, other: &FRect) {
        if !other.is_valid() {
            return;
        }
        if !self.is_valid() {
            self.x = other.x;
            self.y = other.y;
            self.width = other.width;
            self.height = other.height;
            return;
        }
        match self.intersects(other) {
            Some(intersect) => *self = intersect,
            None => self.invalidate(),
        }
    }

    #[inline]
    pub fn or(&mut self, other: &FRect) {
        if !other.is_valid() {
            return;
        }
        if !self.is_valid() {
            self.x = other.x;
            self.y = other.y;
            self.width = other.width;
            self.height = other.height;
            return;
        }
        *self = self.union(other);
    }

    /// (left-top, right-top, right-bottom, left-bottom)
    ///
    /// Each points are ordered by clockwise.
    #[inline]
    pub fn arc_points(&self, radius: f32) -> ArcFPoints {
        (
            (
                FPoint::new(self.left(), self.top() + radius),
                FPoint::new(self.left() + radius, self.top()),
            ),
            (
                FPoint::new(self.right() - radius, self.top()),
                FPoint::new(self.right(), self.top() + radius),
            ),
            (
                FPoint::new(self.right(), self.bottom() - radius),
                FPoint::new(self.right() - radius, self.bottom()),
            ),
            (
                FPoint::new(self.left() + radius, self.bottom()),
                FPoint::new(self.left(), self.bottom() - radius),
            ),
        )
    }

    #[inline]
    pub fn clear(&mut self) {
        self.x = 0.;
        self.y = 0.;
        self.width = 0.;
        self.height = 0.;
    }

    pub fn shrink(&self, distance: f32) -> Self {
        let new_width = if self.width > 2.0 * distance {
            self.width - 2.0 * distance
        } else {
            0.0
        };

        let new_height = if self.height > 2.0 * distance {
            self.height - 2.0 * distance
        } else {
            0.0
        };

        let new_x = self.x + distance;
        let new_y = self.y + distance;

        Self {
            x: new_x,
            y: new_y,
            width: new_width - 1.,
            height: new_height - 1.,
        }
    }

    pub fn enlarge(&self, distance: f32) -> Self {
        let new_width = self.width + 2.0 * distance;
        let new_height = self.height + 2.0 * distance;

        let new_x = self.x - distance;
        let new_y = self.y - distance;

        Self {
            x: new_x,
            y: new_y,
            width: new_width - 1.,
            height: new_height - 1.,
        }
    }
}

impl From<(i32, i32, i32, i32)> for FRect {
    #[inline]
    fn from((x, y, width, height): (i32, i32, i32, i32)) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            width: width as f32,
            height: height as f32,
        }
    }
}

impl From<(f32, f32, f32, f32)> for FRect {
    #[inline]
    fn from((x, y, width, height): (f32, f32, f32, f32)) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl From<FRect> for (f32, f32, f32, f32) {
    #[inline]
    fn from(value: FRect) -> Self {
        (value.x, value.y, value.width, value.height)
    }
}

impl From<FRect> for SkiaRect {
    #[inline]
    fn from(value: FRect) -> Self {
        SkiaRect::from_xywh(value.x, value.y, value.width, value.height)
    }
}

impl From<FRect> for SkiaIRect {
    #[inline]
    fn from(value: FRect) -> Self {
        SkiaIRect::from_xywh(
            value.x as i32,
            value.y as i32,
            value.width as i32,
            value.height as i32,
        )
    }
}

impl From<FRect> for Rect {
    #[inline]
    fn from(value: FRect) -> Self {
        Rect {
            x: value.x as i32,
            y: value.y as i32,
            width: value.width as i32,
            height: value.height as i32,
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

impl Add for FRect {
    type Output = FRect;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.width + rhs.width,
            self.height + rhs.height,
        )
    }
}

impl Sub for FRect {
    type Output = FRect;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.width - rhs.width,
            self.height - rhs.height,
        )
    }
}

impl Mul for FRect {
    type Output = FRect;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.width * rhs.width,
            self.height * rhs.height,
        )
    }
}

impl Div for FRect {
    type Output = FRect;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x / rhs.x,
            self.y / rhs.y,
            self.width / rhs.width,
            self.height / rhs.height,
        )
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
    pub fn set_point(&mut self, point: &Point) {
        self.x.store(point.x(), Ordering::Release);
        self.y.store(point.y(), Ordering::Release);
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
    pub fn width_mut(&mut self) -> &mut AtomicI32 {
        &mut self.width
    }

    #[inline]
    pub fn height_mut(&mut self) -> &mut AtomicI32 {
        &mut self.height
    }

    #[inline]
    pub fn x_mut(&mut self) -> &mut AtomicI32 {
        &mut self.x
    }

    #[inline]
    pub fn y_mut(&mut self) -> &mut AtomicI32 {
        &mut self.y
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

    #[inline]
    pub fn as_rect(&self) -> Rect {
        Rect {
            x: self.x(),
            y: self.y(),
            width: self.width(),
            height: self.height(),
        }
    }

    #[inline]
    pub fn contains(&self, point: &Point) -> bool {
        if !self.is_valid() {
            return false;
        }
        point.x() >= self.x()
            && point.y() >= self.y()
            && point.x() <= self.x() + self.width()
            && point.y() <= self.y() + self.height()
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.width.load(Ordering::SeqCst) > 0 && self.height.load(Ordering::SeqCst) > 0
    }

    #[inline]
    pub fn invalidate(&mut self) {
        self.x.store(0, Ordering::SeqCst);
        self.y.store(0, Ordering::SeqCst);
        self.width.store(0, Ordering::SeqCst);
        self.height.store(0, Ordering::SeqCst);
    }

    #[inline]
    pub fn is_intersects(&self, rect: &AtomicRect) -> bool {
        let x = self.x.load(Ordering::SeqCst);
        let y = self.y.load(Ordering::SeqCst);
        let width = self.width.load(Ordering::SeqCst);
        let height = self.height.load(Ordering::SeqCst);

        let rx = rect.x.load(Ordering::SeqCst);
        let ry = rect.y.load(Ordering::SeqCst);
        let rwidth = rect.width.load(Ordering::SeqCst);
        let rheight = rect.height.load(Ordering::SeqCst);

        x.max(rx) < (x + width).min(rx + rwidth) && y.max(ry) < (y + height).min(ry + rheight)
    }

    #[inline]
    pub fn intersects(&self, rect: &AtomicRect) -> Option<AtomicRect> {
        if !self.is_intersects(rect) {
            None
        } else {
            let x = self.x.load(Ordering::SeqCst);
            let y = self.y.load(Ordering::SeqCst);
            let width = self.width.load(Ordering::SeqCst);
            let height = self.height.load(Ordering::SeqCst);

            let rx = rect.x.load(Ordering::SeqCst);
            let ry = rect.y.load(Ordering::SeqCst);
            let rwidth = rect.width.load(Ordering::SeqCst);
            let rheight = rect.height.load(Ordering::SeqCst);

            let left = x.max(rx);
            let top = y.max(ry);
            let right = (x + width).min(rx + rwidth);
            let bottom = (y + height).min(ry + rheight);

            let x = left;
            let y = top;
            let width = right - left;
            let height = bottom - top;

            Some(AtomicRect::new(x, y, width, height))
        }
    }

    #[inline]
    pub fn union(&self, rect: &AtomicRect) -> AtomicRect {
        let x = self.x.load(Ordering::SeqCst);
        let y = self.y.load(Ordering::SeqCst);
        let width = self.width.load(Ordering::SeqCst);
        let height = self.height.load(Ordering::SeqCst);

        let rx = rect.x.load(Ordering::SeqCst);
        let ry = rect.y.load(Ordering::SeqCst);
        let rwidth = rect.width.load(Ordering::SeqCst);
        let rheight = rect.height.load(Ordering::SeqCst);

        let left = x.min(rx);
        let right = (x + width).max(rx + rwidth);
        let top = y.min(ry);
        let bottom = (y + height).max(ry + rheight);

        AtomicRect::new(left, top, right - left, bottom - top)
    }

    #[inline]
    pub fn subtracted(&self, other: &AtomicRect) -> Option<Vec<AtomicRect>> {
        let x = self.x.load(Ordering::SeqCst);
        let y = self.y.load(Ordering::SeqCst);
        let width = self.width.load(Ordering::SeqCst);
        let height = self.height.load(Ordering::SeqCst);

        let ox = other.x.load(Ordering::SeqCst);
        let oy = other.y.load(Ordering::SeqCst);
        let owidth = other.width.load(Ordering::SeqCst);
        let oheight = other.height.load(Ordering::SeqCst);

        let left = x.max(ox);
        let top = y.max(oy);
        let right = (x + width).min(ox + owidth);
        let bottom = (y + height).min(oy + oheight);

        if left >= right || top >= bottom {
            return None;
        }
        let mut result = vec![];

        if x < left {
            result.push(AtomicRect::new(x, y, left - x, height));
        }

        if y < top {
            result.push(AtomicRect::new(x, y, width, top - y));
        }

        if x + width > right {
            result.push(AtomicRect::new(right, y, x + width - right, height));
        }

        if y + height > bottom {
            result.push(AtomicRect::new(x, bottom, width, y + height - bottom));
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    #[inline]
    pub fn and(&mut self, other: &AtomicRect) {
        if !other.is_valid() {
            return;
        }
        let ox = other.x.load(Ordering::SeqCst);
        let oy = other.y.load(Ordering::SeqCst);
        let owidth = other.width.load(Ordering::SeqCst);
        let oheight = other.height.load(Ordering::SeqCst);

        if !self.is_valid() {
            self.x.store(ox, Ordering::SeqCst);
            self.y.store(oy, Ordering::SeqCst);
            self.width.store(owidth, Ordering::SeqCst);
            self.height.store(oheight, Ordering::SeqCst);
            return;
        }

        match self.intersects(other) {
            Some(intersect) => *self = intersect,
            None => self.invalidate(),
        }
    }

    #[inline]
    pub fn or(&mut self, other: &AtomicRect) {
        if !other.is_valid() {
            return;
        }
        let ox = other.x.load(Ordering::SeqCst);
        let oy = other.y.load(Ordering::SeqCst);
        let owidth = other.width.load(Ordering::SeqCst);
        let oheight = other.height.load(Ordering::SeqCst);

        if !self.is_valid() {
            self.x.store(ox, Ordering::SeqCst);
            self.y.store(oy, Ordering::SeqCst);
            self.width.store(owidth, Ordering::SeqCst);
            self.height.store(oheight, Ordering::SeqCst);
            return;
        }

        *self = self.union(other)
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
impl From<(i32, i32, i32, i32)> for AtomicRect {
    #[inline]
    fn from(value: (i32, i32, i32, i32)) -> Self {
        AtomicRect::new(value.0, value.1, value.2, value.3)
    }
}
impl From<Rect> for AtomicRect {
    #[inline]
    fn from(value: Rect) -> Self {
        AtomicRect::new(value.x, value.y, value.width, value.height)
    }
}
impl From<AtomicRect> for Rect {
    #[inline]
    fn from(value: AtomicRect) -> Self {
        Rect::new(value.x(), value.y(), value.width(), value.height())
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// CoordRect
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct CoordRect {
    rect: FRect,
    coord: Coordinate,
}
impl CoordRect {
    #[inline]
    pub fn new<T: Into<FRect>>(rect: T, coord: Coordinate) -> Self {
        Self {
            rect: rect.into(),
            coord,
        }
    }

    #[inline]
    pub fn rect(&self) -> FRect {
        self.rect
    }

    #[inline]
    pub fn coord(&self) -> Coordinate {
        self.coord
    }

    #[inline]
    pub fn point(&self) -> CoordPoint {
        CoordPoint::new(self.rect.point(), self.coord)
    }

    #[inline]
    pub fn top_left(&self) -> CoordPoint {
        CoordPoint::new(self.rect.top_left(), self.coord)
    }

    #[inline]
    pub fn top_right(&self) -> CoordPoint {
        CoordPoint::new(self.rect.top_left(), self.coord)
    }

    #[inline]
    pub fn bottom_left(&self) -> CoordPoint {
        CoordPoint::new(self.rect.top_left(), self.coord)
    }

    #[inline]
    pub fn bottom_right(&self) -> CoordPoint {
        CoordPoint::new(self.rect.top_left(), self.coord)
    }
}

impl StaticType for CoordRect {
    fn static_type() -> crate::Type {
        crate::Type::from_name("CoordRect")
    }

    fn bytes_len() -> usize {
        FRect::bytes_len() + Coordinate::bytes_len()
    }
}

impl ToBytes for CoordRect {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.rect.to_bytes());
        bytes.append(&mut self.coord.to_bytes());
        bytes
    }
}

impl FromBytes for CoordRect {
    fn from_bytes(data: &[u8], _len: usize) -> Self {
        let rect = FRect::from_bytes(&data[0..16], 16);
        let coord = Coordinate::from_bytes(&data[16..17], 1);
        Self { rect, coord }
    }
}

impl ToValue for CoordRect {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl FromValue for CoordRect {
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

    #[test]
    fn test_contains() {
        let rect = Rect::new(0, 0, 0, 0);
        let pos = Point::new(0, 0);
        assert!(!rect.contains(&pos));
    }

    #[test]
    fn test_or_and() {
        ////// Test for `Rect`
        let mut rect = Rect::new(10, 10, 50, 50);
        let other = Rect::new(30, 30, 70, 70);
        rect.or(&other);
        rect.or(&(0, 0, 0, 0).into());
        assert_eq!(rect.x, 10);
        assert_eq!(rect.y, 10);
        assert_eq!(rect.width, 90);
        assert_eq!(rect.height, 90);

        let mut rect = Rect::new(0, 0, 50, 50);
        rect.and(&other);
        rect.and(&(0, 0, 0, 0).into());
        assert_eq!(rect.x, 30);
        assert_eq!(rect.y, 30);
        assert_eq!(rect.width, 20);
        assert_eq!(rect.height, 20);

        ////// Test for `FRect`
        let mut rect = FRect::new(10., 10., 50., 50.);
        let other = FRect::new(30., 30., 70., 70.);
        rect.or(&other);
        rect.or(&(0., 0., 0., 0.).into());
        assert_eq!(rect.x, 10.);
        assert_eq!(rect.y, 10.);
        assert_eq!(rect.width, 90.);
        assert_eq!(rect.height, 90.);

        let mut rect = FRect::new(0., 0., 50., 50.);
        rect.and(&other);
        rect.and(&(0., 0., 0., 0.).into());
        assert_eq!(rect.x, 30.);
        assert_eq!(rect.y, 30.);
        assert_eq!(rect.width, 20.);
        assert_eq!(rect.height, 20.);

        ////// Test for `AtomicRect`
        let mut rect = AtomicRect::new(10, 10, 50, 50);
        let other = AtomicRect::new(30, 30, 70, 70);
        rect.or(&other);
        rect.or(&(0, 0, 0, 0).into());
        assert_eq!(rect.x(), 10);
        assert_eq!(rect.y(), 10);
        assert_eq!(rect.width(), 90);
        assert_eq!(rect.height(), 90);

        let mut rect = AtomicRect::new(0, 0, 50, 50);
        rect.and(&other);
        rect.and(&(0, 0, 0, 0).into());
        assert_eq!(rect.x(), 30);
        assert_eq!(rect.y(), 30);
        assert_eq!(rect.width(), 20);
        assert_eq!(rect.height(), 20);
    }

    #[test]
    fn test_coord_rect_value() {
        let r = CoordRect::new((10, 10, 30, 30), Coordinate::Widget);
        let val = r.to_value();
        assert_eq!(r, val.get::<CoordRect>())
    }
}
