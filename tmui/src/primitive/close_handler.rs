use std::{cell::RefCell, ptr::NonNull};
use tlib::{nonnull_mut, nonnull_ref, prelude::*};

use crate::widget::WidgetImpl;

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
            ins.borrow_mut()
                .handlers
                .iter_mut()
                .for_each(|h| nonnull_mut!(h).handle())
        })
    }

    #[inline]
    pub fn register(handler: &mut dyn CloseHandler) {
        INSTANCE.with(|ins| ins.borrow_mut().handlers.push(NonNull::new(handler)))
    }

    #[inline]
    pub fn remove(id: ObjectId) {
        INSTANCE.with(|inst| {
            inst.borrow_mut()
                .handlers
                .retain_mut(|r| nonnull_ref!(r).id() != id)
        })
    }
}

pub trait CloseHandlerRequire: CloseHandler {}

#[reflect_trait]
pub trait CloseHandler: WidgetImpl {
    fn handle(&mut self);
}
