#![allow(dead_code)]
use std::{cell::RefCell, collections::HashMap, fmt::Debug};

use crate::{prelude::StaticType, types::{Type, ObjectType, IsA}, values::Value};

#[derive(Debug)]
pub struct Object {
    properties: RefCell<HashMap<&'static str, Box<Value>>>,
}

impl Default for Object {
    fn default() -> Self {
        Self {
            properties: RefCell::new(HashMap::new()),
        }
    }
}

impl Object {
    pub fn new<T: ObjectSubclass + Default + ObjectImpl>() -> T {
        let obj = T::default();
        obj.construct();
        obj
    }
}

impl ObjectSubclass for Object {
    const NAME: &'static str = "Object";

    type Type = Object;

    type ParentType = Object;
}

impl ObjectType for Object {}

impl IsA<Object> for Object {}

impl ObjectImpl for Object {
    fn construct(&self) {
        println!("`Object` construct")
    }
}

impl ObjectImplExt for Object {
    fn parent_construct(&self) {}
}

pub trait ObjectExt: StaticType {
    /// Returns `true` if the object is an instance of (can be cast to) `T`.
    fn is<T: StaticType>(&self) -> bool;

    /// Returns the type of the object.
    fn type_(&self) -> Type;
}

impl<T: StaticType> ObjectExt for T {
    fn is<U: StaticType>(&self) -> bool {
        self.type_().is_a(U::static_type())
    }

    fn type_(&self) -> Type {
        Self::static_type()
    }
}

pub trait ObjectSubclass: Debug + 'static {
    const NAME: &'static str;

    type Type: ObjectImpl + StaticType;

    type ParentType: ObjectImpl + StaticType;
}

impl<T: ObjectSubclass> StaticType for T {
    fn static_type() -> crate::types::Type {
        Type::from_name(T::NAME)
    }

    fn bytes_len() -> usize {
        0
    }
}

pub trait ObjectImpl: ObjectSubclass + ObjectImplExt {
    fn construct(&self) {
        self.parent_construct()
    }
}

pub trait ObjectImplExt {
    fn parent_construct(&self);
}
