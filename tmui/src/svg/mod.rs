pub mod svg_attr;
pub mod svg_str;

#[cfg(test)]
mod tests {
    use tlib::figure::Color;

    use super::{svg_attr::SvgAttr, svg_str::SvgStr};

    const SVG: &'static str = "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 2048 2048\" width=\"0\" height=\"0\"><path d=\"M559 815L414 670l610-610 610 610-145 145-465-465-465 465zm930 418l145 145-610 610-610-610 145-145 465 465 465-465z\" fill=\"#000000\"></path></svg>";

    #[test]
    fn test_svg_str() {
        let svg_str = SvgStr::new(SVG);
        let svg = svg_str.with_attr(SvgAttr::new(10, 10, Color::BLACK));

        let target: &'static str = "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 2048 2048\" width=\"10\" height=\"10\"><path d=\"M559 815L414 670l610-610 610 610-145 145-465-465-465 465zm930 418l145 145-610 610-610-610 145-145 465 465 465-465z\" fill=\"#000000\"></path></svg>";
        assert_eq!(svg, target);
    }
}
