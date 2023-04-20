use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Default)]
pub struct TypeRegistry {
    registers: HashMap<TypeId, TypeRegistration>,
}

impl TypeRegistry {
    pub fn register<T: Reflect, RT: FromType<T> + ReflectTrait>(&mut self) {
        self.registers
            .entry(TypeId::of::<T>())
            .or_insert(TypeRegistration {
                data: Default::default(),
            })
            .data
            .insert(TypeId::of::<RT>(), Box::new(RT::from_type()));
    }

    pub fn get_type_data<T: ReflectTrait>(&self, obj: &dyn Reflect) -> Option<&T> {
        self.registers.get(&obj.type_id()).and_then(|registration| {
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
    fn inner_type_register(&mut self, type_registry: &mut TypeRegistry);
}

pub trait Reflect: Any + 'static {
    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;

    fn as_boxed_any(self: Box<Self>) -> Box<dyn Any>;

    fn as_reflect(&self) -> &dyn Reflect;

    fn as_mut_reflect(&mut self) -> &mut dyn Reflect;

    fn as_boxed_reflect(self: Box<Self>) -> Box<dyn Reflect>;
}

pub trait ReflectTrait: Any + 'static {
    fn as_any(&self) -> &dyn Any;
}

pub trait FromType<T: Reflect>: ReflectTrait {
    fn from_type() -> Self;
}
