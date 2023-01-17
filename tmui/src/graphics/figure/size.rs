use std::ops::{Add, Sub};
use tlib::{
    types::StaticType,
    values::{FromBytes, FromValue, ToBytes, ToValue},
    Type, Value,
};

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// Size
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
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
        tlib::Type::from_name("Point")
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