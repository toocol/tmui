#![allow(dead_code)]
use crate::{prelude::StaticType, types::Type};

#[derive(Debug, Clone)]
pub struct Value {
    data: Vec<u8>,
    value_type: Type,
    /// field for `vec` value.
    element_len: Option<usize>,
    /// field for `tuple` value.
    seg_len: Option<Vec<usize>>,
}

impl Value {
    fn new<T: StaticType + ToBytes>(data: T) -> Self {
        Value {
            data: data.to_bytes(),
            value_type: T::static_type(),
            element_len: if T::static_type().is_a(Type::ARRAY) {
                Some(T::bytes_len())
            } else {
                None
            },
            seg_len: None,
        }
    }

    fn new_with_seg_len<T: StaticType + ToBytes>(data: T, seg_len: Vec<usize>) -> Self {
        Value {
            data: data.to_bytes(),
            value_type: T::static_type(),
            element_len: if T::static_type().is_a(Type::ARRAY) {
                Some(T::bytes_len())
            } else {
                None
            },
            seg_len: Some(seg_len),
        }
    }

    pub fn get<T>(&self) -> T
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
    fn from_value(value: &Value) -> Self;
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait FromBytes {
    fn from_bytes(data: &[u8], len: usize) -> Self;
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
    fn to_bytes(&self) -> Vec<u8> {
        vec![if *self { 1 } else { 0 }; 1]
    }
}
impl FromBytes for bool {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        if data[0] == 1 {
            true
        } else {
            false
        }
    }
}
impl FromValue for bool {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(&value.data, bool::bytes_len())
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
    fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for u8 {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        let mut bytes = [0u8; 1];
        bytes.copy_from_slice(data);
        u8::from_be_bytes(bytes)
    }
}
impl FromValue for u8 {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(&value.data, Self::bytes_len())
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
    fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for i8 {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        let mut bytes = [0u8; 1];
        bytes.copy_from_slice(data);
        i8::from_be_bytes(bytes)
    }
}
impl FromValue for i8 {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(&value.data, Self::bytes_len())
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
    fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for u16 {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(data);
        u16::from_be_bytes(bytes)
    }
}
impl FromValue for u16 {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(&value.data, Self::bytes_len())
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
    fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for i16 {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(data);
        i16::from_be_bytes(bytes)
    }
}
impl FromValue for i16 {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(&value.data, Self::bytes_len())
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
    fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for u32 {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(data);
        u32::from_be_bytes(bytes)
    }
}
impl FromValue for u32 {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(&value.data, Self::bytes_len())
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
    fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for i32 {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(data);
        i32::from_be_bytes(bytes)
    }
}
impl FromValue for i32 {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(&value.data, Self::bytes_len())
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
    fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for u64 {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(data);
        u64::from_be_bytes(bytes)
    }
}
impl FromValue for u64 {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(&value.data, Self::bytes_len())
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
    fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for i64 {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(data);
        i64::from_be_bytes(bytes)
    }
}
impl FromValue for i64 {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(&value.data, Self::bytes_len())
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
    fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for u128 {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(data);
        u128::from_be_bytes(bytes)
    }
}
impl FromValue for u128 {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(&value.data, Self::bytes_len())
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
    fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for i128 {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(data);
        i128::from_be_bytes(bytes)
    }
}
impl FromValue for i128 {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(&value.data, Self::bytes_len())
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
    fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for f32 {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&data);
        f32::from_be_bytes(bytes)
    }
}
impl FromValue for f32 {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(&value.data, Self::bytes_len())
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
    fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for f64 {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(data);
        f64::from_be_bytes(bytes)
    }
}
impl FromValue for f64 {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(&value.data, Self::bytes_len())
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
    fn to_bytes(&self) -> Vec<u8> {
        let mut vec = self.as_bytes().to_vec();
        vec.push(0);
        vec
    }
}
impl FromBytes for String {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        String::from_utf8(data.to_vec()).unwrap()
    }
}
impl FromValue for String {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(&value.data, Self::bytes_len())
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
    fn to_bytes(&self) -> Vec<u8> {
        let mut vec = self.as_bytes().to_vec();
        vec.push(0);
        vec
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
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        for b in self.into_iter() {
            bytes.append(&mut b.to_bytes());
        }
        bytes
    }
}
impl<T: StaticType + FromBytes> FromBytes for Vec<T> {
    fn from_bytes(data: &[u8], len: usize) -> Self {
        let mut v = vec![];

        if T::static_type().is_a(Type::STRING) {
            let mut idx = 0;
            let mut seg_arr: Vec<u8> = vec![];
            loop {
                if data[idx] != 0 {
                    seg_arr.push(data[idx]);
                } else {
                    let seg = T::from_bytes(&seg_arr, T::bytes_len());
                    v.push(seg);
                    seg_arr = vec![];
                }

                idx += 1;
                if idx >= data.len() {
                    break;
                }
            }
        } else {
            let mut idx = 0;
            loop {
                let mut bytes = vec![0u8; len];
                bytes.copy_from_slice(&data[idx..idx + len]);
                let seg = T::from_bytes(&bytes, T::bytes_len());
                v.push(seg);

                idx += len;
                if idx >= data.len() {
                    break;
                }
            }
        }
        v
    }
}
impl<T: StaticType + FromBytes> FromValue for Vec<T> {
    fn from_value(value: &Value) -> Self {
        let len = value.element_len.as_ref().unwrap();
        Self::from_bytes(&value.data, *len)
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

fn from_value_generic<T: StaticType + FromBytes + Default>(
    data: &[u8],
    vec: &Vec<usize>,
    mut idx: usize,
    flag: usize,
) -> (T, usize) {
    let mut t = T::default();
    if T::static_type().is_a(Type::STRING) {
        let mut seg_arr: Vec<u8> = vec![];
        loop {
            if data[idx] != 0 {
                seg_arr.push(data[idx]);
            } else {
                let seg = T::from_bytes(&seg_arr, T::bytes_len());
                t = seg;
                idx += 1;
                return (t, idx);
            }

            idx += 1;
            if idx >= data.len() {
                return (t, idx);
            }
        }
    } else {
        let len = vec[flag];
        let mut seg_arr = vec![0u8; len];
        seg_arr.copy_from_slice(&data[idx..idx + len]);
        t = T::from_bytes(&seg_arr, T::bytes_len());
        idx += len;
        return (t, idx);
    }
}

impl<A: StaticType + ToBytes, B: StaticType + ToBytes> ToBytes for (A, B) {
    fn to_bytes(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.append(&mut self.0.to_bytes());
        vec.append(&mut self.1.to_bytes());
        vec
    }
}
impl<A: StaticType, B: StaticType> FromBytes for (A, B) {
    fn from_bytes(_data: &[u8], _: usize) -> Self {
        panic!("`Tuple` nestification was not supported.")
    }
}
impl<A: StaticType + FromBytes + Default, B: StaticType + FromBytes + Default> FromValue
    for (A, B)
{
    fn from_value(value: &Value) -> Self {
        let a;
        let b;
        let vec = value.seg_len.as_ref().unwrap();
        let mut idx = 0;

        let ret = from_value_generic::<A>(&value.data, vec, idx, 0);
        a = ret.0;
        idx = ret.1;

        let ret = from_value_generic::<B>(&value.data, vec, idx, 1);
        b = ret.0;

        (a, b)
    }
}
impl<A: StaticType + ToBytes, B: StaticType + ToBytes> ToValue for (A, B) {
    fn to_value(self) -> Value {
        let seg_len = vec![self.0.bytes_len_(), self.1.bytes_len_()];
        Value::new_with_seg_len(self, seg_len)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}
