pub mod color;
pub mod font;
pub mod point;
pub mod size;
pub mod rectangle;
pub mod transform;
pub mod region;

pub use color::*;
pub use font::*;
pub use point::*;
pub use size::*;
pub use rectangle::*;
pub use transform::*;
pub use region::*;

#[cfg(test)]
mod tests {
    use crate::values::ToValue;

    use super::*;

    #[test]
    fn test_point() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(20, 20);
        let p3 = p1 + p2;
        assert_eq!(30, p3.x());
        assert_eq!(30, p3.y());
        let p4 = p3 - p1;
        assert_eq!(20, p4.x());
        assert_eq!(20, p4.y());

        let val = p4.to_value();
        let get = val.get::<Point>();
        assert_eq!(20, get.x());
        assert_eq!(20, get.y());
    }

    #[test]
    fn test_size() {
        let size = Size::new(100, 100);
        let val = size.to_value();
        let get = val.get::<Size>();
        assert_eq!(size, get);
    }

    #[test]
    fn test_rect() {
        let rect = Rect::new(10, 10, 50, 50);
        let val = rect.to_value();
        let get = val.get::<Rect>();
        assert_eq!(rect, get)
    }

    #[test]
    fn test_color() {
        let color = Color::from_rgba(13, 13, 13, 13);
        let val = color.to_value();
        let get = val.get();
        assert_eq!(color, get);
        println!("{}", color.hexcode());
    }
}