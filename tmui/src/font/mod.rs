pub mod mgr;

use lazy_static::__Deref;
use tlib::skia_safe::{
    self,
    font_style::Slant,
    textlayout::{
        Paragraph, FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle, TypefaceFontProvider,
    },
    FontStyle,
};
use FontWeight::*;
use FontWidth::*;

use tlib::count_exprs;
use tlib::typedef::{
    SkiaFont, SkiaFontEdging, SkiaFontHiting, SkiaFontWeight, SkiaFontWidth, SkiaTypeface,
};

use self::mgr::FontManager;

macro_rules! default_font_families {
    ($($font:expr),*) => {
        const DEFAULT_FONT_FAMILY: [&'static str; count_exprs!($($font),*)] = [$($font),*];
    };
}

default_font_families!(
    "Arial",
    "Noto Sans SC",
    "Microsoft YaHei"
);

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
}

impl Default for Font {
    #[inline]
    fn default() -> Self {
        let mut font: Font = skia_safe::Font::default().into();

        let mut typefaces = vec![];
        for family in DEFAULT_FONT_FAMILY {
            let typeface = FontTypeface::builder().family(family).build();
            typefaces.push(typeface);
        }
        font.set_typefaces(typefaces);

        font
    }
}

impl Font {
    #[inline]
    fn new() -> Self {
        let mut typefaces = vec![];
        for family in DEFAULT_FONT_FAMILY {
            let typeface = FontTypeface::builder().family(family).build();
            typefaces.push(typeface);
        }
        Self {
            force_auto_hinting: Default::default(),
            embedded_bitmaps: Default::default(),
            subpixel: Default::default(),
            linear_metrics: Default::default(),
            embolden: Default::default(),
            baseline_snap: Default::default(),
            edging: Default::default(),
            hinting: Default::default(),
            typefaces,
            size: Default::default(),
            scale_x: Default::default(),
            skew_x: Default::default(),
        }
    }

    #[inline]
    pub fn with_family<T: ToString>(families: Vec<T>) -> Self {
        let mut font = Self::default();

        let mut typefaces = vec![];
        for family in families {
            let mut typeface = FontTypeface::default();
            typeface.set_family(family);
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
    pub fn to_skia_fonts(&self) -> Vec<SkiaFont> {
        let mut fonts = vec![];
        for tf in self.typefaces() {
            let mut font = SkiaFont::default();
            font.set_force_auto_hinting(self.is_force_auto_hinting());
            font.set_embedded_bitmaps(self.is_embedded_bitmaps());
            font.set_subpixel(self.is_subpixel());
            font.set_linear_metrics(self.is_linear_metrics());
            font.set_embolden(self.is_embolden());
            font.set_baseline_snap(self.is_baseline_snap());
            font.set_edging(self.edging().into());
            font.set_hinting(self.hinting().into());
            font.set_typeface(tf);
            font.set_size(self.size());
            font.set_scale_x(self.scale_x());
            font.set_skew_x(self.skew_x());

            fonts.push(font);
        }
        fonts
    }

    //////////////////////////////////// Typeface related.
    #[inline]
    pub fn set_font_weight(&mut self, weight: FontWeight) {
        self.typefaces
            .iter_mut()
            .for_each(|tf| tf.set_font_weight(weight))
    }

    #[inline]
    pub fn set_font_width(&mut self, width: FontWidth) {
        self.typefaces
            .iter_mut()
            .for_each(|tf| tf.set_font_width(width))
    }

    #[inline]
    pub fn set_italic(&mut self, italic: bool) {
        self.typefaces
            .iter_mut()
            .for_each(|tf| tf.set_italic(italic))
    }
}

impl Into<Font> for SkiaFont {
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
            font.set_typefaces(vec![typeface.into()]);
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

    #[inline]
    pub fn to_skia_typeface(&self) -> SkiaTypeface {
        let font_style = FontStyle::new(
            self.weight.into(),
            self.width.into(),
            if self.italic() {
                Slant::Italic
            } else {
                Slant::Upright
            },
        );

        if let Some(typeface) = FontManager::get(self.family) {
            typeface
        } else {
            SkiaTypeface::new(self.family(), font_style)
                .expect(format!("Create typeface from `{}` failed.", self.family()).as_str())
        }
    }
}

impl Into<SkiaTypeface> for FontTypeface {
    #[inline]
    fn into(self) -> SkiaTypeface {
        self.to_skia_typeface()
    }
}

impl Into<SkiaTypeface> for &FontTypeface {
    fn into(self) -> SkiaTypeface {
        self.to_skia_typeface()
    }
}

impl Into<FontTypeface> for SkiaTypeface {
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

impl Into<SkiaFontWeight> for FontWeight {
    #[inline]
    fn into(self) -> SkiaFontWeight {
        match self {
            Invisible => SkiaFontWeight::INVISIBLE,
            Thin => SkiaFontWeight::THIN,
            ExtraLight => SkiaFontWeight::EXTRA_LIGHT,
            Light => SkiaFontWeight::LIGHT,
            Self::Normal => SkiaFontWeight::NORMAL,
            Medium => SkiaFontWeight::MEDIUM,
            SemiBold => SkiaFontWeight::SEMI_BOLD,
            Bold => SkiaFontWeight::BOLD,
            ExtraBold => SkiaFontWeight::EXTRA_BOLD,
            Black => SkiaFontWeight::BLACK,
            ExtraBlack => SkiaFontWeight::EXTRA_BLACK,
            Self::Custom(x) => SkiaFontWeight::from(x),
        }
    }
}

impl Into<FontWeight> for SkiaFontWeight {
    #[inline]
    fn into(self) -> FontWeight {
        match self {
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

impl Into<SkiaFontWidth> for FontWidth {
    #[inline]
    fn into(self) -> SkiaFontWidth {
        match self {
            UltraCondensed => SkiaFontWidth::ULTRA_CONDENSED,
            ExtraCondensed => SkiaFontWidth::EXTRA_CONDENSED,
            Condensed => SkiaFontWidth::CONDENSED,
            SemiCondensed => SkiaFontWidth::SEMI_CONDENSED,
            Self::Normal => SkiaFontWidth::NORMAL,
            SemiExpanded => SkiaFontWidth::SEMI_EXPANDED,
            Expanded => SkiaFontWidth::EXPANDED,
            ExtraExpanded => SkiaFontWidth::EXTRA_EXPANDED,
            UltraExpanded => SkiaFontWidth::ULTRA_EXPANDED,
            Self::Custom(x) => SkiaFontWidth::from(x),
        }
    }
}

impl Into<FontWidth> for SkiaFontWidth {
    #[inline]
    fn into(self) -> FontWidth {
        match self {
            SkiaFontWidth::ULTRA_CONDENSED => UltraCondensed,
            SkiaFontWidth::EXTRA_CONDENSED => ExtraCondensed,
            SkiaFontWidth::CONDENSED => Condensed,
            SkiaFontWidth::SEMI_CONDENSED => SemiCondensed,
            SkiaFontWidth::NORMAL => FontWidth::Normal,
            SkiaFontWidth::SEMI_EXPANDED => SemiExpanded,
            SkiaFontWidth::EXPANDED => Expanded,
            SkiaFontWidth::EXTRA_EXPANDED => ExtraExpanded,
            SkiaFontWidth::ULTRA_EXPANDED => UltraExpanded,
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

impl Into<SkiaFontEdging> for FontEdging {
    #[inline]
    fn into(self) -> SkiaFontEdging {
        match self {
            Self::Alias => SkiaFontEdging::Alias,
            Self::AntiAlias => SkiaFontEdging::AntiAlias,
            Self::SubpixelAntiAlias => SkiaFontEdging::SubpixelAntiAlias,
        }
    }
}

impl Into<FontEdging> for SkiaFontEdging {
    #[inline]
    fn into(self) -> FontEdging {
        match self {
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

impl Into<SkiaFontHiting> for FontHinting {
    #[inline]
    fn into(self) -> SkiaFontHiting {
        match self {
            Self::None => SkiaFontHiting::None,
            Self::Slight => SkiaFontHiting::Slight,
            Self::Normal => SkiaFontHiting::Normal,
            Self::Full => SkiaFontHiting::Full,
        }
    }
}

impl Into<FontHinting> for SkiaFontHiting {
    #[inline]
    fn into(self) -> FontHinting {
        match self {
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
        "中文符号：，。？！、《》‘’“”"
    );

    fn calc_font_dimension(&self) -> (f32, f32);

    fn calc_text_dimension(&self, text: &str, letter_spacing: f32) -> (f32, f32);
}
impl FontCalculation for Font {
    fn calc_font_dimension(&self) -> (f32, f32) {
        let (w, h) = calc_text_dimension(self, Self::REPCHAR, 0.);
        (w / Self::REPCHAR.chars().count() as f32, h)
    }

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
        let typeface = typeface.to_skia_typeface();
        let family = typeface.family_name();
        typeface_provider.register_typeface(typeface, Some(family.clone()));
        families.push(family);
    }

    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));

    // define text style
    let mut style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_font_size(font.size());
    text_style.set_font_families(&families);
    text_style.set_letter_spacing(letter_spacing);
    style.set_text_style(&text_style);

    // layout the paragraph
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(f32::MAX);

    (paragraph.max_intrinsic_width(), paragraph.height())
}

pub trait SingleLineBaselineAware {
    fn single_line_baseline(&self) -> f32;
}
impl SingleLineBaselineAware for Paragraph {
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
    use super::{Font, FontCalculation, FontTypeface};
    use tlib::global::fuzzy_compare_32;
    use tlib::typedef::{SkiaFont, SkiaTypeface};
    use widestring::U16String;

    const REPCHAR: &'static str = concat!(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "abcdefgjijklmnopqrstuvwxyz",
        "0123456789./+@"
    );

    #[test]
    fn test_font_calc() {
        let font = Font::with_family(vec!["Courier New"]);
        // let font: SkiaFont = font.into();
        let fd = font.calc_font_dimension();
        let td = font.calc_text_dimension(REPCHAR, 0.);
        assert_eq!(fd.1, td.1);

        let us = U16String::from_str(REPCHAR);
        let slice = us.as_slice();
        let mut widths = vec![0.; slice.len()];
        font.to_skia_fonts()[0].get_widths(slice, &mut widths);
        let w = widths.iter().sum::<f32>() / widths.len() as f32;
        assert!(fuzzy_compare_32(td.0 / REPCHAR.len() as f32, w));
    }

    #[test]
    fn test_font_typeface() {
        let typeface = FontTypeface::builder()
            .family("Consolas")
            .bold(true)
            .italic(true)
            .build();
        assert!(typeface.bold());
        assert!(typeface.italic());
        let typeface: SkiaTypeface = typeface.into();
        assert!(typeface.is_bold());
        assert!(typeface.is_italic());
    }

    #[test]
    fn test_font() {
        let font = Font::with_family(vec!["Courier New"]);
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

        let _skia_typeface = skia_font.typeface().unwrap();

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
