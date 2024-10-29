use std::any::TypeId;

use nohash_hasher::IsEnabled;

use crate::skia_safe;
use crate::winit;

pub type SkiaFont = skia_safe::Font;
pub type SkiaTypeface = skia_safe::Typeface;
pub type SkiaFontWeight = skia_safe::font_style::Weight;
pub type SkiaFontWidth = skia_safe::font_style::Width;
pub type SkiaFontEdging = skia_safe::font::Edging;
pub type SkiaFontHiting = skia_safe::FontHinting;
pub type SkiaFontStyle = skia_safe::FontStyle;
pub type SkiaData = skia_safe::Data;
pub type SkiaPoint = skia_safe::Point;
pub type SkiaIPoint = skia_safe::IPoint;
pub type SkiaRect = skia_safe::Rect;
pub type SkiaIRect = skia_safe::IRect;
pub type SkiaRRect = skia_safe::RRect;
pub type SkiaRegion = skia_safe::Region;
pub type SkiaColor = skia_safe::Color;
pub type SkiaColor3f = skia_safe::Color3f;
pub type SkiaColor4f = skia_safe::Color4f;
pub type SkiaBlendMode = skia_safe::BlendMode;
pub type SkiaSvgDom = skia_safe::svg::Dom;
pub type SkiaImage = skia_safe::Image;
pub type SkiaSize = skia_safe::Size;
pub type SkiaISize = skia_safe::ISize;
pub type SkiaClipOp = skia_safe::ClipOp;
pub type SkiaPaintStyle = skia_safe::PaintStyle;

pub type WinitWindow = winit::window::Window;
pub type WinitWindowBuilder = winit::window::WindowBuilder;
pub type WinitIcon = winit::window::Icon;
pub type WinitKeyCode = winit::keyboard::KeyCode;
pub type WinitMouseButton = winit::event::MouseButton;
pub type WinitPosition = winit::dpi::Position;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WrappedWindowId(winit::window::WindowId);
impl From<winit::window::WindowId> for WrappedWindowId {
    #[inline]
    fn from(value: winit::window::WindowId) -> Self {
        Self(value)
    }
}
impl IsEnabled for WrappedWindowId {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WrappedTypeId(TypeId);
impl From<TypeId> for WrappedTypeId {
    #[inline]
    fn from(value: TypeId) -> Self {
        WrappedTypeId(value)
    }
}
impl IsEnabled for WrappedTypeId {}