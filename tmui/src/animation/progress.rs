use std::ops::{Add, Div, Mul, Sub};
use tlib::{
    figure::{Color, FPoint, FRect, Point, Rect},
    global::CreateBy,
};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub(crate) struct Progress(pub(crate) f32);

impl CreateBy<f32> for Progress {
    #[inline]
    fn create_by(t: f32) -> Self {
        Self(t)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////
/// Implement arithmetic operations for Progress between [`Progress`]
////////////////////////////////////////////////////////////////////////////////////////////
impl Add<Progress> for Progress {
    type Output = Progress;

    #[inline]
    fn add(self, rhs: Progress) -> Self::Output {
        Progress(self.0 + rhs.0)
    }
}

impl Sub<Progress> for Progress {
    type Output = Progress;

    #[inline]
    fn sub(self, rhs: Progress) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<Progress> for Progress {
    type Output = Progress;

    #[inline]
    fn mul(self, rhs: Progress) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div<Progress> for Progress {
    type Output = Progress;

    #[inline]
    fn div(self, rhs: Progress) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////
/// Implement arithmetic operations for Progress between [`Rect`]
////////////////////////////////////////////////////////////////////////////////////////////
impl Add<Rect> for Progress {
    type Output = Rect;

    #[inline]
    fn add(self, rhs: Rect) -> Self::Output {
        Rect::new(
            (self.0 + rhs.x() as f32) as i32,
            (self.0 + rhs.y() as f32) as i32,
            (self.0 + rhs.width() as f32) as i32,
            (self.0 + rhs.height() as f32) as i32,
        )
    }
}

impl Sub<Rect> for Progress {
    type Output = Rect;

    #[inline]
    fn sub(self, rhs: Rect) -> Self::Output {
        Rect::new(
            (self.0 - rhs.x() as f32) as i32,
            (self.0 - rhs.y() as f32) as i32,
            (self.0 - rhs.width() as f32) as i32,
            (self.0 - rhs.height() as f32) as i32,
        )
    }
}

impl Mul<Rect> for Progress {
    type Output = Rect;

    #[inline]
    fn mul(self, rhs: Rect) -> Self::Output {
        Rect::new(
            (self.0 * rhs.x() as f32) as i32,
            (self.0 * rhs.y() as f32) as i32,
            (self.0 * rhs.width() as f32) as i32,
            (self.0 * rhs.height() as f32) as i32,
        )
    }
}

impl Div<Rect> for Progress {
    type Output = Rect;

    #[inline]
    fn div(self, rhs: Rect) -> Self::Output {
        Rect::new(
            (self.0 / rhs.x() as f32) as i32,
            (self.0 / rhs.y() as f32) as i32,
            (self.0 / rhs.width() as f32) as i32,
            (self.0 / rhs.height() as f32) as i32,
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////
/// Implement arithmetic operations for Progress between [`FRect`]
////////////////////////////////////////////////////////////////////////////////////////////
impl Add<FRect> for Progress {
    type Output = FRect;

    #[inline]
    fn add(self, rhs: FRect) -> Self::Output {
        FRect::new(
            self.0 + rhs.x() as f32,
            self.0 + rhs.y() as f32,
            self.0 + rhs.width() as f32,
            self.0 + rhs.height() as f32,
        )
    }
}

impl Sub<FRect> for Progress {
    type Output = FRect;

    #[inline]
    fn sub(self, rhs: FRect) -> Self::Output {
        FRect::new(
            self.0 - rhs.x() as f32,
            self.0 - rhs.y() as f32,
            self.0 - rhs.width() as f32,
            self.0 - rhs.height() as f32,
        )
    }
}

impl Mul<FRect> for Progress {
    type Output = FRect;

    #[inline]
    fn mul(self, rhs: FRect) -> Self::Output {
        FRect::new(
            self.0 * rhs.x() as f32,
            self.0 * rhs.y() as f32,
            self.0 * rhs.width() as f32,
            self.0 * rhs.height() as f32,
        )
    }
}

impl Div<FRect> for Progress {
    type Output = FRect;

    #[inline]
    fn div(self, rhs: FRect) -> Self::Output {
        FRect::new(
            self.0 / rhs.x() as f32,
            self.0 / rhs.y() as f32,
            self.0 / rhs.width() as f32,
            self.0 / rhs.height() as f32,
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////
/// Implement arithmetic operations for Progress between [`Point`]
////////////////////////////////////////////////////////////////////////////////////////////
impl Add<Point> for Progress {
    type Output = Point;

    #[inline]
    fn add(self, rhs: Point) -> Self::Output {
        Point::new(
            (self.0 + rhs.x() as f32) as i32,
            (self.0 + rhs.y() as f32) as i32,
        )
    }
}

impl Sub<Point> for Progress {
    type Output = Point;

    #[inline]
    fn sub(self, rhs: Point) -> Self::Output {
        Point::new(
            (self.0 - rhs.x() as f32) as i32,
            (self.0 - rhs.y() as f32) as i32,
        )
    }
}

impl Mul<Point> for Progress {
    type Output = Point;

    #[inline]
    fn mul(self, rhs: Point) -> Self::Output {
        Point::new(
            (self.0 * rhs.x() as f32) as i32,
            (self.0 * rhs.y() as f32) as i32,
        )
    }
}

impl Div<Point> for Progress {
    type Output = Point;

    #[inline]
    fn div(self, rhs: Point) -> Self::Output {
        Point::new(
            (self.0 / rhs.x() as f32) as i32,
            (self.0 / rhs.y() as f32) as i32,
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////
/// Implement arithmetic operations for Progress between [`FPoint`]
////////////////////////////////////////////////////////////////////////////////////////////
impl Add<FPoint> for Progress {
    type Output = FPoint;

    #[inline]
    fn add(self, rhs: FPoint) -> Self::Output {
        FPoint::new(self.0 + rhs.x() as f32, self.0 + rhs.y() as f32)
    }
}

impl Sub<FPoint> for Progress {
    type Output = FPoint;

    #[inline]
    fn sub(self, rhs: FPoint) -> Self::Output {
        FPoint::new(self.0 - rhs.x() as f32, self.0 - rhs.y() as f32)
    }
}

impl Mul<FPoint> for Progress {
    type Output = FPoint;

    #[inline]
    fn mul(self, rhs: FPoint) -> Self::Output {
        FPoint::new(self.0 * rhs.x() as f32, self.0 * rhs.y() as f32)
    }
}

impl Div<FPoint> for Progress {
    type Output = FPoint;

    #[inline]
    fn div(self, rhs: FPoint) -> Self::Output {
        FPoint::new(self.0 / rhs.x() as f32, self.0 / rhs.y() as f32)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////
/// Implement arithmetic operations for Progress between [`Color`]
////////////////////////////////////////////////////////////////////////////////////////////
impl Add<Color> for Progress {
    type Output = Color;

    #[inline]
    fn add(self, rhs: Color) -> Self::Output {
        Color::from_rgba_uncheck(
            (self.0 + rhs.r_i16() as f32) as i16,
            (self.0 + rhs.g_i16() as f32) as i16,
            (self.0 + rhs.b_i16() as f32) as i16,
            (self.0 + rhs.a_i16() as f32) as i16,
        )
    }
}

impl Sub<Color> for Progress {
    type Output = Color;

    #[inline]
    fn sub(self, rhs: Color) -> Self::Output {
        Color::from_rgba_uncheck(
            (self.0 - rhs.r_i16() as f32) as i16,
            (self.0 - rhs.g_i16() as f32) as i16,
            (self.0 - rhs.b_i16() as f32) as i16,
            (self.0 - rhs.a_i16() as f32) as i16,
        )
    }
}

impl Mul<Color> for Progress {
    type Output = Color;

    #[inline]
    fn mul(self, rhs: Color) -> Self::Output {
        Color::from_rgba_uncheck(
            (self.0 * rhs.r_i16() as f32) as i16,
            (self.0 * rhs.g_i16() as f32) as i16,
            (self.0 * rhs.b_i16() as f32) as i16,
            (self.0 * rhs.a_i16() as f32) as i16,
        )
    }
}

impl Div<Color> for Progress {
    type Output = Color;

    #[inline]
    fn div(self, rhs: Color) -> Self::Output {
        Color::from_rgba_uncheck(
            (self.0 / rhs.r_i16() as f32) as i16,
            (self.0 / rhs.g_i16() as f32) as i16,
            (self.0 / rhs.b_i16() as f32) as i16,
            (self.0 / rhs.a_i16() as f32) as i16,
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////
/// Implement arithmetic operations for Progress between [`f32`]
////////////////////////////////////////////////////////////////////////////////////////////
impl Add<f32> for Progress {
    type Output = f32;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        self.0 + rhs
    }
}

impl Sub<f32> for Progress {
    type Output = f32;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        self.0 - rhs
    }
}

impl Mul<f32> for Progress {
    type Output = f32;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.0 * rhs
    }
}

impl Div<f32> for Progress {
    type Output = f32;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.0 / rhs
    }
}

////////////////////////////////////////////////////////////////////////////////////////////
/// Implement arithmetic operations for Progress between [`i32`]
////////////////////////////////////////////////////////////////////////////////////////////
impl Add<i32> for Progress {
    type Output = i32;

    #[inline]
    fn add(self, rhs: i32) -> Self::Output {
        (self.0 + rhs as f32) as i32
    }
}

impl Sub<i32> for Progress {
    type Output = i32;

    #[inline]
    fn sub(self, rhs: i32) -> Self::Output {
        (self.0 - rhs as f32) as i32
    }
}

impl Mul<i32> for Progress {
    type Output = i32;

    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        (self.0 * rhs as f32) as i32
    }
}

impl Div<i32> for Progress {
    type Output = i32;

    #[inline]
    fn div(self, rhs: i32) -> Self::Output {
        (self.0 / rhs as f32) as i32
    }
}

////////////////////////////////////////////////////////////////////////////////////////////
/// Implement arithmetic operations for Progress between [`u8`]
////////////////////////////////////////////////////////////////////////////////////////////
impl Add<u8> for Progress {
    type Output = u8;

    #[inline]
    fn add(self, rhs: u8) -> Self::Output {
        (self.0 + rhs as f32) as u8
    }
}

impl Sub<u8> for Progress {
    type Output = u8;

    #[inline]
    fn sub(self, rhs: u8) -> Self::Output {
        (self.0 - rhs as f32) as u8
    }
}

impl Mul<u8> for Progress {
    type Output = u8;

    #[inline]
    fn mul(self, rhs: u8) -> Self::Output {
        (self.0 * rhs as f32) as u8 
    }
}

impl Div<u8> for Progress {
    type Output = u8;

    #[inline]
    fn div(self, rhs: u8) -> Self::Output {
        (self.0 / rhs as f32) as u8
    }
}

#[cfg(test)]
mod tests {
    use tlib::figure::Color;

    use super::Progress;

    #[test]
    fn test_op() {
        let p = Progress(10.);
        let color = Color::from_rgb(100, 100, 100);
        let sub = p - color;
        assert_eq!(sub.r_i16(), -90);
        assert_eq!(sub.g_i16(), -90);
        assert_eq!(sub.b_i16(), -90);
        assert_eq!(sub.a_i16(), -245);
    }
}