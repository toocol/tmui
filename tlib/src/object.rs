#![allow(dead_code)]
use crate::{
    prelude::{FromType, InnerTypeRegister, Reflect, ReflectTrait, TypeRegistry},
    types::{IsA, ObjectType, StaticType, Type, TypeDowncast},
    values::{ToValue, Value},
};
use macros::reflect_trait;
use std::{
    any::Any,
    collections::HashMap,
    sync::atomic::{AtomicU16, Ordering},
};

static ID_INCREMENT: AtomicU16 = AtomicU16::new(1);

/// Super type of object system, every subclass object should extends this struct by proc-marco `[extends_object]`,
/// and impl `ObjectSubclass, ObjectImpl`
///
/// ```
/// use tlib::prelude::*;
/// use tlib::object::{ObjectImpl, ObjectSubclass};
///
/// #[extends(Object)]
/// #[derive(Default)]
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
///     fn construct(&mut self) {
///         self.parent_construct();
///         // Processing your own logic
///     }
/// }
/// ```
#[derive(Debug)]
pub struct Object {
    id: u16,
    properties: HashMap<String, Box<Value>>,
}

impl Default for Object {
    fn default() -> Self {
        Self {
            id: ID_INCREMENT.fetch_add(1, Ordering::SeqCst),
            properties: HashMap::new(),
        }
    }
}

#[reflect_trait]
pub trait ObjectOperation {
    /// Returns the type of the object.
    ///
    /// Go to[`Function defination`](ObjectOperation::id) (Defined in [`ObjectOperation`])
    fn id(&self) -> u16;

    /// Go to[`Function defination`](ObjectOperation::set_property) (Defined in [`ObjectOperation`])
    fn set_property(&mut self, name: &str, value: Value);

    /// Go to[`Function defination`](ObjectOperation::get_property) (Defined in [`ObjectOperation`])
    fn get_property(&self, name: &str) -> Option<&Value>;
}

impl Object {
    pub fn new<T: ObjectSubclass + Default + ObjectImpl + ObjectOperation>(
        properties: &[(&str, &dyn ToValue)],
    ) -> T {
        let mut obj = T::default();
        obj.construct();

        let default_name = format!("{}#{}", T::NAME, obj.id());
        obj.set_property("name", default_name.to_value());

        // Set the customize properties, if user specified the name, replace the default one.
        for (name, value) in properties {
            obj.set_property(*name, value.to_value())
        }
        obj
    }

    pub fn primitive_set_property(&mut self, name: &str, value: Value) {
        self.properties.insert(name.to_string(), Box::new(value));
    }

    pub fn primitive_get_property(&self, name: &str) -> Option<&Value> {
        self.properties.get(name).and_then(|p| Some(&**p))
    }
}

impl ObjectOperation for Object {
    fn id(&self) -> u16 {
        self.id
    }

    fn set_property(&mut self, name: &str, value: Value) {
        self.primitive_set_property(name, value);
    }

    fn get_property(&self, name: &str) -> Option<&Value> {
        self.primitive_get_property(name)
    }
}

impl Reflect for Object {
    #[inline]
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    #[inline]
    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    #[inline]
    fn as_boxed_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }

    #[inline]
    fn as_reflect(&self) -> &dyn Reflect {
        self
    }

    #[inline]
    fn as_mut_reflect(&mut self) -> &mut dyn Reflect {
        self
    }

    #[inline]
    fn as_boxed_reflect(self: Box<Self>) -> Box<dyn Reflect> {
        self
    }
}

impl ObjectSubclass for Object {
    const NAME: &'static str = "Object";
}

impl ObjectType for Object {
    fn object_type(&self) -> Type {
        Self::static_type()
    }
}

impl IsA<Object> for Object {}

impl ObjectImpl for Object {}

impl ObjectImplExt for Object {
    /// Go to[`Function defination`](ObjectImplExt::parent_construct) (Defined in [`ObjectImplExt`])
    fn parent_construct(&mut self) {}

    /// Go to[`Function defination`](ObjectImplExt::parent_on_property_set) (Defined in [`ObjectImplExt`])
    fn parent_on_property_set(&mut self, _name: &str, _value: &Value) {}
}

impl InnerTypeRegister for Object {
    fn inner_type_register(&self, _: &mut TypeRegistry) {}
}

pub trait ObjectExt: StaticType {
    /// Returns `true` if the object is an instance of (can be cast to) `T`.
    ///
    /// Go to[`Function defination`](ObjectExt::is) (Defined in [`ObjectExt`])
    fn is<T: StaticType>(&self) -> bool;

    /// Returns the type of the object.
    ///
    /// Go to[`Function defination`](ObjectExt::type_) (Defined in [`ObjectExt`])
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

pub trait ObjectAcquire: ObjectImpl + Default {}
pub trait ParentType {
    fn parent_type(&self) -> Type;
}

pub trait ObjectSubclass: 'static {
    const NAME: &'static str;
}

impl<T: ObjectSubclass> StaticType for T {
    fn static_type() -> crate::types::Type {
        Type::from_name(T::NAME)
    }

    fn bytes_len() -> usize {
        0
    }
}

pub trait TypeName {
    fn type_name(&self) -> &'static str;
}

impl<T: ObjectSubclass> TypeName for T {
    fn type_name(&self) -> &'static str {
        Self::NAME
    }
}

impl<T: ObjectType> TypeDowncast for T {}

#[reflect_trait]
#[allow(unused_variables)]
pub trait ObjectImpl: ObjectImplExt + InnerTypeRegister + TypeName {
    /// Override this function should invoke `self.parent_construct()` manually.
    fn construct(&mut self) {
        self.parent_construct()
    }

    /// Override this function should invoke `self.parent_on_property_set()` manually.
    fn on_property_set(&mut self, name: &str, value: &Value) {
        self.parent_on_property_set(name, value)
    }

    /// `initialize()` will be called when widget as a `child` of another widget.
    /// ### All the signals/slots [`connect!()`] should be called in this function.
    fn initialize(&mut self) {}

    /// Override to register the reflect type info to [`TypeRegistry`] in this function.
    fn type_register(&self, type_registry: &mut TypeRegistry) {}
}

#[reflect_trait]
pub trait ObjectImplExt {
    fn parent_construct(&mut self);

    fn parent_on_property_set(&mut self, name: &str, value: &Value);
}

#[reflect_trait]
pub trait ObjectChildrenConstruct {
    fn children_construct(&mut self) {}
}
