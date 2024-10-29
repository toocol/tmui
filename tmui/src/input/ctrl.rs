use super::{InputBounds, InputWrapper};
use nohash_hasher::IntMap;
use std::{cell::RefCell, rc::Rc};
use tlib::object::ObjectId;

pub struct RadioControl<T: InputBounds> {
    selected: RefCell<Option<Rc<InputWrapper<T>>>>,
    wrappers: RefCell<IntMap<ObjectId, Rc<InputWrapper<T>>>>,
}

impl<T: InputBounds> RadioControl<T> {
    #[inline]
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            selected: RefCell::new(None),
            wrappers: RefCell::new(IntMap::default()),
        })
    }

    #[inline]
    pub fn value(&self) -> Option<T> {
        self.selected.borrow().as_ref().map(|s| s.value())
    }

    #[inline]
    pub(crate) fn link_wrapper(&self, wrapper: Rc<InputWrapper<T>>) {
        self.wrappers.borrow_mut().insert(wrapper.id(), wrapper);
    }
}
