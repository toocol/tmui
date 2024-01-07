use crate::skia_safe;
use crate::winit;

pub type SkiaFont = skia_safe::Font;
pub type SkiaTypeface = skia_safe::Typeface;
pub type SkiaFontWeight = skia_safe::font_style::Weight;
pub type SkiaFontWidth = skia_safe::font_style::Width;
pub type SkiaFontEdging = skia_safe::font::Edging;
pub type SkiaFontHiting = skia_safe::FontHinting;
pub type SkiaPoint = skia_safe::Point;
pub type SkiaRect = skia_safe::Rect;
pub type SkiaRRect = skia_safe::RRect;
pub type SkiaColor = skia_safe::Color;
pub type SkiaBlendMode = skia_safe::BlendMode;

pub type WinitWindow = winit::window::Window;
pub type WinitWindowBuilder = winit::window::WindowBuilder;
pub type WinitIcon = winit::window::Icon;