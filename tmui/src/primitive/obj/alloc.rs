use super::Tr;
use crate::prelude::*;
use crate::widget::WidgetImpl;
use nohash_hasher::IntMap;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
use tlib::{
    object::{ObjectId, ObjectSubclass},
    Object,
};

type OwnerMap = IntMap<ObjectId, (Box<dyn WidgetImpl>, Rc<Cell<i32>>)>;

thread_local! {
    static OWNER_STORAGE: RefCell<OwnerMap> = RefCell::new(IntMap::default());
}

pub(crate) struct TrAllocater {}
impl TrAllocater {
    #[inline]
    pub(crate) fn insert(owner: Box<dyn WidgetImpl>, ref_count: Rc<Cell<i32>>) {
        OWNER_STORAGE.with_borrow_mut(|map| map.insert(owner.id(), (owner, ref_count)));
    }

    #[inline]
    pub(crate) fn remove(id: ObjectId) {
        OWNER_STORAGE.with_borrow_mut(|map| map.remove(&id));
    }
}

pub trait TrAlloc<T: WidgetImpl + ObjectSubclass + Default>: ObjectOperation {
    #[inline]
    fn new_alloc() -> Tr<T> {
        let mut obj: Box<T> = Object::new(&[]);
        let tr = Tr::from_raw(obj.as_mut());
        let ref_count = tr.clone_ref_count();
        TrAllocater::insert(obj, ref_count);
        tr
    }

    #[inline]
    fn to_tr(&self) -> Tr<T> {
        let id = self.id();
        OWNER_STORAGE.with_borrow_mut(|map| {
            if let Some((widget, ref_count)) = map.get_mut(&id) {
                Tr::new_directly(widget.downcast_mut::<T>().unwrap(), ref_count.clone())
            } else {
                panic!("The widget is not managed internally by `tmui`.")
            }
        })
    }

    #[inline]
    fn to_dyn_tr(&self) -> DynTr {
        let id = self.id();
        OWNER_STORAGE.with_borrow_mut(|map| {
            if let Some((widget, ref_count)) = map.get_mut(&id) {
                DynTr::new_directly(widget.as_mut(), ref_count.clone())
            } else {
                panic!("The widget is not managed internally by `tmui`.")
            }
        })
    }
}
impl<T: WidgetImpl + ObjectSubclass + Default> TrAlloc<T> for T {}

pub trait DynTrAlloc: WidgetImpl {
    #[inline]
    fn to_tr(&self) -> DynTr {
        let id = self.id();
        OWNER_STORAGE.with_borrow_mut(|map| {
            if let Some((widget, ref_count)) = map.get_mut(&id) {
                DynTr::new_directly(widget.as_mut(), ref_count.clone())
            } else {
                panic!("The widget is not managed internally by `tmui`.")
            }
        })
    }
}
impl DynTrAlloc for dyn WidgetImpl {}

#[cfg(test)]
mod tests {
    use crate::{
        prelude::{DynTr, TrAlloc},
        widget::Widget,
    };

    #[test]
    fn test_tr() {
        let tr = Widget::new_alloc();
        {
            let dyn_tr: DynTr = tr.clone().into();
            let _ = dyn_tr.clone();
        }
    }
}
