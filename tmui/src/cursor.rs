use lazy_static::lazy_static;
use tipc::parking_lot::Mutex;
use tlib::figure::Point;

lazy_static!{
    static ref CURSOR: Mutex<Cursor> = Mutex::new(Cursor { position: Point::default() });
}

pub struct Cursor {
    position: Point,
}

impl Cursor {
    #[inline]
    pub fn position() -> Point {
        CURSOR.lock().position
    }

    #[inline]
    pub(crate) fn set_position<T: Into<Point>>(pos: T) {
        CURSOR.lock().position = pos.into()
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