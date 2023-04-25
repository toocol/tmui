use std::ops::{Add, Sub};
use tlib::{
    types::StaticType,
    values::{FromBytes, FromValue, ToBytes, ToValue},
    Type, Value,
};

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

    pub fn max(&mut self, size: Size) {
        self.width = self.width.max(size.width);
        self.height = self.height.max(size.height);
    }

    pub fn min(&mut self, size: Size) {
        self.width = self.width.min(size.width);
        self.height = self.height.min(size.height);
    }
}

impl From<(i32, i32)> for Size {
    fn from((width, height): (i32, i32)) -> Self {
        Self { width, height }
    }
}

impl Into<(i32, i32)> for Size {
    fn into(self) -> (i32, i32) {
        (self.width, self.height)
    }
}

impl Into<FSize> for Size {
    fn into(self) -> FSize {
        FSize {
            width: self.width as f32,
            height: self.height as f32,
        }
    }
}

impl Add for Size {
    type Output = Size;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl Sub for Size {
    type Output = Size;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}

impl StaticType for Size {
    fn static_type() -> tlib::Type {
        tlib::Type::from_name("Size")
    }

    fn bytes_len() -> usize {
        i32::bytes_len() + i32::bytes_len()
    }
}

impl ToBytes for Size {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.width.to_bytes());
        bytes.append(&mut self.height.to_bytes());
        bytes
    }
}

impl FromBytes for Size {
    fn from_bytes(data: &[u8], _len: usize) -> Self {
        let width = i32::from_bytes(&data[0..4], 4);
        let height = i32::from_bytes(&data[4..8], 4);
        Self { width, height }
    }
}

impl ToValue for Size {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl FromValue for Size {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// Size
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
}

impl From<(f32, f32)> for FSize {
    fn from((width, height): (f32, f32)) -> Self {
        Self { width, height }
    }
}

impl Into<(f32, f32)> for FSize {
    fn into(self) -> (f32, f32) {
        (self.width, self.height)
    }
}

impl Into<Size> for FSize {
    fn into(self) -> Size {
        Size {
            width: self.width as i32,
            height: self.height as i32,
        }
    }
}

impl Add for FSize {
    type Output = FSize;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl Sub for FSize {
    type Output = FSize;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}

impl StaticType for FSize {
    fn static_type() -> tlib::Type {
        tlib::Type::from_name("FSize")
    }

    fn bytes_len() -> usize {
        f32::bytes_len() + f32::bytes_len()
    }
}

impl ToBytes for FSize {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.width.to_bytes());
        bytes.append(&mut self.height.to_bytes());
        bytes
    }
}

impl FromBytes for FSize {
    fn from_bytes(data: &[u8], _len: usize) -> Self {
        let width = f32::from_bytes(&data[0..4], 4);
        let height = f32::from_bytes(&data[4..8], 4);
        Self { width, height }
    }
}

impl ToValue for FSize {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl FromValue for FSize {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}
