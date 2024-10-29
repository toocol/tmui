use crate::{prelude::AsAny, typedef::WrappedTypeId};
use nohash_hasher::IntMap;
use once_cell::sync::Lazy;
use std::{
    any::{Any, TypeId},
    ptr::addr_of_mut,
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
    registers: IntMap<WrappedTypeId, TypeRegistration>,
}

impl TypeRegistry {
    /// Create a boxed TypeRegistry.
    #[inline]
    fn new() -> Box<Self> {
        Box::new(Self {
            registers: Default::default(),
        })
    }

    /// Get the single instance of Registry.
    #[inline]
    pub fn instance() -> &'static mut TypeRegistry {
        static mut TYPE_REGISTRY: Lazy<Box<TypeRegistry>> = Lazy::new(TypeRegistry::new);
        unsafe { addr_of_mut!(TYPE_REGISTRY).as_mut().unwrap().as_mut() }
    }

    #[inline]
    pub fn register<T: Reflect, RT: FromType<T> + ReflectTrait>(&mut self) {
        self.registers
            .entry(TypeId::of::<T>().into())
            .or_insert(TypeRegistration {
                data: Default::default(),
            })
            .data
            .insert(TypeId::of::<RT>().into(), Box::new(RT::from_type()));
    }

    #[inline]
    pub fn get_type_data<T: ReflectTrait>(obj: &dyn Reflect) -> Option<&T> {
        Self::instance()
            .registers
            .get(&obj.type_id().into())
            .and_then(|registration| {
                registration
                    .data
                    .get(&TypeId::of::<T>().into())
                    .and_then(|reflect| reflect.as_any().downcast_ref::<T>())
            })
    }
}

pub struct TypeRegistration {
    data: IntMap<WrappedTypeId, Box<dyn ReflectTrait>>,
}

/// Auto implemented by defined [`extends`](macros::extends) on struct. <br>
///
/// [`Any`] trait bound was needed, because of the [`type_id()`](Any::type_id) function. <br>
/// if there was NO [`Any`] trait bound, the type_id() was [`dyn Reflect`](Reflect), can't get the actual [`TypeId`]:
/// ```ignore
/// fn func(obj: &dyn Reflect) {
///     ...
///     // If `Reflect` doesn't have `Any` trait bound:
///     assert_eq!(obj.type_id(), TypeId::of::<dyn Reflect>());
///     ...
/// }
/// ```
pub trait Reflect: Any + AsAny + 'static {
    fn as_reflect(&self) -> &dyn Reflect;

    fn as_reflect_mut(&mut self) -> &mut dyn Reflect;

    fn as_reflect_boxed(self: Box<Self>) -> Box<dyn Reflect>;
}

/// implemented for trait which defined proc-macro [`reflect_trait`](macros::reflect_trait())
pub trait ReflectTrait: Any + 'static {
    fn as_any(&self) -> &dyn Any;
}

pub trait FromType<T: Reflect>: ReflectTrait {
    fn from_type() -> Self;
}

#[macro_export]
macro_rules! reflect_bound {
    ( $name:ident ) => {
        impl Reflect for $name {
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

        impl AsAny for $name {
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
    };
}
