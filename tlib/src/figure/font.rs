use skia_safe::{
    font_style::{Slant, Weight, Width},
    FontStyle, Typeface,
};
use FontWeight::*;
use FontWidth::*;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct FontTypeface {
    family: String,
    weight: FontWeight,
    width: FontWidth,
    italic: bool,
}

impl FontTypeface {
    /// Get the [`FontTypefaceBuilder`]
    #[inline]
    pub fn builder() -> FontTypefaceBuilder {
        FontTypefaceBuilder::default()
    }

    #[inline]
    pub fn faimly(&self) -> &str {
        &self.family
    }
    #[inline]
    pub fn set_family<T: ToString>(&mut self, family: T) {
        self.family = family.to_string();
    }

    #[inline]
    pub fn bold(&self) -> bool {
        self.weight == FontWeight::Bold
    }
    #[inline]
    pub fn set_bold(&mut self, bold: bool) {
        if bold {
            self.weight = FontWeight::Bold
        } else {
            self.weight = FontWeight::Normal
        }
    }

    #[inline]
    pub fn font_weight(&self) -> FontWeight {
        self.weight
    }
    #[inline]
    pub fn set_font_weight(&mut self, weight: FontWeight) {
        self.weight = weight
    }

    #[inline]
    pub fn font_width(&self) -> FontWidth {
        self.width
    }
    #[inline]
    pub fn set_font_width(&mut self, width: FontWidth) {
        self.width = width
    }

    #[inline]
    pub fn italic(&self) -> bool {
        self.italic
    }
    #[inline]
    pub fn set_italic(&mut self, italic: bool) {
        self.italic = italic
    }
}

impl Into<Typeface> for FontTypeface {
    #[inline]
    fn into(self) -> Typeface {
        let font_style = FontStyle::new(
            self.weight.into(),
            self.width.into(),
            if self.italic() {
                Slant::Italic
            } else {
                Slant::Upright
            },
        );
        Typeface::new(self.faimly(), font_style)
            .expect(format!("Create typeface from `{}` failed.", self.faimly()).as_str())
    }
}

/// The builder to construct the [`FontTypeface`]
#[derive(Debug, Default)]
pub struct FontTypefaceBuilder {
    family: Option<String>,
    weight: FontWeight,
    width: FontWidth,
    italic: bool,
}

impl FontTypefaceBuilder {
    /// Build the [`FontTypeface`]
    #[inline]
    pub fn build(self) -> FontTypeface {
        let mut typeface = FontTypeface::default();
        if let Some(ref family) = self.family {
            typeface.family = family.clone();
        }
        typeface.weight = self.weight;
        typeface.italic = self.italic;
        typeface
    }

    #[inline]
    pub fn family<T: ToString>(mut self, family: T) -> Self {
        self.family = Some(family.to_string());
        self
    }

    #[inline]
    pub fn bold(mut self, bold: bool) -> Self {
        if bold {
            self.weight = FontWeight::Bold
        } else {
            self.weight = FontWeight::Normal
        }
        self
    }

    #[inline]
    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    #[inline]
    pub fn width(mut self, width: FontWidth) -> Self {
        self.width = width;
        self
    }

    #[inline]
    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }
}

#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum FontWeight {
    Invisible,
    Thin,
    ExtraLight,
    Light,
    #[default]
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
    ExtraBlack,
}

impl Into<Weight> for FontWeight {
    fn into(self) -> Weight {
        match self {
            Invisible => Weight::INVISIBLE,
            Thin => Weight::THIN,
            ExtraLight => Weight::EXTRA_LIGHT,
            Light => Weight::LIGHT,
            Self::Normal => Weight::NORMAL,
            Medium => Weight::MEDIUM,
            SemiBold => Weight::SEMI_BOLD,
            Bold => Weight::BOLD,
            ExtraBold => Weight::EXTRA_BOLD,
            Black => Weight::BLACK,
            ExtraBlack => Weight::EXTRA_BLACK,
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum FontWidth {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    #[default]
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl Into<Width> for FontWidth {
    fn into(self) -> Width {
        match self {
            UltraCondensed => Width::ULTRA_CONDENSED,
            ExtraCondensed => Width::EXTRA_CONDENSED,
            Condensed => Width::CONDENSED,
            SemiCondensed => Width::SEMI_CONDENSED,
            Self::Normal => Width::NORMAL,
            SemiExpanded => Width::SEMI_EXPANDED,
            Expanded => Width::EXPANDED,
            ExtraExpanded => Width::EXTRA_EXPANDED,
            UltraExpanded => Width::ULTRA_EXPANDED,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FontTypeface;

    #[test]
    fn test_font_typeface() {
        let typeface = FontTypeface::builder()
            .family("Consolas")
            .bold(true)
            .italic(true)
            .build();
        assert!(typeface.bold());
        assert!(typeface.italic());
        let typeface: skia_safe::Typeface = typeface.into();
        assert!(typeface.is_bold());
        assert!(typeface.is_italic());
    }
}