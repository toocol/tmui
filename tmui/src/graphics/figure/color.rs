use hex_color::HexColor;
use tlib::{
    types::StaticType,
    values::{FromBytes, FromValue, ToBytes, ToValue},
    Value,
};

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// Color
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
    pub valid: bool,
}

impl Color {
    #[inline]
    pub fn new() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
            valid: false,
        }
    }

    #[inline]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: 255,
            valid: true,
        }
    }

    #[inline]
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
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
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
            valid: true,
        }
    }

    #[inline]
    pub fn hexcode(&self) -> String {
        HexColor::rgba(self.r, self.g, self.b, self.a).to_string()
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
        skia_safe::Color::from_argb(self.a, self.r, self.g, self.b)
    }
}

impl StaticType for Color {
    fn static_type() -> tlib::Type {
        tlib::Type::from_name("Color")
    }

    fn bytes_len() -> usize {
        u8::bytes_len() * 4
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
        let r = u8::from_bytes(&data[0..1], 1);
        let g = u8::from_bytes(&data[1..2], 1);
        let b = u8::from_bytes(&data[2..3], 1);
        let a = u8::from_bytes(&data[3..4], 1);
        let valid = data[4] == 1;
        Self { r, g, b, a, valid }
    }
}

impl ToValue for Color {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> tlib::Type {
        Self::static_type()
    }
}

impl FromValue for Color {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}
