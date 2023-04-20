use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Default)]
pub struct TypeRegistry {
    registers: HashMap<TypeId, TypeRegistration>,
}

impl TypeRegistry {
    pub fn register<Type: Reflect, ReflectTrait: FromType<Type> + Any>(&mut self) {
        self.registers
            .entry(TypeId::of::<Type>())
            .or_insert(TypeRegistration {
                data: Default::default(),
            })
            .data
            .insert(
                TypeId::of::<ReflectTrait>(),
                Box::new(ReflectTrait::from_type()),
            );
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

pub trait Reflect: Any + 'static {
    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;

    fn as_boxed_any(self: Box<Self>) -> Box<dyn Any>;
}

pub trait ReflectTrait: Any + 'static {
    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;

    fn as_boxed_any(self: Box<Self>) -> Box<dyn Any>;
}

pub trait FromType<T: Reflect>: ReflectTrait {
    fn from_type() -> Self;
}
