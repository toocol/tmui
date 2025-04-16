use crate::{
    prelude::{AsAny, FromType, Reflect, ReflectTrait, TypeRegistry},
    types::{IsA, ObjectType, StaticType, Type, TypeDowncast},
    values::{ToValue, Value},
};
use ahash::AHashMap;
use macros::reflect_trait;
use std::{
    any::Any,
    sync::atomic::{AtomicU32, Ordering},
};

pub type IdGenerator = AtomicU32;
pub type ObjectId = u32;

static ID_INCREMENT: IdGenerator = IdGenerator::new(1);

/// Super type of object system, every subclass object should extends this struct by proc-marco `[extends_object]`,
/// and impl `ObjectSubclass, ObjectImpl`
///
/// ```
/// use tlib::prelude::*;
/// use tlib::object::{ObjectImpl, ObjectSubclass};
///
/// #[extends(Object)]
/// pub struct SubObject {};
///
/// impl ObjectSubclass for SubObject {
///     const NAME: &'static str = "SubObject";
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
    id: ObjectId,
    properties: AHashMap<String, Box<Value>>,
    constructed: bool,
    signal_source: Option<ObjectId>,
}

impl Default for Object {
    fn default() -> Self {
        Self {
            id: ID_INCREMENT.fetch_add(1, Ordering::SeqCst),
            properties: AHashMap::default(),
            constructed: false,
            signal_source: None,
        }
    }
}

#[reflect_trait]
pub trait ObjectOperation {
    /// Returns the type of the object.
    ///
    /// Go to[`Function defination`](ObjectOperation::id) (Defined in [`ObjectOperation`])
    fn id(&self) -> ObjectId;

    /// Go to[`Function defination`](ObjectOperation::set_property) (Defined in [`ObjectOperation`])
    fn set_property(&mut self, name: &str, value: Value);

    /// Go to[`Function defination`](ObjectOperation::get_property) (Defined in [`ObjectOperation`])
    fn get_property(&self, name: &str) -> Option<&Value>;

    /// Go to[`Function defination`](ObjectOperation::constructed) (Defined in [`ObjectOperation`])
    fn constructed(&self) -> bool;

    /// Set the signal source.
    fn set_signal_source(&mut self, id: Option<ObjectId>);

    /// Get the signal source.
    fn get_signal_source(&self) -> Option<ObjectId>;

    /// Set the name of object.
    ///
    /// Go to[`Function defination`](ObjectOperation::id) (Defined in [`ObjectOperation`])
    fn set_name(&mut self, name: &str) {
        self.set_property("name", name.to_value())
    }
}

impl Object {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: ObjectSubclass + Default + ObjectImpl + ObjectOperation>(
        properties: &[(&str, &dyn ToValue)],
    ) -> Box<T> {
        let mut obj = Box::<T>::default();

        let default_name = format!("{}#{}", T::NAME, obj.id());
        obj.set_property("name", default_name.to_value());

        // Set the customize properties, if user specified the name, replace the default one.
        for (name, value) in properties {
            obj.set_property(name, value.to_value())
        }

        obj.pretreat_construct();
        obj.construct();

        obj
    }

    pub fn primitive_set_property(&mut self, name: &str, value: Value) {
        self.properties.insert(name.to_string(), Box::new(value));
    }

    pub fn primitive_get_property(&self, name: &str) -> Option<&Value> {
        self.properties.get(name).map(|p| &**p)
    }
}

impl ObjectOperation for Object {
    #[inline]
    fn id(&self) -> ObjectId {
        self.id
    }

    #[inline]
    fn set_property(&mut self, name: &str, value: Value) {
        self.primitive_set_property(name, value);
    }

    #[inline]
    fn get_property(&self, name: &str) -> Option<&Value> {
        self.primitive_get_property(name)
    }

    #[inline]
    fn constructed(&self) -> bool {
        self.constructed
    }

    #[inline]
    fn set_signal_source(&mut self, id: Option<ObjectId>) {
        self.signal_source = id;
    }

    #[inline]
    fn get_signal_source(&self) -> Option<ObjectId> {
        self.signal_source
    }
}

impl AsAny for Object {
    #[inline]
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    #[inline]
    fn as_any_boxed(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

impl Reflect for Object {
    #[inline]
    fn as_reflect(&self) -> &dyn Reflect {
        self
    }

    #[inline]
    fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
        self
    }

    #[inline]
    fn as_reflect_boxed(self: Box<Self>) -> Box<dyn Reflect> {
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

impl ObjectImpl for Object {
    fn construct(&mut self) {
        self.parent_construct();

        self.constructed = true;
    }
}

impl ObjectImplExt for Object {
    /// Go to[`Function defination`](ObjectImplExt::parent_construct) (Defined in [`ObjectImplExt`])
    fn parent_construct(&mut self) {}

    /// Go to[`Function defination`](ObjectImplExt::parent_on_property_set) (Defined in [`ObjectImplExt`])
    fn parent_on_property_set(&mut self, _name: &str, _value: &Value) {}
}

pub trait InnerInitializer {
    /// Register the reflect type info to [`TypeRegistry`] in this function.
    fn inner_type_register(&self, type_registry: &mut TypeRegistry);

    /// Inner initialize function for widget.
    #[inline]
    fn inner_initialize(&mut self) {}

    /// Pretreatment when constructing.
    #[inline]
    fn pretreat_construct(&mut self) {}

    /// Inner handle the property setting processing.
    ///
    /// Retrun `true` if need prevent the `on_property_set` handle.
    #[allow(unused_variables)]
    #[inline]
    fn inner_on_property_set(&mut self, name: &str, value: &Value) -> bool {
        false
    }
}

impl InnerInitializer for Object {
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
pub trait SuperType {
    fn super_type(&self) -> Type;
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
pub trait ObjectImpl: ObjectImplExt + InnerInitializer + TypeName {
    /// Construct function when create the instance. <br>
    ///
    /// ### Object should create by function [`Object::new`]
    /// ### Override this function should invoke `self.parent_construct()` manually.
    fn construct(&mut self) {
        self.parent_construct()
    }

    /// Override this function should invoke `self.parent_on_property_set()` manually.
    fn on_property_set(&mut self, name: &str, value: &Value) {
        self.parent_on_property_set(name, value)
    }

    /// `initialize()` the widget. <br>
    /// Widget's parent or child can be acquired in this function.
    fn initialize(&mut self) {}

    /// Override to register the reflect type info to [`TypeRegistry`] in this function.
    fn type_register(&self, type_registry: &mut TypeRegistry) {}

    /// Any struct that extends `Object` will automatically implement `Drop` trait.
    ///
    /// To customize the behavior, override on_drop instead.
    fn on_drop(&mut self) {}
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
