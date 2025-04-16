pub mod event;
pub mod event_handle;

use crate::{nonnull_mut, nonnull_ref, prelude::*};
use event::{IEvent, IEventType};
use event_handle::EventHandle;
use log::warn;
use nohash_hasher::IntMap;
use std::{
    collections::VecDeque,
    hash::{DefaultHasher, Hasher},
    marker::PhantomData,
    ptr::{self, NonNull},
};

pub type RegisterMap<E, T> =
    IntMap<u64, Vec<Option<NonNull<dyn EventHandle<Event = E, EventType = T>>>>>;

#[derive(Default)]
pub struct InnerEventBus<E: IEvent, T: IEventType> {
    register: RegisterMap<E, T>,
    deferred_events: VecDeque<E>,

    _t: PhantomData<T>,
}

impl<E: IEvent<EventType = T>, T: IEventType> InnerEventBus<E, T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            register: Default::default(),
            deferred_events: Default::default(),
            _t: PhantomData,
        }
    }

    #[inline]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn register(&mut self, handle: *mut dyn EventHandle<Event = E, EventType = T>) {
        for listen in unsafe { handle.as_ref() }.unwrap().listen() {
            let mut hasher = DefaultHasher::default();
            listen.hash(&mut hasher);
            let k = hasher.finish();

            self.register
                .entry(k)
                .or_default()
                .push(NonNull::new(handle));
        }
    }

    #[inline]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn remove(&mut self, handle: *const dyn EventHandle<Event = E, EventType = T>) {
        for listen in unsafe { handle.as_ref() }.unwrap().listen() {
            let mut hasher = DefaultHasher::default();
            listen.hash(&mut hasher);
            let k = hasher.finish();

            if let Some(hnds) = self.register.get_mut(&k) {
                hnds.retain(|h| {
                    let h = nonnull_ref!(h);
                    !ptr::eq(handle, h)
                });
            }
        }
    }

    #[inline]
    pub fn push(&mut self, e: E) {
        let mut hasher = DefaultHasher::default();
        e.ty().hash(&mut hasher);
        let k = hasher.finish();

        if let Some(registers) = self.register.get_mut(&k) {
            for handle in registers.iter_mut() {
                let handle = nonnull_mut!(handle);
                handle.handle(&e);
            }
        } else {
            warn!("[EventBus::push] Event `{:?}` has no registers.", e.ty());
        }
    }

    #[inline]
    pub fn push_deferred(&mut self, e: E) {
        self.deferred_events.push_back(e);
    }

    #[inline]
    pub fn process_deferred_evts(&mut self) {
        while let Some(e) = self.deferred_events.pop_front() {
            self.push(e)
        }
    }
}
impl<E: IEvent<EventType = T>, T: IEventType> AsMutPtr for InnerEventBus<E, T> {
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self {
        self as *mut Self
    }
}

#[macro_export]
macro_rules! event_bus_init {
    ( $event:ty, $event_type:ty ) => {
        use $crate::prelude::*;
        use $crate::event_bus::{InnerEventBus, event_handle::EventHandle};
        use $crate::actions::AsMutPtr;
        use std::sync::atomic::{AtomicPtr, Ordering};
        use std::cell::RefCell;
        use std::ptr::null_mut;

        thread_local! {
            static EVENT_BUS: RefCell<InnerEventBus<$event, $event_type>> = RefCell::new(InnerEventBus::<$event, $event_type>::new());
            static INSTANCE_PTR: AtomicPtr<InnerEventBus<$event, $event_type>> = AtomicPtr::new({
                EVENT_BUS.with(|rf| {
                    rf.borrow_mut().as_mut_ptr()
                })
            });
        }

        #[inline]
        fn with_event_bus<F: FnOnce(&mut InnerEventBus<$event, $event_type>)>(f: F) {
            INSTANCE_PTR.with(|ptr| {
                let event_bus = unsafe { ptr.load(Ordering::Acquire).as_mut().unwrap() };
                f(event_bus)
            });
        }

        pub struct EventBus {}

        impl EventBus {
            #[inline]
            pub fn process() {
                with_event_bus(|event_bus| event_bus.process_deferred_evts());
            }

            #[inline]
            pub fn register(handle: *mut dyn EventHandle<Event = $event, EventType = $event_type>) {
                with_event_bus(|event_bus| event_bus.register(handle))
            }

            #[inline]
            pub fn remove(handle: *const dyn EventHandle<Event = $event, EventType = $event_type>) {
                with_event_bus(|event_bus| event_bus.remove(handle))
            }

            #[inline]
            pub fn push(e: $event) {
                with_event_bus(|event_bus| event_bus.push(e))
            }

            #[inline]
            pub fn push_deferred(e: $event) {
                with_event_bus(|event_bus| event_bus.push_deferred(e))
            }
        }
    };
}
