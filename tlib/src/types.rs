#![allow(dead_code)]

use std::{fmt::Debug, mem::size_of};

use crate::object::ObjectSubclass;

type TType = u8;
const TTYPE_NONE: TType = 0x00;
const TTYPE_BOOL: TType = 0x01;
const TTYPE_I8: TType = 0x02;
const TTYPE_U8: TType = 0x03;
const TTYPE_I16: TType = 0x04;
const TTYPE_U16: TType = 0x05;
const TTYPE_I32: TType = 0x06;
const TTYPE_U32: TType = 0x07;
const TTYPE_I64: TType = 0x08;
const TTYPE_U64: TType = 0x09;
const TTYPE_I128: TType = 0x0A;
const TTYPE_U128: TType = 0x0B;
const TTYPE_F32: TType = 0x0C;
const TTYPE_F64: TType = 0x0D;
const TTYPE_STRING: TType = 0x0E;
const TTYPE_OBJECT: TType = 0x0F;
const TTYPE_ARRAY: TType = 0x10;
const TTYPE_TUPLE: TType = 0x11;
const TTYPE_USIZE: TType = 0x12;
const TTYPE_PATH_BUF: TType = 0x13;
const TTYPE_CHAR: TType = 0x14;

/// Fundamental type of ObjectSystem
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Type {
    /// Type code.
    ttype: TType,
    /// Type name.
    name: &'static str,
}

impl Type {
    /// The fundamental type corresponding to `()`
    pub const NONE: Self = Self {
        ttype: TTYPE_NONE,
        name: "()",
    };

    /// The fundamental type corresponding to `bool`
    pub const BOOL: Self = Self {
        ttype: TTYPE_BOOL,
        name: "bool",
    };

    /// The fundamental type corresponding to `i8`
    pub const I8: Self = Self {
        ttype: TTYPE_I8,
        name: "i8",
    };

    /// The fundamental type corresponding to `u8`
    pub const U8: Self = Self {
        ttype: TTYPE_U8,
        name: "u8",
    };

    /// The fundamental type corresponding to `i16`
    pub const I16: Self = Self {
        ttype: TTYPE_I16,
        name: "i16",
    };

    /// The fundamental type corresponding to `u16`
    pub const U16: Self = Self {
        ttype: TTYPE_U16,
        name: "u16",
    };

    /// The fundamental type corresponding to `i32`
    pub const I32: Self = Self {
        ttype: TTYPE_I32,
        name: "i32",
    };

    /// The fundamental type corresponding to `u32`
    pub const U32: Self = Self {
        ttype: TTYPE_U32,
        name: "u32",
    };

    /// The fundamental type corresponding to `i64`
    pub const I64: Self = Self {
        ttype: TTYPE_I64,
        name: "i64",
    };

    /// The fundamental type corresponding to `u64`
    pub const U64: Self = Self {
        ttype: TTYPE_U64,
        name: "u64",
    };

    /// The fundamental type corresponding to `i128`
    pub const I128: Self = Self {
        ttype: TTYPE_I128,
        name: "i128",
    };

    /// The fundamental type corresponding to `u128`
    pub const U128: Self = Self {
        ttype: TTYPE_U16,
        name: "u128",
    };

    /// The fundamental type corresponding to `f32`
    pub const F32: Self = Self {
        ttype: TTYPE_F32,
        name: "f32",
    };

    /// The fundamental type corresponding to `f64`
    pub const F64: Self = Self {
        ttype: TTYPE_F64,
        name: "f64",
    };

    /// The fundamental type corresponding to `String`
    pub const STRING: Self = Self {
        ttype: TTYPE_STRING,
        name: "String",
    };

    /// The fundamental type from which all objects are derived
    pub const OBJECT: Self = Self {
        ttype: TTYPE_OBJECT,
        name: "Object",
    };

    /// The fundamental type corresponding to `Vec`
    pub const ARRAY: Self = Self {
        ttype: TTYPE_ARRAY,
        name: "Array",
    };

    /// The fundamental type corresponding to `Vec`
    pub const TUPLE: Self = Self {
        ttype: TTYPE_TUPLE,
        name: "TUPLE",
    };

    /// The fundamental type corresponding to `Vec`
    pub const USIZE: Self = Self {
        ttype: TTYPE_USIZE,
        name: "usize",
    };

    pub const PATH_BUF: Self = Self {
        ttype: TTYPE_PATH_BUF,
        name: "PathBuf",
    };

    pub const CHAR: Self = Self {
        ttype: TTYPE_CHAR,
        name: "char",
    };

    #[inline]
    pub fn from_name(name: &'static str) -> Self {
        Type {
            ttype: TTYPE_OBJECT,
            name: name,
        }
    }

    #[inline]
    pub fn is_object(&self) -> bool {
        self.ttype == TTYPE_OBJECT
    }

    #[inline]
    pub fn is_a(&self, t: Self) -> bool {
        self.name == t.name
    }

    #[inline]
    pub fn name(&self) -> &'static str {
        self.name
    }
}

/// tmui runtime dynamic type supporting.
pub trait StaticType {
    /// Get the static Type.
    fn static_type() -> Type;

    /// Get the bytes length of Type.
    fn bytes_len() -> usize;

    fn static_type_(&self) -> Type {
        Self::static_type()
    }

    fn dyn_bytes_len(&self) -> usize {
        Self::bytes_len()
    }
}

impl StaticType for usize {
    fn static_type() -> Type {
        Type::USIZE
    }

    fn bytes_len() -> usize {
        size_of::<usize>()
    }
}

impl StaticType for bool {
    fn static_type() -> Type {
        Type::BOOL
    }

    fn bytes_len() -> usize {
        1
    }
}

impl StaticType for i8 {
    fn static_type() -> Type {
        Type::I8
    }

    fn bytes_len() -> usize {
        1
    }
}

impl StaticType for u8 {
    fn static_type() -> Type {
        Type::U8
    }

    fn bytes_len() -> usize {
        1
    }
}

impl StaticType for i16 {
    fn static_type() -> Type {
        Type::I16
    }

    fn bytes_len() -> usize {
        2
    }
}

impl StaticType for u16 {
    fn static_type() -> Type {
        Type::U16
    }

    fn bytes_len() -> usize {
        2
    }
}

impl StaticType for i32 {
    fn static_type() -> Type {
        Type::I32
    }

    fn bytes_len() -> usize {
        4
    }
}

impl StaticType for u32 {
    fn static_type() -> Type {
        Type::U32
    }

    fn bytes_len() -> usize {
        4
    }
}

impl StaticType for i64 {
    fn static_type() -> Type {
        Type::I64
    }

    fn bytes_len() -> usize {
        8
    }
}

impl StaticType for u64 {
    fn static_type() -> Type {
        Type::U64
    }

    fn bytes_len() -> usize {
        8
    }
}

impl StaticType for i128 {
    fn static_type() -> Type {
        Type::I128
    }

    fn bytes_len() -> usize {
        16
    }
}

impl StaticType for u128 {
    fn static_type() -> Type {
        Type::U128
    }

    fn bytes_len() -> usize {
        16
    }
}

impl StaticType for f32 {
    fn static_type() -> Type {
        Type::F32
    }

    fn bytes_len() -> usize {
        4
    }
}

impl StaticType for f64 {
    fn static_type() -> Type {
        Type::F64
    }

    fn bytes_len() -> usize {
        8
    }
}

impl StaticType for String {
    fn static_type() -> Type {
        Type::STRING
    }

    fn bytes_len() -> usize {
        0
    }

    fn dyn_bytes_len(&self) -> usize {
        // An extra '\0'
        self.bytes().len() + 1
    }
}

impl StaticType for &str {
    fn static_type() -> Type {
        Type::STRING
    }

    fn bytes_len() -> usize {
        0
    }

    fn dyn_bytes_len(&self) -> usize {
        // An extra '\0'
        self.bytes().len() + 1
    }
}

impl StaticType for char {
    fn static_type() -> Type {
        Type::CHAR
    }

    fn bytes_len() -> usize {
        size_of::<char>()
    }
}

impl<T: StaticType> StaticType for Vec<T> {
    fn static_type() -> Type {
        Type::ARRAY
    }

    fn bytes_len() -> usize {
        T::bytes_len()
    }

    fn dyn_bytes_len(&self) -> usize {
        let mut len = 0;
        for i in self.iter() {
            len += i.dyn_bytes_len();
        }
        len
    }
}

macro_rules! implements_tuple_type {
    ( $($x:ident),+ ) => {
        impl <$($x: StaticType,)*> StaticType for ($($x,)*) {
            fn static_type() -> Type {
                Type::TUPLE
            }

            fn bytes_len() -> usize {
                0
            }
        }
    };
}
implements_tuple_type!(A, B);
implements_tuple_type!(A, B, C);
implements_tuple_type!(A, B, C, D);
implements_tuple_type!(A, B, C, D, E);
implements_tuple_type!(A, B, C, D, E, F);
implements_tuple_type!(A, B, C, D, E, F, G);
implements_tuple_type!(A, B, C, D, E, F, G, H);
implements_tuple_type!(A, B, C, D, E, F, G, H, I);
implements_tuple_type!(A, B, C, D, E, F, G, H, I, J);
implements_tuple_type!(A, B, C, D, E, F, G, H, I, J, K);
implements_tuple_type!(A, B, C, D, E, F, G, H, I, J, K, L);
implements_tuple_type!(A, B, C, D, E, F, G, H, I, J, K, L, M);
implements_tuple_type!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
implements_tuple_type!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
implements_tuple_type!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
implements_tuple_type!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
implements_tuple_type!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
implements_tuple_type!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
implements_tuple_type!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);

pub trait ObjectType: 'static {
    fn object_type(&self) -> Type;
}

pub trait IsA<T: ObjectType + StaticType>:
    ObjectType + StaticType + ObjectSubclass + TypeDowncast
{
}

pub trait TypeDowncast: ObjectType + Sized {
    fn downcast_ref<R: StaticType>(&self) -> Option<&R> {
        downcast_ref::<Self, R>(self)
    }

    fn downcast_mut<R: StaticType>(&mut self) -> Option<&mut R> {
        downcast_ref_mut::<Self, R>(self)
    }
}

pub fn downcast_ref<T: ObjectType, R: StaticType>(obj: &T) -> Option<&R> {
    if obj.object_type().is_a(R::static_type()) {
        Some(unsafe { &*(obj as *const T as *const R) })
    } else {
        None
    }
}

pub fn downcast_ref_mut<T: ObjectType, R: StaticType>(obj: &mut T) -> Option<&mut R> {
    if obj.object_type().is_a(R::static_type()) {
        Some(unsafe { &mut *(obj as *mut T as *mut R) })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_tupe_types() {
        let tuple = ("Hello".to_string(), 1024, 12, 64.);
        println!("{:?}", tuple.static_type_())
    }
}
