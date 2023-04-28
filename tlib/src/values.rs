#![allow(dead_code)]
use crate::types::{StaticType, Type};

/// Generic Value type, basically, most basic types can be converted to Value.
///
/// ```
/// use tlib::prelude::*;
///
/// let val = 12.to_value();
/// let get = val.get::<i32>();
/// assert_eq!(12, get);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value {
    data: Vec<u8>,
    value_type: Type,
    /// field for `vec` value.
    element_len: Option<usize>,
    /// field for `tuple` value.
    seg_len: Option<Vec<usize>>,
}

impl Value {
    pub fn new<T: StaticType + ToBytes>(data: &T) -> Self {
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

    pub fn new_with_seg_len<T: StaticType + ToBytes>(data: &T, seg_len: Vec<usize>) -> Self {
        Value {
            data: data.to_bytes(),
            value_type: T::static_type(),
            element_len: None,
            seg_len: Some(seg_len),
        }
    }

    pub fn empty() -> Self {
        Value {
            data: vec![],
            value_type: Type::NONE,
            element_len: None,
            seg_len: None,
        }
    }

    pub fn tuple_count(&self) -> usize {
        match self.seg_len.as_ref() {
            Some(seg_len) => seg_len.len(),
            None => 0,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
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

/// Trait to convert a fundamental value to a `Value`.
pub trait ToValue {
    /// Convert a value to a `Value`.
    fn to_value(&self) -> Value;

    /// Returns the type identifer of `self`.
    ///
    /// This is the type of the value to be returned by `to_value`.
    fn value_type(&self) -> Type;
}

impl ToValue for () {
    fn to_value(&self) -> Value {
        Value::empty()
    }

    fn value_type(&self) -> Type {
        Type::NONE
    }
}

impl<T: ToValue + StaticType> ToValue for &T {
    fn to_value(&self) -> Value {
        T::to_value(*self)
    }

    fn value_type(&self) -> Type {
        T::static_type()
    }
}

impl ToBytes for usize {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl FromBytes for usize {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&data);
        Self::from_be_bytes(bytes)
    }
}
impl FromValue for usize {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(&value.data, Self::bytes_len())
    }
}
impl ToValue for usize {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
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
    fn to_value(&self) -> Value {
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
    fn to_value(&self) -> Value {
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
    fn to_value(&self) -> Value {
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
    fn to_value(&self) -> Value {
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
    fn to_value(&self) -> Value {
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
    fn to_value(&self) -> Value {
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
    fn to_value(&self) -> Value {
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
    fn to_value(&self) -> Value {
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
    fn to_value(&self) -> Value {
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
    fn to_value(&self) -> Value {
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
    fn to_value(&self) -> Value {
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
    fn to_value(&self) -> Value {
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
    fn to_value(&self) -> Value {
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
        let data = if data[data.len() - 1] == 0 {
            &data[0..data.len() - 1]
        } else {
            data
        };
        String::from_utf8(data.to_vec()).unwrap()
    }
}
impl FromValue for String {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(&value.data, Self::bytes_len())
    }
}
impl ToValue for String {
    fn to_value(&self) -> Value {
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
    fn to_value(&self) -> Value {
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
    fn to_value(&self) -> Value {
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

macro_rules! implements_tuple_value{
    ( $($gtype:ident),+ => $($name:ident),+ => $($index:tt),+) => {
        impl <$($gtype: StaticType + ToBytes,)+> ToBytes for ($($gtype,)+) {
            fn to_bytes(&self) -> Vec<u8> {
                let mut vec = vec![];
                $(
                    vec.append(&mut self.$index.to_bytes());
                )+
                vec
            }
        }

        impl <$($gtype: StaticType,)+> FromBytes for ($($gtype,)+) {
            fn from_bytes(_data: &[u8], _: usize) -> Self {
                panic!("`Tuple` nestification was not supported.")
            }
        }

        #[allow(unused_assignments)]
        impl <$($gtype: StaticType + FromBytes + Default,)+> FromValue for ($($gtype,)+) {
            fn from_value(value: &Value) -> Self {
                let ($($name,)+);
                let vec = value.seg_len.as_ref().unwrap();
                let mut idx = 0;
                $(
                    let ret = from_value_generic::<$gtype>(&value.data, vec, idx, $index);
                    $name = ret.0;
                    idx = ret.1;
                )+
                ($($name,)+)
            }
        }

        impl <$($gtype: StaticType + ToBytes,)+> ToValue for ($($gtype,)+) {
            fn to_value(&self) -> Value {
                let seg_len = vec![$(self.$index.dyn_bytes_len(),)+];
                Value::new_with_seg_len(self, seg_len)
            }

            fn value_type(&self) -> Type {
                Self::static_type()
            }
        }
    };
}
implements_tuple_value!(A,B => a,b => 0,1);
implements_tuple_value!(A,B,C => a,b,c => 0,1,2);
implements_tuple_value!(A,B,C,D => a,b,c,d => 0,1,2,3);
implements_tuple_value!(A,B,C,D,E => a,b,c,d,e => 0,1,2,3,4);
implements_tuple_value!(A,B,C,D,E,F => a,b,c,d,e,f => 0,1,2,3,4,5);
implements_tuple_value!(A,B,C,D,E,F,G => a,b,c,d,e,f,g => 0,1,2,3,4,5,6);
implements_tuple_value!(A,B,C,D,E,F,G,H => a,b,c,d,e,f,g,h => 0,1,2,3,4,5,6,7);
implements_tuple_value!(A,B,C,D,E,F,G,H,I => a,b,c,d,e,f,g,h,i => 0,1,2,3,4,5,6,7,8);
implements_tuple_value!(A,B,C,D,E,F,G,H,I,J => a,b,c,d,e,f,g,h,i,j => 0,1,2,3,4,5,6,7,8,9);
implements_tuple_value!(A,B,C,D,E,F,G,H,I,J,K => a,b,c,d,e,f,g,h,i,j,k => 0,1,2,3,4,5,6,7,8,9,10);
implements_tuple_value!(A,B,C,D,E,F,G,H,I,J,K,L => a,b,c,d,e,f,g,h,i,j,k,l => 0,1,2,3,4,5,6,7,8,9,10,11);
implements_tuple_value!(A,B,C,D,E,F,G,H,I,J,K,L,M => a,b,c,d,e,f,g,h,i,j,k,l,m => 0,1,2,3,4,5,6,7,8,9,10,11,12);
implements_tuple_value!(A,B,C,D,E,F,G,H,I,J,K,L,M,N => a,b,c,d,e,f,g,h,i,j,k,l,m,n => 0,1,2,3,4,5,6,7,8,9,10,11,12,13);
implements_tuple_value!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O => a,b,c,d,e,f,g,h,i,j,k,l,m,n,o => 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14);
implements_tuple_value!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P => a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p => 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15);
implements_tuple_value!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q => a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q => 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
implements_tuple_value!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R => a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r => 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17);
implements_tuple_value!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S => a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s => 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18);
implements_tuple_value!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T => a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t => 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19);

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_value() {
        let tuple = (12, 64., "Hello".to_string(), 1024);
        let value = tuple.to_value();
        assert_eq!(tuple, value.get::<(i32, f64, String, i32)>())
    }
}
