use tlib::figure::Color;

pub struct SvgAttr {
    width: u32,
    height: u32,
    color: Color,
}

impl SvgAttr {
    #[inline]
    pub fn new(width: u32, height: u32, color: Color) -> Self {
        Self {
            width,
            height,
            color,
        }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn color(&self) -> Color {
        self.color
    }
}
