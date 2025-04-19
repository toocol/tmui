use super::Tr;
use crate::widget::WidgetImpl;
use nohash_hasher::IntMap;
use std::cell::RefCell;
use tlib::{
    object::{ObjectId, ObjectSubclass},
    Object,
};

thread_local! {
    static OWNER_STORAGE: RefCell<IntMap<ObjectId, Box<dyn WidgetImpl>>> = RefCell::new(IntMap::default());
}

pub(crate) struct TrAllocater;
impl TrAllocater {
    #[inline]
    pub(crate) fn insert(owner: Box<dyn WidgetImpl>) {
        OWNER_STORAGE.with_borrow_mut(|map| map.insert(owner.id(), owner));
    }

    #[inline]
    pub(crate) fn remove(id: ObjectId) {
        OWNER_STORAGE.with_borrow_mut(|map| map.remove(&id));
    }
}

pub trait TrAlloc<T: WidgetImpl + ObjectSubclass + Default> {
    #[inline]
    fn new_alloc() -> Tr<T> {
        let mut obj: Box<T> = Object::new(&[]);
        let tr = Tr::from_raw(obj.as_mut());
        TrAllocater::insert(obj);
        tr
    }
}

impl<T: WidgetImpl + ObjectSubclass + Default> TrAlloc<T> for T {}
