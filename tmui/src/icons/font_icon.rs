use crate::{
    font::FontCalculation,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{widget_inner::WidgetInnerExt, WidgetImpl},
};
use tlib::{figure::FSize, global::PrecisionOps};

#[cfg(external_fonts)]
const DEFAULT_ICON_FAMILIES: [&'static str; 3] = [
    "Font Awesome 6 Brands",
    "Font Awesome 6 Free",
    "Font Awesome 6 Free Solid",
];
const DEFAULT_ICON_COLOR: Color = Color::BLACK;
const DEFAULT_ICON_SIZE: f32 = 20.;

/// The default font-family was <a href="https://fontawesome.com/search?o=r&m=free&s=regular">Font-Awesome</a>, <br>
/// If need the default font-family, add feature `external_fonts`.
#[extends(Widget)]
pub struct FontIcon {
    font_dimension: (f32, f32),
    origin: (f32, f32),
    code: char,
    #[derivative(Default(value = "DEFAULT_ICON_COLOR"))]
    color: Color,
}

impl ObjectSubclass for FontIcon {
    const NAME: &'static str = "FontIcon";
}

impl ObjectImpl for FontIcon {}

impl WidgetImpl for FontIcon {
    fn paint(&mut self, painter: &mut Painter) {
        if self.code == '\0' {
            return;
        }

        let content = self.contents_rect_f(Some(Coordinate::Widget));

        painter.set_color(self.color);
        painter.draw_paragraph(
            &self.code.to_string(),
            self.origin,
            0.,
            content.width(),
            None,
            false,
        );
    }

    fn font_changed(&mut self) {
        if self.code == '\0' {
            return;
        }

        self.font_dimension = self
            .font()
            .calc_text_dimension(&self.code.to_string(), 0.)
            .ceil();

        let size: FSize = self.size().into();

        let mut resized = false;

        if size.width() < self.font_dimension.0 {
            self.set_fixed_width(self.font_dimension.0 as i32);
            self.set_detecting_width(self.font_dimension.0 as i32);
            resized = true;
        }

        if size.height() < self.font_dimension.1 {
            self.set_fixed_height(self.font_dimension.1 as i32);
            self.set_detecting_height(self.font_dimension.1 as i32);
            resized = true;
        }

        if resized && self.window_id() != 0 && self.window().initialized() {
            self.window().layout_change(self);
        }

        self.calc_origin();
    }
}

impl FontIcon {
    #[cfg(external_fonts)]
    #[inline]
    pub fn new(code: char) -> Box<Self> {
        let mut icon: Box<Self> = Object::new(&[]);
        icon.code = code;

        let mut font = Font::with_families(&DEFAULT_ICON_FAMILIES);
        font.set_size(DEFAULT_ICON_SIZE);
        icon.set_font(font);

        icon
    }

    #[cfg(not(external_fonts))]
    #[inline]
    pub fn new<T: ToString>(code: char, families: &[T]) -> Box<Self> {
        Self::with_families(code, families)
    }

    #[inline]
    pub fn with_families<T: ToString>(code: char, families: &[T]) -> Box<Self> {
        let mut icon: Box<Self> = Object::new(&[]);
        icon.code = code;

        let mut font = Font::with_families(families);
        font.set_size(DEFAULT_ICON_SIZE);
        icon.set_font(font);

        icon
    }

    #[inline]
    pub fn set_code(&mut self, code: char) {
        self.code = code;
        self.calc_origin();
        self.update()
    }

    #[inline]
    pub fn set_size(&mut self, size: f32) {
        self.font_mut().set_size(size);
        self.font_changed();
    }

    #[inline]
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
        self.update();
    }
}

impl FontIcon {
    #[inline]
    fn calc_origin(&mut self) {
        let content = self.contents_rect_f(Some(Coordinate::Widget));
        let tf = content.top_left();
        let (x1, y1, w1, h1) = (tf.x(), tf.y(), content.width(), content.height());
        let (w2, h2) = (self.font_dimension.0, self.font_dimension.1);

        self.origin = (x1 + w1 / 2. - w2 / 2., y1 + h1 / 2. - h2 / 2.);
    }
}
