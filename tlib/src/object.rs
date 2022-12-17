#![allow(dead_code)]
use std::{cell::RefCell, collections::HashMap, fmt::Debug};

use lazy_static::__Deref;

use crate::{
    types::{IsA, ObjectType, StaticType, Type},
    values::{ToValue, Value},
};

/// Super type of object system, every subclass object should extends this struct by proc-marco `[extends_object]`,
/// and impl `ObjectSubclass, ObjectImpl`
///
/// ```
/// use tlib::prelude::*;
/// use tlib::object::{ObjectImpl, ObjectSubclass};
///
/// #[extends_object]
/// pub struct SubObject {};
///
/// impl ObjectSubclass for SubObject {
///     const NAME: &'static str = "SubObject";
///
///     type Type = SubObject;
///
///     type ParentType = Object;
/// }
///
/// impl ObjectImpl for SubObject {
///     // overwrite this method if you need to processing your own logic during object constructing.
///     fn construct(&self) {
///         self.parent_construct();
///         // Processing your own logic
///     }
/// }
/// ```
#[derive(Debug)]
pub struct Object {
    properties: RefCell<HashMap<String, Box<Value>>>,
}

impl Default for Object {
    fn default() -> Self {
        Self {
            properties: RefCell::new(HashMap::new()),
        }
    }
}

pub trait ObjectOperation {
    fn set_property(&self, name: &str, value: Value);

    fn get_property(&self, name: &str) -> Option<Value>;
}

impl Object {
    pub fn new<T: ObjectSubclass + Default + ObjectImpl + ObjectOperation>(
        properties: &[(&str, &dyn ToValue)],
    ) -> T {
        let obj = T::default();
        obj.construct();
        for (name, value) in properties {
            obj.set_property(*name, value.to_value())
        }
        obj
    }

    pub fn primitive_set_property(&self, name: &str, value: Value) {
        self.properties
            .borrow_mut()
            .insert(name.to_string(), Box::new(value));
    }

    pub fn primitive_get_property(&self, name: &str) -> Option<Value> {
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
    fn set_property(&self, name: &str, value: Value) {
        self.primitive_set_property(name, value)
    }

    fn get_property(&self, name: &str) -> Option<Value> {
        self.primitive_get_property(name)
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

    type Type: ObjectExt + ObjectOperation + ObjectType;

    type ParentType: IsSubclassable + ObjectExt + ObjectOperation + ObjectType;
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
