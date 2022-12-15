#![allow(dead_code)]
use crate::{prelude::StaticType, types::Type};

#[derive(Debug)]
pub struct Value {
    data: Vec<u8>,
    value_type: Type,
    /// field for `vec` value.
    element_len: Option<usize>,
}

impl Value {
    pub fn new<T: StaticType + ToBytes>(data: T) -> Self {
        Value {
            data: data.to_bytes(),
            value_type: T::static_type(),
            element_len: if T::static_type().is_a(Type::ARRAY) {
                Some(T::bytes_len())
            } else {
                None
            },
        }
    }

    pub fn get<T>(self) -> T
    where
        T: FromValue + StaticType,
    {
        if !self.value_type.is_a(T::static_type()) {
            panic!(
                "Get value type mismatched, require: `{}`, get: `{}`",
                self.value_type.name(),
                T::static_type().name()
            )
        }
        T::from_value(self)
    }
}

pub trait FromValue: FromBytes {
    fn from_value(value: Value) -> Self;
}

pub trait ToBytes {
    fn to_bytes(self) -> Vec<u8>;
}

pub trait FromBytes {
    fn from_bytes(data: Vec<u8>) -> Self;
}

pub trait ToValue: ToBytes {
    /// Convert a value to a `Value`.
    fn to_value(self) -> Value;

    /// Returns the type identifer of `self`.
    ///
    /// This is the type of the value to be returned by `to_value`.
    fn value_type(&self) -> Type;
}

impl ToBytes for bool {
    fn to_bytes(self) -> Vec<u8> {
        vec![if self { 1 } else { 0 }; 1]
    }
}
impl FromBytes for bool {
    fn from_bytes(data: Vec<u8>) -> Self {
        if data[0] == 1 {
            true
        } else {
            false
        }
    }
}
impl FromValue for bool {
    fn from_value(value: Value) -> Self {
        Self::from_bytes(value.data)
    }
}
impl ToValue for bool {
    fn to_value(self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl ToBytes for u8 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for u8 {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut bytes = [0u8; 1];
        bytes.copy_from_slice(&data);
        u8::from_be_bytes(bytes)
    }
}
impl FromValue for u8 {
    fn from_value(value: Value) -> Self {
        Self::from_bytes(value.data)
    }
}
impl ToValue for u8 {
    fn to_value(self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl ToBytes for i8 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for i8 {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut bytes = [0u8; 1];
        bytes.copy_from_slice(&data);
        i8::from_be_bytes(bytes)
    }
}
impl FromValue for i8 {
    fn from_value(value: Value) -> Self {
        Self::from_bytes(value.data)
    }
}
impl ToValue for i8 {
    fn to_value(self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl ToBytes for u16 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for u16 {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&data);
        u16::from_be_bytes(bytes)
    }
}
impl FromValue for u16 {
    fn from_value(value: Value) -> Self {
        Self::from_bytes(value.data)
    }
}
impl ToValue for u16 {
    fn to_value(self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl ToBytes for i16 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for i16 {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&data);
        i16::from_be_bytes(bytes)
    }
}
impl FromValue for i16 {
    fn from_value(value: Value) -> Self {
        Self::from_bytes(value.data)
    }
}
impl ToValue for i16 {
    fn to_value(self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl ToBytes for u32 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for u32 {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&data);
        u32::from_be_bytes(bytes)
    }
}
impl FromValue for u32 {
    fn from_value(value: Value) -> Self {
        Self::from_bytes(value.data)
    }
}
impl ToValue for u32 {
    fn to_value(self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl ToBytes for i32 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for i32 {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&data);
        i32::from_be_bytes(bytes)
    }
}
impl FromValue for i32 {
    fn from_value(value: Value) -> Self {
        Self::from_bytes(value.data)
    }
}
impl ToValue for i32 {
    fn to_value(self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl ToBytes for u64 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for u64 {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&data);
        u64::from_be_bytes(bytes)
    }
}
impl FromValue for u64 {
    fn from_value(value: Value) -> Self {
        Self::from_bytes(value.data)
    }
}
impl ToValue for u64 {
    fn to_value(self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl ToBytes for i64 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for i64 {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&data);
        i64::from_be_bytes(bytes)
    }
}
impl FromValue for i64 {
    fn from_value(value: Value) -> Self {
        Self::from_bytes(value.data)
    }
}
impl ToValue for i64 {
    fn to_value(self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl ToBytes for u128 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for u128 {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&data);
        u128::from_be_bytes(bytes)
    }
}
impl FromValue for u128 {
    fn from_value(value: Value) -> Self {
        Self::from_bytes(value.data)
    }
}
impl ToValue for u128 {
    fn to_value(self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl ToBytes for i128 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for i128 {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&data);
        i128::from_be_bytes(bytes)
    }
}
impl FromValue for i128 {
    fn from_value(value: Value) -> Self {
        Self::from_bytes(value.data)
    }
}
impl ToValue for i128 {
    fn to_value(self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl ToBytes for f32 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for f32 {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&data);
        f32::from_be_bytes(bytes)
    }
}
impl FromValue for f32 {
    fn from_value(value: Value) -> Self {
        Self::from_bytes(value.data)
    }
}
impl ToValue for f32 {
    fn to_value(self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl ToBytes for f64 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for f64 {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&data);
        f64::from_be_bytes(bytes)
    }
}
impl FromValue for f64 {
    fn from_value(value: Value) -> Self {
        Self::from_bytes(value.data)
    }
}
impl ToValue for f64 {
    fn to_value(self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl ToBytes for String {
    fn to_bytes(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}
impl FromBytes for String {
    fn from_bytes(data: Vec<u8>) -> Self {
        String::from_utf8(data).unwrap()
    }
}
impl FromValue for String {
    fn from_value(value: Value) -> Self {
        Self::from_bytes(value.data)
    }
}
impl ToValue for String {
    fn to_value(self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl ToBytes for &str {
    fn to_bytes(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}
impl ToValue for &str {
    fn to_value(self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl<T: StaticType + ToBytes> ToBytes for Vec<T> {
    fn to_bytes(self) -> Vec<u8> {
        let mut bytes = vec![];
        for b in self.into_iter() {
            bytes.append(&mut b.to_bytes());
            if T::static_type().is_a(Type::STRING) {
                bytes.push(0);
            }
        }
        bytes
    }
}
impl<T: StaticType + FromBytes> FromBytes for Vec<T> {
    fn from_bytes(_: Vec<u8>) -> Self {
        vec![]
    }
}
impl<T: StaticType + FromBytes> FromValue for Vec<T> {
    fn from_value(value: Value) -> Self {
        let mut value = value;
        let mut v = vec![];
        let len = value.element_len.take().unwrap();

        if T::static_type().is_a(Type::STRING) {
            let mut idx = 0;
            let mut seg_arr: Vec<u8> = vec![];
            loop {
                if value.data[idx] != 0 {
                    seg_arr.push(value.data[idx]);
                } else {
                    let seg = T::from_bytes(seg_arr);
                    v.push(seg);
                    seg_arr = vec![];
                }

                idx += 1;
                if idx >= value.data.len() {
                    break;
                }
            }
        } else {
            let mut idx = 0;
            loop {
                let mut bytes = vec![0u8; len];
                bytes.copy_from_slice(&value.data[idx..idx + len]);
                let seg = T::from_bytes(bytes);
                v.push(seg);

                idx += len;
                if idx >= value.data.len() {
                    break;
                }
            }
        }
        v
    }
}
impl<T: StaticType + ToBytes> ToValue for Vec<T> {
    fn to_value(self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}
