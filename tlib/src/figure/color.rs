use std::ops::{Add, Sub, Mul, Div};

use crate::{
    types::StaticType,
    values::{FromBytes, FromValue, ToBytes, ToValue},
    Value,
};
use hex_color::HexColor;

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// Color
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Color {
    pub r: i16,
    pub g: i16,
    pub b: i16,
    pub a: i16,
    pub valid: bool,
}

impl Color {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as i16,
            g: g as i16,
            b: b as i16,
            a: 255,
            valid: true,
        }
    }

    #[inline]
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as i16,
            g: g as i16,
            b: b as i16,
            a: a as i16,
            valid: true,
        }
    }

    #[inline]
    pub const fn from_rgb_uncheck(r: i16, g: i16, b: i16) -> Self {
        Self {
            r,
            g,
            b,
            a: 255,
            valid: true,
        }
    }

    #[inline]
    pub const fn from_rgba_uncheck(r: i16, g: i16, b: i16, a: i16) -> Self {
        Self {
            r,
            g,
            b,
            a,
            valid: true,
        }
    }

    #[inline]
    pub fn from_hex(hex_code: &str) -> Self {
        let color = HexColor::parse(hex_code)
            .expect(format!("Parse hex color failed, {}", hex_code).as_str());
        Self {
            r: color.r as i16,
            g: color.g as i16,
            b: color.b as i16,
            a: color.a as i16,
            valid: true,
        }
    }

    #[inline]
    pub fn hexcode(&self) -> String {
        HexColor::rgba(self.r as u8, self.g as u8, self.b as u8, self.a as u8).to_string()
    }

    #[inline]
    pub fn r(&self) -> u8 {
        self.r as u8
    }

    #[inline]
    pub fn g(&self) -> u8 {
        self.g as u8
    }

    #[inline]
    pub fn b(&self) -> u8 {
        self.b as u8
    }

    #[inline]
    pub fn a(&self) -> u8 {
        self.a as u8
    }
 
    #[inline]
    pub fn r_i16(&self) -> i16 {
        self.r
    }

    #[inline]
    pub fn g_i16(&self) -> i16 {
        self.g
    }

    #[inline]
    pub fn b_i16(&self) -> i16 {
        self.b
    }

    #[inline]
    pub fn a_i16(&self) -> i16 {
        self.a
    }

    #[inline]
    pub fn set_transparency(&mut self, a: u8) {
        self.a = a as i16
    }

    pub const RED: Color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
        valid: true,
    };
    pub const GREEN: Color = Color {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
        valid: true,
    };
    pub const YELLOW: Color = Color {
        r: 255,
        g: 255,
        b: 0,
        a: 255,
        valid: true,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
        valid: true,
    };
    pub const MAGENTA: Color = Color {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
        valid: true,
    };
    pub const CYAN: Color = Color {
        r: 0,
        g: 255,
        b: 255,
        a: 255,
        valid: true,
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
        valid: true,
    };
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
        valid: true,
    };
    pub const GREY: Color = Color {
        r: 128,
        g: 128,
        b: 128,
        a: 255,
        valid: true,
    };
    pub const TRANSPARENT: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
        valid: true,
    };
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::from_rgb(r, g, b)
    }
}

impl From<(i32, i32, i32)> for Color {
    fn from((r, g, b): (i32, i32, i32)) -> Self {
        Self::from_rgb(r as u8, g as u8, b as u8)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self::from_rgba(r, g, b, a)
    }
}

impl From<(i32, i32, i32, i32)> for Color {
    fn from((r, g, b, a): (i32, i32, i32, i32)) -> Self {
        Self::from_rgba(r as u8, g as u8, b as u8, a as u8)
    }
}

impl From<String> for Color {
    fn from(hex: String) -> Self {
        Self::from_hex(hex.as_str())
    }
}

impl From<&str> for Color {
    fn from(hex: &str) -> Self {
        Self::from_hex(hex)
    }
}

impl Into<(i32, i32, i32)> for Color {
    fn into(self) -> (i32, i32, i32) {
        (self.r as i32, self.g as i32, self.b as i32)
    }
}

impl Into<(i32, i32, i32)> for &Color {
    fn into(self) -> (i32, i32, i32) {
        (self.r as i32, self.g as i32, self.b as i32)
    }
}

impl Into<skia_safe::Color> for Color {
    fn into(self) -> skia_safe::Color {
        skia_safe::Color::from_argb(self.a(), self.r(), self.g(), self.b())
    }
}

impl Into<skia_safe::Color4f> for Color {
    fn into(self) -> skia_safe::Color4f {
        skia_safe::Color4f {
            r: self.r as f32,
            g: self.g as f32,
            b: self.b as f32,
            a: self.a as f32,
        }
    }
}

impl Into<skia_safe::Color3f> for Color {
    fn into(self) -> skia_safe::Color3f {
        skia_safe::Color3f::new(self.r as f32, self.g as f32, self.b as f32)
    }
}

impl StaticType for Color {
    fn static_type() -> crate::Type {
        crate::Type::from_name("Color")
    }

    fn bytes_len() -> usize {
        i16::bytes_len() * 4 + 1
    }
}
impl ToBytes for Color {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.r.to_bytes());
        bytes.append(&mut self.g.to_bytes());
        bytes.append(&mut self.b.to_bytes());
        bytes.append(&mut self.a.to_bytes());
        bytes.append(&mut self.valid.to_bytes());
        bytes
    }
}

impl FromBytes for Color {
    fn from_bytes(data: &[u8], _len: usize) -> Self {
        let r = i16::from_bytes(&data[0..2], 2);
        let g = i16::from_bytes(&data[2..4], 2);
        let b = i16::from_bytes(&data[4..6], 2);
        let a = i16::from_bytes(&data[6..8], 2);
        let valid = data[8] == 1;
        Self { r, g, b, a, valid }
    }
}

impl ToValue for Color {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> crate::Type {
        Self::static_type()
    }
}

impl FromValue for Color {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

impl Add<Color> for Color {
    type Output = Color;

    #[inline]
    fn add(self, rhs: Color) -> Self::Output {
        Self::from_rgba_uncheck(
            self.r + rhs.r,
            self.g + rhs.g,
            self.b + rhs.b,
            self.a + rhs.a,
        )
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    #[inline]
    fn sub(self, rhs: Color) -> Self::Output {
        Self::from_rgba_uncheck(
            self.r - rhs.r,
            self.g - rhs.g,
            self.b - rhs.b,
            self.a - rhs.a,
        )
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    #[inline]
    fn mul(self, rhs: Color) -> Self::Output {
        Self::from_rgba_uncheck(
            self.r * rhs.r,
            self.g * rhs.g,
            self.b * rhs.b,
            self.a * rhs.a,
        )
    }
}

impl Div<Color> for Color {
    type Output = Color;

    #[inline]
    fn div(self, rhs: Color) -> Self::Output {
        Self::from_rgba_uncheck(
            self.r / rhs.r,
            self.g / rhs.g,
            self.b / rhs.b,
            self.a / rhs.a,
        )
    }
}