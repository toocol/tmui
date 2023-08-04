use std::ops::DerefMut;
use once_cell::sync::Lazy;
use tlib::figure::Point;

pub struct Cursor {
    position: Point,
}

impl Cursor {
    #[inline]
    fn instance() -> &'static mut Cursor {
        static mut CURSOR: Lazy<Cursor> = Lazy::new(|| Cursor { position: Point::default() });
        unsafe { CURSOR.deref_mut() }
    }

    #[inline]
    pub fn position() -> Point {
        Self::instance().position
    }

    #[inline]
    pub(crate) fn set_position<T: Into<Point>>(pos: T) {
        Self::instance().position = pos.into()
    }
}

#[cfg(test)]
mod tests {
    use tlib::figure::Point;

    use super::Cursor;

    #[test]
    fn test_cursor() {
        let pos = Point::new(100, 323);
        Cursor::set_position(pos);
        assert_eq!(pos, Cursor::position());
    }
}