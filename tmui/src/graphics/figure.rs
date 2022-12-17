#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
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
