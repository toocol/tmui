#![allow(dead_code)]
pub struct Type {
    name: Option<&'static str>
}

impl Type {
    /// The fundamental type corresponding to `i8`
    pub const I8: Self = Self { name: Some("i8") };

    /// The fundamental type corresponding to `u8`
    pub const U8: Self = Self { name: Some("u8") };

    /// The fundamental type corresponding to `bool`
    pub const BOOL: Self = Self { name: Some("bool") };

    pub fn name(&self) -> &'static str {
        self.name.unwrap()
    }
}

/// tmui runtime dynamic type supporting.
pub trait StaticType {
    /// Get the static Type.
    fn static_type() -> Type;
}