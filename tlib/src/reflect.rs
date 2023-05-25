use once_cell::sync::Lazy;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

/// User register the type reflect info by override the function [`type_register()`](crate::object::ObjectImpl::type_register).<br>
/// Framwork level type reflect info will write in proc-macro [`extends`](macros::extends)
/// by implementing [`InnerTypeRegister`].<br>
/// User can use macros [`cast!`](macros::cast!), [`cast_mut!`](macros::cast_mut!) or [`cast_boxed!`](macros::cast_boxed!) to cast runtime dyn trait object(ref).
///
/// ## example
/// ```ignore
/// use crate::prelude::*;
/// use crate::object::ObjectSubclass;
///
/// #[extends(Object)]
/// struct Foo {}
///
/// #[reflect_trait]
/// trait DoA {
///     fn do_a(&self);
/// }
/// impl DoA for Foo {
///     fn do_a(&self) {}
/// }
///
/// #[reflect_trait]
/// trait DoB {
///     fn do_b(&self);
/// }
/// impl DoB for Foo {
///     fn do_b(&self) {}
/// }
///
/// impl ObjectSubclass for Foo {
///    const NAME: &'static str = "Foo";
///
///     type Type = Foo;
///
///     type ParentType = Object;
/// }
/// impl ObjectImpl for Foo {
///     fn type_register(&self, type_registry: &mut TypeRegistry) {
///         type_registry.register::<Foo, DoA>();
///         type_registry.register::<Foo, DoB>();
///     }
/// }
///
/// fn cast(foo: &dyn DoA) {
///     if let Some(do_b) = cast!(foo as DoB) {
///         do_b.do_b();
///     }
/// }
/// ```
pub struct TypeRegistry {
    registers: HashMap<TypeId, TypeRegistration>,
}

impl TypeRegistry {
    /// Create a boxed TypeRegistry.
    fn new() -> Box<Self> {
        Box::new(Self {
            registers: Default::default(),
        })
    }

    /// Get the single instance of Registry.
    pub fn instance<'a>() -> &'static mut TypeRegistry {
        static mut TYPE_REGISTRY: Lazy<Box<TypeRegistry>> = Lazy::new(|| TypeRegistry::new());
        unsafe { &mut TYPE_REGISTRY }
    }

    pub fn register<T: Reflect, RT: FromType<T> + ReflectTrait>(&mut self) {
        self.registers
            .entry(TypeId::of::<T>())
            .or_insert(TypeRegistration {
                data: Default::default(),
            })
            .data
            .insert(TypeId::of::<RT>(), Box::new(RT::from_type()));
    }

    pub fn get_type_data<T: ReflectTrait>(obj: &dyn Reflect) -> Option<&T> {
        Self::instance()
            .registers
            .get(&obj.type_id())
            .and_then(|registration| {
                registration
                    .data
                    .get(&TypeId::of::<T>())
                    .and_then(|reflect| reflect.as_any().downcast_ref::<T>())
            })
    }
}

pub struct TypeRegistration {
    data: HashMap<TypeId, Box<dyn ReflectTrait>>,
}

pub trait InnerTypeRegister {
    /// Register the reflect type info to [`TypeRegistry`] in this function.
    fn inner_type_register(&self, type_registry: &mut TypeRegistry);
}

/// Auto implemented by defined [`extends`](macros::extends) on struct.
pub trait Reflect: Any + 'static {
    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;

    fn as_boxed_any(self: Box<Self>) -> Box<dyn Any>;

    fn as_reflect(&self) -> &dyn Reflect;

    fn as_mut_reflect(&mut self) -> &mut dyn Reflect;

    fn as_boxed_reflect(self: Box<Self>) -> Box<dyn Reflect>;
}

/// implemented for trait which defined proc-macro [`reflect_trait`](macros::reflect_trait())
pub trait ReflectTrait: Any + 'static {
    fn as_any(&self) -> &dyn Any;
}

pub trait FromType<T: Reflect>: ReflectTrait {
    fn from_type() -> Self;
}

#[macro_export]
macro_rules! impl_reflect {
    ( $st:ident ) => {
        impl Reflect for $st {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_mut_any(&mut self) -> &mut dyn Any {
                self
            }

            fn as_boxed_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }

            fn as_reflect(&self) -> &dyn Reflect {
                self
            }

            fn as_mut_reflect(&mut self) -> &mut dyn Reflect {
                self
            }

            fn as_boxed_reflect(self: Box<Self>) -> Box<dyn Reflect> {
                self
            }
        }
    };
}
