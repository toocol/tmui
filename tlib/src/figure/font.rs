use lazy_static::__Deref;
use skia_safe::{
    self,
    font::Edging,
    font_style::{Slant, Weight, Width},
    FontStyle, Typeface,
};
use FontWeight::*;
use FontWidth::*;

/////////////////////////////////////////////////////////////////////////////////////////
/// [`Font`]
/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Font {
    force_auto_hinting: bool,
    embedded_bitmaps: bool,
    subpixel: bool,
    linear_metrics: bool,
    embolden: bool,
    baseline_snap: bool,
    edging: FontEdging,
    hinting: FontHinting,
    typeface: Option<FontTypeface>,
    size: f32,
    scale_x: f32,
    skew_x: f32,
}

impl Default for Font {
    #[inline]
    fn default() -> Self {
        skia_safe::Font::default().into()
    }
}

impl Font {
    #[inline]
    fn new() -> Self {
        Self {
            force_auto_hinting: Default::default(),
            embedded_bitmaps: Default::default(),
            subpixel: Default::default(),
            linear_metrics: Default::default(),
            embolden: Default::default(),
            baseline_snap: Default::default(),
            edging: Default::default(),
            hinting: Default::default(),
            typeface: Default::default(),
            size: Default::default(),
            scale_x: Default::default(),
            skew_x: Default::default(),
        }
    }

    #[inline]
    pub fn with_family<T: ToString>(family: T) -> Self {
        let mut font = Self::default();
        let mut typeface = FontTypeface::default();
        typeface.set_family(family);
        font.set_typeface(typeface);
        font
    }

    #[inline]
    pub fn set_force_auto_hinting(&mut self, force_auto_hinting: bool) {
        self.force_auto_hinting = force_auto_hinting
    }
    #[inline]
    pub fn is_force_auto_hinting(&self) -> bool {
        self.force_auto_hinting
    }

    #[inline]
    pub fn set_embedded_bitmaps(&mut self, embedded_bitmaps: bool) {
        self.embedded_bitmaps = embedded_bitmaps
    }
    #[inline]
    pub fn is_embedded_bitmaps(&self) -> bool {
        self.embedded_bitmaps
    }

    #[inline]
    pub fn set_subpixel(&mut self, subpixel: bool) {
        self.subpixel = subpixel
    }
    #[inline]
    pub fn is_subpixel(&self) -> bool {
        self.subpixel
    }

    #[inline]
    pub fn set_linear_metrics(&mut self, linear_metrics: bool) {
        self.linear_metrics = linear_metrics
    }
    #[inline]
    pub fn is_linear_metrics(&self) -> bool {
        self.linear_metrics
    }

    #[inline]
    pub fn set_embolden(&mut self, embolden: bool) {
        self.embolden = embolden
    }
    #[inline]
    pub fn is_embolden(&self) -> bool {
        self.embolden
    }

    #[inline]
    pub fn set_baseline_snap(&mut self, baseline_snap: bool) {
        self.baseline_snap = baseline_snap
    }
    #[inline]
    pub fn is_baseline_snap(&self) -> bool {
        self.baseline_snap
    }

    #[inline]
    pub fn set_edging(&mut self, edging: FontEdging) {
        self.edging = edging
    }
    #[inline]
    pub fn edging(&self) -> FontEdging {
        self.edging
    }

    #[inline]
    pub fn set_hinting(&mut self, hinting: FontHinting) {
        self.hinting = hinting
    }
    #[inline]
    pub fn hinting(&self) -> FontHinting {
        self.hinting
    }

    #[inline]
    pub fn set_typeface(&mut self, typeface: FontTypeface) {
        self.typeface = Some(typeface)
    }
    #[inline]
    pub fn typeface(&self) -> Option<FontTypeface> {
        self.typeface
    }

    #[inline]
    pub fn set_size(&mut self, size: f32) {
        self.size = size
    }
    #[inline]
    pub fn size(&self) -> f32 {
        self.size
    }

    #[inline]
    pub fn set_scale_x(&mut self, scale_x: f32) {
        self.scale_x = scale_x
    }
    #[inline]
    pub fn scale_x(&self) -> f32 {
        self.scale_x
    }

    #[inline]
    pub fn set_skew_x(&mut self, skew_x: f32) {
        self.skew_x = skew_x
    }
    #[inline]
    pub fn skew_x(&self) -> f32 {
        self.skew_x
    }

    #[inline]
    pub fn to_skia_font(&self) -> skia_safe::Font {
        let mut font = skia_safe::Font::default();
        font.set_force_auto_hinting(self.is_force_auto_hinting());
        font.set_embedded_bitmaps(self.is_embedded_bitmaps());
        font.set_subpixel(self.is_subpixel());
        font.set_linear_metrics(self.is_linear_metrics());
        font.set_embolden(self.is_embolden());
        font.set_baseline_snap(self.is_baseline_snap());
        font.set_edging(self.edging().into());
        font.set_hinting(self.hinting().into());
        if let Some(typeface) = self.typeface() {
            font.set_typeface(typeface);
        }
        font.set_size(self.size());
        font.set_scale_x(self.scale_x());
        font.set_skew_x(self.skew_x());
        font
    }
}

impl Into<skia_safe::Font> for Font {
    #[inline]
    fn into(self) -> skia_safe::Font {
        self.to_skia_font()
    }
}

impl Into<Font> for skia_safe::Font {
    #[inline]
    fn into(self) -> Font {
        let mut font = Font::new();
        font.set_force_auto_hinting(self.is_force_auto_hinting());
        font.set_embedded_bitmaps(self.is_embedded_bitmaps());
        font.set_subpixel(self.is_subpixel());
        font.set_linear_metrics(self.is_linear_metrics());
        font.set_embolden(self.is_embolden());
        font.set_baseline_snap(self.is_baseline_snap());
        font.set_edging(self.edging().into());
        font.set_hinting(self.hinting().into());
        if let Some(typeface) = self.typeface() {
            font.set_typeface(typeface.into());
        }
        font.set_size(self.size());
        font.set_scale_x(self.scale_x());
        font.set_skew_x(self.skew_x());
        font
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
/// [`FontTypeface`]
/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct FontTypeface {
    family: &'static str,
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
    pub fn family(&self) -> &str {
        &self.family
    }
    #[inline]
    pub fn set_family<T: ToString>(&mut self, family: T) {
        self.family = Box::leak(family.to_string().into_boxed_str());
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
        Typeface::new(self.family(), font_style)
            .expect(format!("Create typeface from `{}` failed.", self.family()).as_str())
    }
}

impl Into<FontTypeface> for Typeface {
    #[inline]
    fn into(self) -> FontTypeface {
        let font_style = self.font_style();
        FontTypeface::builder()
            .family(self.family_name())
            .weight(font_style.weight().into())
            .width(font_style.width().into())
            .italic(font_style.slant() == Slant::Italic)
            .build()
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
/// [`FontTypefaceBuilder`] Builder to create `FontTypeface`
/////////////////////////////////////////////////////////////////////////////////////////
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
        if let Some(family) = self.family {
            typeface.family = Box::leak(family.into_boxed_str());
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

/////////////////////////////////////////////////////////////////////////////////////////
/// [`FontWeight`]
/////////////////////////////////////////////////////////////////////////////////////////
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
    Custom(i32),
}

impl Into<Weight> for FontWeight {
    #[inline]
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
            Self::Custom(x) => Weight::from(x),
        }
    }
}

impl Into<FontWeight> for Weight {
    #[inline]
    fn into(self) -> FontWeight {
        match self {
            Weight::INVISIBLE => Invisible,
            Weight::THIN => Thin,
            Weight::EXTRA_LIGHT => ExtraLight,
            Weight::LIGHT => Light,
            Weight::NORMAL => FontWeight::Normal,
            Weight::MEDIUM => Medium,
            Weight::SEMI_BOLD => SemiBold,
            Weight::BOLD => Bold,
            Weight::EXTRA_BOLD => ExtraBold,
            Weight::BLACK => Black,
            Weight::EXTRA_BLACK => ExtraBlack,
            _ => FontWeight::Custom(*self.deref()),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
/// [`FontWidth`]
/////////////////////////////////////////////////////////////////////////////////////////
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
    Custom(i32),
}

impl Into<Width> for FontWidth {
    #[inline]
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
            Self::Custom(x) => Width::from(x),
        }
    }
}

impl Into<FontWidth> for Width {
    #[inline]
    fn into(self) -> FontWidth {
        match self {
            Width::ULTRA_CONDENSED => UltraCondensed,
            Width::EXTRA_CONDENSED => ExtraCondensed,
            Width::CONDENSED => Condensed,
            Width::SEMI_CONDENSED => SemiCondensed,
            Width::NORMAL => FontWidth::Normal,
            Width::SEMI_EXPANDED => SemiExpanded,
            Width::EXPANDED => Expanded,
            Width::EXTRA_EXPANDED => ExtraExpanded,
            Width::ULTRA_EXPANDED => UltraExpanded,
            _ => FontWidth::Custom(*self.deref()),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
/// [`FontEdging`]
/////////////////////////////////////////////////////////////////////////////////////////
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum FontEdging {
    #[default]
    Alias,
    AntiAlias,
    SubpixelAntiAlias,
}

impl Into<Edging> for FontEdging {
    #[inline]
    fn into(self) -> Edging {
        match self {
            Self::Alias => Edging::Alias,
            Self::AntiAlias => Edging::AntiAlias,
            Self::SubpixelAntiAlias => Edging::SubpixelAntiAlias,
        }
    }
}

impl Into<FontEdging> for Edging {
    #[inline]
    fn into(self) -> FontEdging {
        match self {
            Edging::Alias => FontEdging::Alias,
            Edging::AntiAlias => FontEdging::AntiAlias,
            Edging::SubpixelAntiAlias => FontEdging::SubpixelAntiAlias,
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
/// [`FontHinting`]
/////////////////////////////////////////////////////////////////////////////////////////
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum FontHinting {
    #[default]
    None,
    Slight,
    Normal,
    Full,
}

impl Into<skia_safe::FontHinting> for FontHinting {
    #[inline]
    fn into(self) -> skia_safe::FontHinting {
        match self {
            Self::None => skia_safe::FontHinting::None,
            Self::Slight => skia_safe::FontHinting::Slight,
            Self::Normal => skia_safe::FontHinting::Normal,
            Self::Full => skia_safe::FontHinting::Full,
        }
    }
}

impl Into<FontHinting> for skia_safe::FontHinting {
    #[inline]
    fn into(self) -> FontHinting {
        match self {
            skia_safe::FontHinting::None => FontHinting::None,
            skia_safe::FontHinting::Slight => FontHinting::Slight,
            skia_safe::FontHinting::Normal => FontHinting::Normal,
            skia_safe::FontHinting::Full => FontHinting::Full,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Font, FontTypeface};

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

    #[test]
    fn test_font() {
        let _font = Font::default();
    }
}
