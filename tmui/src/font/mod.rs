pub mod mgr;

use lazy_static::__Deref;
use log::warn;
use tlib::{
    count_exprs,
    skia_safe::{
        self,
        font_style::Slant,
        textlayout::{
            FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextStyle,
            TypefaceFontProvider,
        },
    },
    typedef::{
        SkiaFont, SkiaFontEdging, SkiaFontHiting, SkiaFontStyle, SkiaFontWeight, SkiaFontWidth,
        SkiaTypeface,
    },
};
use FontWeight::*;
use FontWidth::*;

use self::mgr::FontManager;

macro_rules! default_font_families {
    ($($font:expr),*) => {
        const DEFAULT_FONT_FAMILY: [&'static str; count_exprs!($($font),*)] = [$($font),*];
    };
}

#[cfg(windows_platform)]
default_font_families!("Arial", "Microsoft YaHei");

#[cfg(apple)]
default_font_families!("Helvetica", "PingFang SC");

#[cfg(free_unix)]
default_font_families!("Liberation Sans", "Sim Sun");

/////////////////////////////////////////////////////////////////////////////////////////
/// [`Font`]
/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Clone)]
pub struct Font {
    force_auto_hinting: bool,
    embedded_bitmaps: bool,
    subpixel: bool,
    linear_metrics: bool,
    embolden: bool,
    baseline_snap: bool,
    edging: FontEdging,
    hinting: FontHinting,
    typefaces: Vec<FontTypeface>,
    size: f32,
    scale_x: f32,
    skew_x: f32,

    // Font styles:
    weight: FontWeight,
    width: FontWidth,
    italic: bool,
}

impl Default for Font {
    #[inline]
    fn default() -> Self {
        let mut font: Font = skia_safe::Font::default().into();

        let mut typefaces = vec![];
        for family in DEFAULT_FONT_FAMILY {
            let typeface = FontTypeface::new(family);
            typefaces.push(typeface);
        }
        font.set_typefaces(typefaces);

        font
    }
}

impl Font {
    #[inline]
    fn empty() -> Self {
        Self {
            force_auto_hinting: Default::default(),
            embedded_bitmaps: Default::default(),
            subpixel: Default::default(),
            linear_metrics: Default::default(),
            embolden: Default::default(),
            baseline_snap: Default::default(),
            edging: Default::default(),
            hinting: Default::default(),
            typefaces: Default::default(),
            size: Default::default(),
            scale_x: Default::default(),
            skew_x: Default::default(),
            weight: Default::default(),
            width: Default::default(),
            italic: Default::default(),
        }
    }

    #[inline]
    pub fn with_families<T: ToString>(families: &[T]) -> Self {
        let mut font = Self::default();

        let mut typefaces = vec![];
        for family in families {
            let mut typeface = FontTypeface::default();
            typeface.set_family(family.to_string());
            typefaces.push(typeface);
        }
        font.set_typefaces(typefaces);

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
    pub fn set_typefaces(&mut self, typefaces: Vec<FontTypeface>) {
        self.typefaces = typefaces
    }
    #[inline]
    pub fn typefaces(&self) -> &[FontTypeface] {
        &self.typefaces
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

    #[inline]
    pub fn to_skia_fonts(&self) -> Vec<SkiaFont> {
        let mut fonts = vec![];
        for tf in self.typefaces() {
            if let Some(typeface) = tf.to_skia_typeface(self) {
                let mut font = SkiaFont::default();
                font.set_force_auto_hinting(self.is_force_auto_hinting());
                font.set_embedded_bitmaps(self.is_embedded_bitmaps());
                font.set_subpixel(self.is_subpixel());
                font.set_linear_metrics(self.is_linear_metrics());
                font.set_embolden(self.is_embolden());
                font.set_baseline_snap(self.is_baseline_snap());
                font.set_edging(self.edging().into());
                font.set_hinting(self.hinting().into());
                font.set_typeface(typeface);
                font.set_size(self.size());
                font.set_scale_x(self.scale_x());
                font.set_skew_x(self.skew_x());

                fonts.push(font);
            }
        }
        fonts
    }
}

impl From<SkiaFont> for Font {
    #[inline]
    fn from(val: SkiaFont) -> Self {
        let mut font = Font::empty();
        font.set_force_auto_hinting(val.is_force_auto_hinting());
        font.set_embedded_bitmaps(val.is_embedded_bitmaps());
        font.set_subpixel(val.is_subpixel());
        font.set_linear_metrics(val.is_linear_metrics());
        font.set_embolden(val.is_embolden());
        font.set_baseline_snap(val.is_baseline_snap());
        font.set_edging(val.edging().into());
        font.set_hinting(val.hinting().into());
        {
            let typeface = val.typeface();
            let fs = typeface.font_style();
            font.set_font_weight(fs.weight().into());
            font.set_font_width(fs.width().into());
            font.set_italic(fs.slant() == Slant::Italic);
            font.typefaces = vec![FontTypeface::new(&typeface.family_name())];
        }
        font.set_size(val.size());
        font.set_scale_x(val.scale_x());
        font.set_skew_x(val.skew_x());
        font
    }
}

impl Font {
    #[inline]
    pub(crate) fn get_skia_font_style(&self) -> SkiaFontStyle {
        SkiaFontStyle::new(
            self.weight.into(),
            self.width.into(),
            if self.italic() {
                Slant::Italic
            } else {
                Slant::Upright
            },
        )
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
/// [`FontTypeface`]
/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct FontTypeface {
    family: &'static str,
}

impl FontTypeface {
    /// Get the [`FontTypefaceBuilder`]
    #[inline]
    pub fn new(family: &str) -> Self {
        Self {
            family: Box::leak(family.to_string().into_boxed_str()),
        }
    }

    #[inline]
    pub fn family(&self) -> &str {
        self.family
    }
    #[inline]
    pub fn set_family(&mut self, family: String) {
        self.family = Box::leak(family.into_boxed_str());
    }

    #[inline]
    pub fn to_skia_typeface(&self, font: &Font) -> Option<SkiaTypeface> {
        let typeface = if let Some(typeface) = FontManager::get(self.family) {
            Some(typeface)
        } else {
            FontManager::make_typeface(self.family, font.get_skia_font_style())
        };

        if typeface.is_none() {
            warn!("Make typeface from family `{}` failed.", self.family);
        }

        typeface
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

impl From<FontWeight> for SkiaFontWeight {
    #[inline]
    fn from(val: FontWeight) -> Self {
        match val {
            Invisible => SkiaFontWeight::INVISIBLE,
            Thin => SkiaFontWeight::THIN,
            ExtraLight => SkiaFontWeight::EXTRA_LIGHT,
            Light => SkiaFontWeight::LIGHT,
            FontWeight::Normal => SkiaFontWeight::NORMAL,
            Medium => SkiaFontWeight::MEDIUM,
            SemiBold => SkiaFontWeight::SEMI_BOLD,
            Bold => SkiaFontWeight::BOLD,
            ExtraBold => SkiaFontWeight::EXTRA_BOLD,
            Black => SkiaFontWeight::BLACK,
            ExtraBlack => SkiaFontWeight::EXTRA_BLACK,
            FontWeight::Custom(x) => SkiaFontWeight::from(x),
        }
    }
}

impl From<SkiaFontWeight> for FontWeight {
    #[inline]
    fn from(val: SkiaFontWeight) -> Self {
        match val {
            SkiaFontWeight::INVISIBLE => Invisible,
            SkiaFontWeight::THIN => Thin,
            SkiaFontWeight::EXTRA_LIGHT => ExtraLight,
            SkiaFontWeight::LIGHT => Light,
            SkiaFontWeight::NORMAL => FontWeight::Normal,
            SkiaFontWeight::MEDIUM => Medium,
            SkiaFontWeight::SEMI_BOLD => SemiBold,
            SkiaFontWeight::BOLD => Bold,
            SkiaFontWeight::EXTRA_BOLD => ExtraBold,
            SkiaFontWeight::BLACK => Black,
            SkiaFontWeight::EXTRA_BLACK => ExtraBlack,
            _ => FontWeight::Custom(*val.deref()),
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

impl From<FontWidth> for SkiaFontWidth {
    #[inline]
    fn from(val: FontWidth) -> Self {
        match val {
            UltraCondensed => SkiaFontWidth::ULTRA_CONDENSED,
            ExtraCondensed => SkiaFontWidth::EXTRA_CONDENSED,
            Condensed => SkiaFontWidth::CONDENSED,
            SemiCondensed => SkiaFontWidth::SEMI_CONDENSED,
            FontWidth::Normal => SkiaFontWidth::NORMAL,
            SemiExpanded => SkiaFontWidth::SEMI_EXPANDED,
            Expanded => SkiaFontWidth::EXPANDED,
            ExtraExpanded => SkiaFontWidth::EXTRA_EXPANDED,
            UltraExpanded => SkiaFontWidth::ULTRA_EXPANDED,
            FontWidth::Custom(x) => SkiaFontWidth::from(x),
        }
    }
}

impl From<SkiaFontWidth> for FontWidth {
    #[inline]
    fn from(val: SkiaFontWidth) -> Self {
        match val {
            SkiaFontWidth::ULTRA_CONDENSED => UltraCondensed,
            SkiaFontWidth::EXTRA_CONDENSED => ExtraCondensed,
            SkiaFontWidth::CONDENSED => Condensed,
            SkiaFontWidth::SEMI_CONDENSED => SemiCondensed,
            SkiaFontWidth::NORMAL => FontWidth::Normal,
            SkiaFontWidth::SEMI_EXPANDED => SemiExpanded,
            SkiaFontWidth::EXPANDED => Expanded,
            SkiaFontWidth::EXTRA_EXPANDED => ExtraExpanded,
            SkiaFontWidth::ULTRA_EXPANDED => UltraExpanded,
            _ => FontWidth::Custom(*val.deref()),
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

impl From<FontEdging> for SkiaFontEdging {
    #[inline]
    fn from(val: FontEdging) -> Self {
        match val {
            FontEdging::Alias => SkiaFontEdging::Alias,
            FontEdging::AntiAlias => SkiaFontEdging::AntiAlias,
            FontEdging::SubpixelAntiAlias => SkiaFontEdging::SubpixelAntiAlias,
        }
    }
}

impl From<SkiaFontEdging> for FontEdging {
    #[inline]
    fn from(val: SkiaFontEdging) -> Self {
        match val {
            SkiaFontEdging::Alias => FontEdging::Alias,
            SkiaFontEdging::AntiAlias => FontEdging::AntiAlias,
            SkiaFontEdging::SubpixelAntiAlias => FontEdging::SubpixelAntiAlias,
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

impl From<FontHinting> for SkiaFontHiting {
    #[inline]
    fn from(val: FontHinting) -> Self {
        match val {
            FontHinting::None => SkiaFontHiting::None,
            FontHinting::Slight => SkiaFontHiting::Slight,
            FontHinting::Normal => SkiaFontHiting::Normal,
            FontHinting::Full => SkiaFontHiting::Full,
        }
    }
}

impl From<SkiaFontHiting> for FontHinting {
    #[inline]
    fn from(val: SkiaFontHiting) -> Self {
        match val {
            SkiaFontHiting::None => FontHinting::None,
            SkiaFontHiting::Slight => FontHinting::Slight,
            SkiaFontHiting::Normal => FontHinting::Normal,
            SkiaFontHiting::Full => FontHinting::Full,
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
/// [`FontCalculation`]
/////////////////////////////////////////////////////////////////////////////////////////
pub trait FontCalculation {
    const REPCHAR: &'static str = concat!(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "abcdefgjijklmnopqrstuvwxyz",
        "0123456789./+@*()#$!",
    );

    const REPCHAR_UNICODE: &'static str =
        concat!("一二三四五六七八九十中文符号零百千万亿：，。？！、《》‘’“”",);

    fn calc_font_dimension(&self) -> (f32, f32);

    fn calc_font_dimension_unicode(&self) -> (f32, f32);

    fn calc_text_dimension(&self, text: &str, letter_spacing: f32) -> (f32, f32);
}
impl FontCalculation for Font {
    #[inline]
    fn calc_font_dimension(&self) -> (f32, f32) {
        let (w, h) = calc_text_dimension(self, Self::REPCHAR, 0.);
        (w / Self::REPCHAR.chars().count() as f32, h)
    }

    #[inline]
    fn calc_font_dimension_unicode(&self) -> (f32, f32) {
        let (w, h) = calc_text_dimension(self, Self::REPCHAR_UNICODE, 0.);
        (w / Self::REPCHAR_UNICODE.chars().count() as f32, h)
    }

    #[inline]
    fn calc_text_dimension(&self, text: &str, letter_spacing: f32) -> (f32, f32) {
        calc_text_dimension(self, text, letter_spacing)
    }
}
#[inline]
fn calc_text_dimension(font: &Font, text: &str, letter_spacing: f32) -> (f32, f32) {
    let typefaces = font.typefaces();

    let mut typeface_provider = TypefaceFontProvider::new();
    let mut families = vec![];
    for typeface in typefaces {
        let typeface = typeface.to_skia_typeface(font);

        if let Some(typeface) = typeface {
            let family = typeface.family_name();
            typeface_provider.register_typeface(typeface, Some(family.as_str()));
            families.push(family);
        }
    }

    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));

    // define text style
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_font_style(font.get_skia_font_style());
    text_style.set_font_size(font.size());
    text_style.set_font_families(&families);
    text_style.set_letter_spacing(letter_spacing);
    style.set_text_style(&text_style);
    style.set_max_lines(None);
    style.set_ellipsis("");

    // layout the paragraph
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(f32::MAX);

    (paragraph.max_intrinsic_width(), paragraph.height())
}

pub trait SkiaParagraphExt {
    fn single_line_baseline(&self) -> f32;
}
impl SkiaParagraphExt for Paragraph {
    fn single_line_baseline(&self) -> f32 {
        let mut baseline = 0.0f32;

        let line_metrics = self.get_line_metrics();
        if let Some(metrics) = line_metrics.first() {
            for (_, mcs) in metrics.get_style_metrics(0..usize::MAX) {
                let fm = mcs.text_style.font_metrics();
                baseline = baseline.max(-fm.ascent);
            }
        }

        baseline
    }
}

#[cfg(test)]
mod tests {
    use super::{Font, FontCalculation};
    use tlib::typedef::SkiaFont;
    use widestring::U16String;

    const REPCHAR: &'static str = concat!(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "abcdefgjijklmnopqrstuvwxyz",
        "0123456789./+@"
    );

    #[test]
    fn test_font_calc() {
        let font  = Font::default();
        let fd = font.calc_font_dimension();
        let td = font.calc_text_dimension(REPCHAR, 0.);
        assert_eq!(fd.1, td.1);

        let us = U16String::from_str(REPCHAR);
        let slice = us.as_slice();
        let mut widths = vec![0.; slice.len()];
        font.to_skia_fonts()[0].get_widths(slice, &mut widths);
    }

    #[test]
    fn test_font() {
        let font  = Font::default();
        let skia_font: &SkiaFont = &font.to_skia_fonts()[0];

        let measure = skia_font.measure_str(REPCHAR, None);

        let metrics = skia_font.metrics();
        let wstring = U16String::from_str(REPCHAR);
        let wchar_t_repchar = wstring.as_slice();
        let mut widths = vec![0f32; wchar_t_repchar.len()];
        skia_font.get_widths(wchar_t_repchar, &mut widths);
        let sum_width: f32 = widths.iter().sum();
        let font_width = sum_width as f64 / wchar_t_repchar.len() as f64;

        println!(
            "metrics => height: {}, width: {}",
            metrics.1.cap_height, font_width
        );
        println!(
            "measure => height: {}, width: {}",
            measure.1.height(),
            measure.1.width() / REPCHAR.len() as f32
        );

        let _skia_typeface = skia_font.typeface();

        assert_eq!(
            font.is_force_auto_hinting(),
            skia_font.is_force_auto_hinting()
        );
        assert_eq!(font.is_embedded_bitmaps(), skia_font.is_embedded_bitmaps());
        assert_eq!(font.is_subpixel(), skia_font.is_subpixel());
        assert_eq!(font.is_linear_metrics(), skia_font.is_linear_metrics());
        assert_eq!(font.is_embolden(), skia_font.is_embolden());
        assert_eq!(font.is_baseline_snap(), skia_font.is_baseline_snap());

        let skia_edging = skia_font.edging().into();
        assert_eq!(font.edging(), skia_edging);

        let skia_hiting = skia_font.hinting().into();
        assert_eq!(font.hinting, skia_hiting);

        assert_eq!(font.size(), skia_font.size());
        assert_eq!(font.scale_x(), skia_font.scale_x());
        assert_eq!(font.skew_x(), skia_font.skew_x());
    }
}
