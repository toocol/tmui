#![allow(dead_code)]
use std::{cell::RefCell, collections::HashMap, fmt::Debug};

use lazy_static::__Deref;

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

pub trait ObjectOperation {
    fn set_property(&self, name: &'static str, value: Value);

    fn get_property(&self, name: &'static str) -> Option<Value>;
}

impl Object {
    pub fn new<T: ObjectSubclass + Default + ObjectImpl>() -> T {
        let obj = T::default();
        obj.construct();
        obj
    }

    pub fn _set_property(&self, name: &'static str, value: Value) {
        self.properties.borrow_mut().insert(name, Box::new(value));
    }

    pub fn _get_property(&self, name: &'static str) -> Option<Value> {
        let borrowed = self.properties.borrow();
        let val_opt = borrowed.get(name);
        if val_opt.is_some() {
            Some(val_opt.as_deref().unwrap().deref().clone())
        } else {
            None
        }
    }
}

impl ObjectOperation for Object {
    fn set_property(&self, name: &'static str, value: Value) {
        self._set_property(name, value)
    }

    fn get_property(&self, name: &'static str) -> Option<Value> {
        self._get_property(name)
    }
}

impl IsSubclassable for Object {}

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

pub trait IsSubclassable {}

pub trait ObjectSubclass: Debug + 'static {
    const NAME: &'static str;

    type Type: ObjectExt + ObjectOperation + StaticType;

    type ParentType: IsSubclassable + ObjectExt + ObjectOperation + StaticType;
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
