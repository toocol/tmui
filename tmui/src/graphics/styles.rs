use super::{
    border::Border,
    box_shadow::BoxShadow,
};
use crate::font::Font;
use derivative::Derivative;
use tlib::{figure::Color, prelude::Align};

#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct Styles {
    #[derivative(Default(value = "Color::TRANSPARENT"))]
    background: Color,
    /// Some UI components will ignore this property.
    color: Option<Color>,

    font: Font,
    border: Border,
    box_shadow: Option<BoxShadow>,

    halign: Align,
    valign: Align,
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
    pub fn color(&self) -> Option<Color> {
        self.color
    }

    #[inline]
    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color)
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
    pub fn halign(&self) -> Align {
        self.halign
    }

    #[inline]
    pub fn set_halign(&mut self, halign: Align) {
        self.halign = halign
    }

    #[inline]
    pub fn valign(&self) -> Align {
        self.valign
    }

    #[inline]
    pub fn set_valign(&mut self, valign: Align) {
        self.valign = valign
    }

    #[inline]
    pub fn with_background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }

    #[inline]
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
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

    #[inline]
    pub fn with_halign(mut self, halign: Align) -> Self {
        self.halign = halign;
        self
    }

    #[inline]
    pub fn with_valign(mut self, valign: Align) -> Self {
        self.valign = valign;
        self
    }
}

#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
pub(crate) struct InnerStyles {
    #[derivative(Default(value = "Color::TRANSPARENT"))]
    background: Color,
    font: Font,
    border: Border,
    box_shadow: Option<BoxShadow>,
    halign: Align,
    valign: Align,
}

impl InnerStyles {
    #[inline]
    pub(crate) fn new(
        background: Color,
        font: Font,
        border: Border,
        box_shadow: Option<BoxShadow>,
        halign: Align,
        valign: Align,
    ) -> Self {
        Self {
            background,
            font,
            border,
            box_shadow,
            halign,
            valign,
        }
    }
}

impl From<Styles> for InnerStyles {
    #[inline]
    fn from(value: Styles) -> Self {
        Self::new(value.background, value.font, value.border, value.box_shadow, value.halign, value.valign)
    }
}

impl InnerStyles {
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
    pub fn halign(&self) -> Align {
        self.halign
    }

    #[inline]
    pub fn set_halign(&mut self, halign: Align) {
        self.halign = halign
    }

    #[inline]
    pub fn valign(&self) -> Align {
        self.valign
    }

    #[inline]
    pub fn set_valign(&mut self, valign: Align) {
        self.valign = valign
    }
}