use std::{cell::RefCell, collections::HashMap, rc::Rc};

use tlib::object::ObjectId;

use super::{InputBounds, InputWrapper};

pub struct RadioControl<T: InputBounds> {
    selected: RefCell<Option<Rc<InputWrapper<T>>>>,
    wrappers: RefCell<HashMap<ObjectId, Rc<InputWrapper<T>>>>,
}

impl<T: InputBounds> RadioControl<T> {
    #[inline]
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            selected: RefCell::new(None),
            wrappers: RefCell::new(HashMap::new()),
        })
    }

    #[inline]
    pub fn value(&self) -> Option<T> {
        self.selected
            .borrow()
            .as_ref()
            .and_then(|s| Some(s.value()))
            .or(None)
    }

    #[inline]
    pub(crate) fn link_wrapper(&self, wrapper: Rc<InputWrapper<T>>) {
        self.wrappers.borrow_mut().insert(wrapper.id(), wrapper);
    }
}
