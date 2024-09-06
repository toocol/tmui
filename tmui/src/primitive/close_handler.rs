use std::{cell::RefCell, ptr::NonNull};
use tlib::{nonnull_mut, prelude::*};

thread_local! {
    static INSTANCE: RefCell<CloseHandlerMgr> = RefCell::new(CloseHandlerMgr::new());
}

pub struct CloseHandlerMgr {
    handlers: Vec<Option<NonNull<dyn CloseHandler>>>,
}

impl CloseHandlerMgr {
    #[inline]
    pub(crate) fn new() -> Self {
        Self { handlers: vec![] }
    }

    #[inline]
    pub(crate) fn process() {
        INSTANCE.with(|ins| {
            ins.borrow_mut().handlers.iter_mut().for_each(|h| nonnull_mut!(h).handle())
        })
    }

    #[inline]
    pub fn register(handler: &mut dyn CloseHandler) {
        INSTANCE.with(|ins| {
            ins.borrow_mut().handlers.push(NonNull::new(handler))
        })
    }
}

pub trait CloseHandlerRequire: CloseHandler {}

#[reflect_trait]
pub trait CloseHandler {
    fn handle(&mut self);
}