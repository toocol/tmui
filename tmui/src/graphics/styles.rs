use derivative::Derivative;
use tlib::figure::Color;
use crate::font::Font;
use super::{border::Border, box_shadow::BoxShadow};

#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct Styles {
    #[derivative(Default(value = "Color::TRANSPARENT"))]
    background: Color,
    font: Font,
    border: Border,
    box_shadow: Option<BoxShadow>,
}

impl Styles {
    #[inline]
    pub fn background(&self) -> Color {
        self.background
    }

    #[inline]
    pub fn set_background(&mut self, background: Color) {
        self.background = background
    }

    #[inline]
    pub fn font(&self) -> &Font {
        &self.font
    }

    #[inline]
    pub fn font_mut(&mut self) -> &mut Font {
        &mut self.font
    }

    #[inline]
    pub fn set_font(&mut self, font: Font) {
        self.font = font
    }

    #[inline]
    pub fn border(&self) -> &Border {
        &self.border
    }

    #[inline]
    pub fn border_mut(&mut self) -> &mut Border {
        &mut self.border
    }

    #[inline]
    pub fn box_shadow(&self) -> Option<&BoxShadow> {
        self.box_shadow.as_ref()
    }

    #[inline]
    pub fn set_box_shadow(&mut self, shadow: BoxShadow) {
        self.box_shadow = Some(shadow)
    }

    #[inline]
    pub fn with_background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }

    #[inline]
    pub fn with_font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    #[inline]
    pub fn with_border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    #[inline]
    pub fn with_box_shadow(mut self, box_shadow: BoxShadow) -> Self {
        self.box_shadow = Some(box_shadow);
        self
    }
}