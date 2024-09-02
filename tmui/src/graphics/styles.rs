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
    background: Option<Color>,
    /// Some UI components will ignore this property.
    color: Option<Color>,

    font: Option<Font>,
    border: Option<Border>,
    box_shadow: Option<BoxShadow>,

    halign: Option<Align>,
    valign: Option<Align>,
}

impl Styles {
    #[inline]
    pub fn background(&self) -> Option<Color> {
        self.background
    }

    #[inline]
    pub fn set_background(&mut self, background: Color) {
        self.background = Some(background)
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
    pub fn font(&self) -> Option<&Font> {
        self.font.as_ref()
    }

    #[inline]
    pub fn font_mut(&mut self) -> Option<&mut Font> {
        self.font.as_mut()
    }

    #[inline]
    pub fn set_font(&mut self, font: Font) {
        self.font = Some(font)
    }

    #[inline]
    pub fn border(&self) -> Option<&Border> {
        self.border.as_ref()
    }

    #[inline]
    pub fn border_mut(&mut self) -> Option<&mut Border> {
        self.border.as_mut()
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
    pub fn halign(&self) -> Option<Align> {
        self.halign
    }

    #[inline]
    pub fn set_halign(&mut self, halign: Align) {
        self.halign = Some(halign)
    }

    #[inline]
    pub fn valign(&self) -> Option<Align> {
        self.valign
    }

    #[inline]
    pub fn set_valign(&mut self, valign: Align) {
        self.valign = Some(valign)
    }

    #[inline]
    pub fn with_background(mut self, background: Color) -> Self {
        self.background = Some(background);
        self
    }

    #[inline]
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    #[inline]
    pub fn with_font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    #[inline]
    pub fn with_border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }

    #[inline]
    pub fn with_box_shadow(mut self, box_shadow: BoxShadow) -> Self {
        self.box_shadow = Some(box_shadow);
        self
    }

    #[inline]
    pub fn with_halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    #[inline]
    pub fn with_valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }
}
impl Styles {
    #[inline]
    pub(crate) fn take_font(&mut self) -> Option<Font> {
        self.font.take()
    }

    #[inline]
    pub(crate) fn take_border(&mut self) -> Option<Border> {
        self.border.take()
    }

    #[inline]
    pub(crate) fn take_box_shadow(&mut self) -> Option<BoxShadow> {
        self.box_shadow.take()
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
    pub fn set_border(&mut self, border: Border) {
        self.border = border
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