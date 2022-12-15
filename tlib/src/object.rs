#![allow(dead_code)]
use crate::types::Type;

pub struct Object {
    static_type: Type,
}

pub trait ObjectSubclass: 'static {
    const NAME: &'static str;
}

pub trait ObjectImpl: ObjectSubclass + ObjectImplExt {
    fn construct(&self) {
        self.parent_construct()
    }
}

pub trait ObjectImplExt {
    fn parent_construct(&self);
}

impl<T: ObjectImpl> ObjectImplExt for T {
    fn parent_construct(&self) {
        todo!()
    }
}