use crate::{
    typedef::{SkiaISize, SkiaSize},
    types::StaticType,
    values::{FromBytes, FromValue, ToBytes, ToValue},
    Type, Value,
};
use std::ops::{Add, Sub};

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// Size
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Size {
    width: i32,
    height: i32,
}

impl Size {
    #[inline]
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.height
    }

    #[inline]
    pub fn set_width(&mut self, width: i32) {
        self.width = width
    }

    #[inline]
    pub fn set_height(&mut self, height: i32) {
        self.height = height
    }

    #[inline]
    pub fn width_mut(&mut self) -> &mut i32 {
        &mut self.width
    }

    #[inline]
    pub fn height_mut(&mut self) -> &mut i32 {
        &mut self.height
    }

    #[inline]
    pub fn add_width(&mut self, width: i32) {
        self.width += width
    }

    #[inline]
    pub fn add_height(&mut self, height: i32) {
        self.height += height
    }

    #[inline]
    pub fn max(&mut self, size: Size) {
        self.width = self.width.max(size.width);
        self.height = self.height.max(size.height);
    }

    #[inline]
    pub fn min(&mut self, size: Size) {
        self.width = self.width.min(size.width);
        self.height = self.height.min(size.height);
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.width != 0 && self.height != 0
    }
}

impl From<(i32, i32)> for Size {
    #[inline]
    fn from((width, height): (i32, i32)) -> Self {
        Self { width, height }
    }
}

impl From<Size> for (i32, i32) {
    #[inline]
    fn from(value: Size) -> Self {
        (value.width, value.height)
    }
}

impl From<Size> for FSize {
    #[inline]
    fn from(value: Size) -> Self {
        FSize {
            width: value.width as f32,
            height: value.height as f32,
        }
    }
}

impl From<Size> for SkiaSize {
    #[inline]
    fn from(value: Size) -> Self {
        SkiaSize::new(value.width as f32, value.height as f32)
    }
}

impl From<Size> for SkiaISize {
    #[inline]
    fn from(value: Size) -> Self {
        SkiaISize::new(value.width, value.height)
    }
}

impl Add for Size {
    type Output = Size;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl Sub for Size {
    type Output = Size;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}

impl StaticType for Size {
    #[inline]
    fn static_type() -> crate::Type {
        crate::Type::from_name("Size")
    }

    #[inline]
    fn bytes_len() -> usize {
        i32::bytes_len() + i32::bytes_len()
    }
}

impl ToBytes for Size {
    #[inline]
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.width.to_bytes());
        bytes.append(&mut self.height.to_bytes());
        bytes
    }
}

impl FromBytes for Size {
    #[inline]
    fn from_bytes(data: &[u8], _len: usize) -> Self {
        let width = i32::from_bytes(&data[0..4], 4);
        let height = i32::from_bytes(&data[4..8], 4);
        Self { width, height }
    }
}

impl ToValue for Size {
    #[inline]
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    #[inline]
    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl FromValue for Size {
    #[inline]
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// FSize
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct FSize {
    width: f32,
    height: f32,
}

impl FSize {
    #[inline]
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    #[inline]
    pub fn width(&self) -> f32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.height
    }

    #[inline]
    pub fn set_width(&mut self, width: f32) {
        self.width = width
    }

    #[inline]
    pub fn set_height(&mut self, height: f32) {
        self.height = height
    }

    #[inline]
    pub fn width_mut(&mut self) -> &mut f32 {
        &mut self.width
    }

    #[inline]
    pub fn height_mut(&mut self) -> &mut f32 {
        &mut self.height
    }

    #[inline]
    pub fn add_width(&mut self, width: f32) {
        self.width += width
    }

    #[inline]
    pub fn add_height(&mut self, height: f32) {
        self.height += height
    }

    #[inline]
    pub fn max(&mut self, size: FSize) {
        self.width = self.width.max(size.width);
        self.height = self.height.max(size.height);
    }

    #[inline]
    pub fn min(&mut self, size: FSize) {
        self.width = self.width.min(size.width);
        self.height = self.height.min(size.height);
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.width != 0. && self.height != 0.
    }

    #[inline]
    pub fn floor(mut self) -> Self {
        self.width = self.width.floor();
        self.height = self.height.floor();
        self
    }

    #[inline]
    pub fn ceil(mut self) -> Self {
        self.width = self.width.ceil();
        self.height = self.height.ceil();
        self
    }

    #[inline]
    pub fn round(mut self) -> Self {
        self.width = self.width.round();
        self.height = self.height.round();
        self
    }
}

impl From<(f32, f32)> for FSize {
    #[inline]
    fn from((width, height): (f32, f32)) -> Self {
        Self { width, height }
    }
}

impl From<FSize> for (f32, f32) {
    #[inline]
    fn from(value: FSize) -> Self {
        (value.width, value.height)
    }
}

impl From<FSize> for Size {
    #[inline]
    fn from(value: FSize) -> Self {
        Size {
            width: value.width as i32,
            height: value.height as i32,
        }
    }
}

impl From<FSize> for SkiaSize {
    #[inline]
    fn from(value: FSize) -> Self {
        SkiaSize::new(value.width, value.height)
    }
}

impl From<FSize> for SkiaISize {
    #[inline]
    fn from(value: FSize) -> Self {
        SkiaISize::new(value.width.round() as i32, value.height.round() as i32)
    }
}

impl Add for FSize {
    type Output = FSize;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl Sub for FSize {
    type Output = FSize;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}

impl StaticType for FSize {
    #[inline]
    fn static_type() -> crate::Type {
        crate::Type::from_name("FSize")
    }

    #[inline]
    fn bytes_len() -> usize {
        f32::bytes_len() + f32::bytes_len()
    }
}

impl ToBytes for FSize {
    #[inline]
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.width.to_bytes());
        bytes.append(&mut self.height.to_bytes());
        bytes
    }
}

impl FromBytes for FSize {
    #[inline]
    fn from_bytes(data: &[u8], _len: usize) -> Self {
        let width = f32::from_bytes(&data[0..4], 4);
        let height = f32::from_bytes(&data[4..8], 4);
        Self { width, height }
    }
}

impl ToValue for FSize {
    #[inline]
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    #[inline]
    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl FromValue for FSize {
    #[inline]
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// SizeHint
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct SizeHint {
    max_width: Option<i32>,
    max_height: Option<i32>,
    min_width: Option<i32>,
    min_height: Option<i32>,
}

impl SizeHint {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_max_width(mut self, max_width: i32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    #[inline]
    pub fn with_max_height(mut self, max_height: i32) -> Self {
        self.max_height = Some(max_height);
        self
    }

    #[inline]
    pub fn with_min_width(mut self, min_width: i32) -> Self {
        self.min_width = Some(min_width);
        self
    }

    #[inline]
    pub fn with_min_height(mut self, min_height: i32) -> Self {
        self.min_height = Some(min_height);
        self
    }

    #[inline]
    pub fn max_width(&self) -> Option<i32> {
        self.max_width
    }

    #[inline]
    pub fn max_height(&self) -> Option<i32> {
        self.max_height
    }

    #[inline]
    pub fn min_width(&self) -> Option<i32> {
        self.min_width
    }

    #[inline]
    pub fn min_height(&self) -> Option<i32> {
        self.min_height
    }

    #[inline]
    pub fn minimum(&self) -> (Option<i32>, Option<i32>) {
        (self.min_width, self.min_height)
    }

    #[inline]
    pub fn maximum(&self) -> (Option<i32>, Option<i32>) {
        (self.max_width, self.max_height)
    }

    #[inline]
    pub fn all(&self) -> (Option<i32>, Option<i32>, Option<i32>, Option<i32>) {
        (
            self.min_width,
            self.min_height,
            self.max_width,
            self.max_height,
        )
    }

    #[inline]
    pub fn all_width(&self) -> (Option<i32>, Option<i32>) {
        (self.min_width, self.max_width)
    }

    #[inline]
    pub fn all_height(&self) -> (Option<i32>, Option<i32>) {
        (self.min_height, self.max_height)
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// OptionSize
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OptionSize {
    width: Option<i32>,
    height: Option<i32>,
}

impl OptionSize {
    #[inline]
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width: Some(width),
            height: Some(height),
        }
    }

    #[inline]
    pub fn width_only(width: i32) -> Self {
        Self { width: Some(width), height: None }
    }

    #[inline]
    pub fn height_only(height: i32) -> Self {
        Self { width: None, height: Some(height) }
    }

    #[inline]
    pub fn none() -> Self {
        Self { width: None, height: None }
    }

    #[inline]
    pub fn width(&self) -> Option<i32> {
        self.width
    }

    #[inline]
    pub fn height(&self) -> Option<i32> {
        self.height
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_size_value() {
        let size = Size::new(125, 250);
        let val = size.to_value();
        assert_eq!(size, val.get::<Size>());

        let fsize = FSize::new(125., 250.);
        let val = fsize.to_value();
        assert_eq!(fsize, val.get::<FSize>());
    }
}
