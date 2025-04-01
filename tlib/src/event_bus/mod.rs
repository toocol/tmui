pub mod event;
pub mod event_handle;

use crate::{nonnull_mut, prelude::*};
use event::{IEvent, IEventType};
use event_handle::EventHandle;
use log::warn;
use nohash_hasher::IntMap;
use std::{
    collections::VecDeque,
    hash::{DefaultHasher, Hasher},
    marker::PhantomData,
    ptr::NonNull,
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
        use $crate::common::event_bus::{InnerEventBus, event_handle::EventHandle};
        use $crate::builtin::action::AsMutPtr;
        use std::sync::atomic::{AtomicPtr, Ordering};
        use std::cell::RefCell;
        use std::ptr::null_mut;

        thread_local! {
            static EVENT_BUS: RefCell<InnerEventBus<$event, $event_type>> = RefCell::new(InnerEventBus::<$event, $event_type>::new());
            static INSTANCE_PTR: AtomicPtr<InnerEventBus<$event, $event_type>> = const { AtomicPtr::new(null_mut()) };
        }

        #[inline]
        fn with_event_bus<F: FnOnce(&mut InnerEventBus<$event, $event_type>)>(f: F) {
            INSTANCE_PTR.with(|ptr| {
                let event_bus = unsafe { ptr.load(Ordering::Acquire).as_mut().unwrap() };
                f(event_bus)
            });
        }

        #[derive(GodotClass)]
        #[class(base = Node)]
        pub struct EventBus {
            base: Base<Node>,
        }

        #[godot_api]
        impl INode for EventBus {
            #[inline]
            fn init(base: Base<Node>) -> Self {
                EVENT_BUS.with(|rf| {
                    INSTANCE_PTR.with(|ptr| ptr.store(rf.borrow_mut().as_mut_ptr(), Ordering::Release))
                });
                Self { base }
            }

            #[inline]
            fn process(&mut self, _: f64) {
                with_event_bus(|event_bus| event_bus.process_deferred_evts());
            }
        }

        impl EventBus {
            #[inline]
            pub fn register(handle: *mut dyn EventHandle<Event = $event, EventType = $event_type>) {
                with_event_bus(|event_bus| event_bus.register(handle))
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
