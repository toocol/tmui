#![allow(dead_code)]
use crate::{
    object::{ObjectImpl, ObjectSubclass},
    prelude::*,
    signal, signals, emit,
};
use lazy_static::lazy_static;
use log::warn;
use std::{
    cell::RefCell,
    collections::HashMap,
    ptr::{null_mut, NonNull},
    sync::{
        atomic::{AtomicPtr, Ordering},
        Once,
    },
    time::{Duration, SystemTime},
};

static INIT: Once = Once::new();
thread_local! {static IS_MAIN_THREAD: RefCell<bool>  = RefCell::new(false)}
lazy_static! {
    static ref TIMER_HUB: AtomicPtr<TimerHub> = AtomicPtr::new(null_mut());
}

/// `TimerHub` hold all raw pointer of [`Timer`]
pub struct TimerHub {
    timers: RefCell<Box<HashMap<u16, Option<NonNull<Timer>>>>>,
}

impl TimerHub {
    pub fn new() -> Self {
        Self {
            timers: RefCell::new(Box::new(HashMap::new())),
        }
    }

    pub fn instance<'a>() -> &'a Self {
        let timer_hub = TIMER_HUB.load(Ordering::SeqCst);
        unsafe {
            timer_hub
                .as_ref()
                .expect("`TimerHub` was not initialized, or already dead.")
        }
    }

    /// Intialize the `TimerHub`, this function should only call once.
    pub fn initialize(&mut self) {
        INIT.call_once(|| {
            IS_MAIN_THREAD.with(|is_main| *is_main.borrow_mut() = true);
            TIMER_HUB.store(self as *mut TimerHub, std::sync::atomic::Ordering::SeqCst);
        });
    }

    /// Check the timers, if any timer was arrival the interval, emit the [`timeout()`] signal.
    pub fn check_timers(&self) {
        for (id, timer) in self.timers.borrow_mut().iter_mut() {
            if let Some(timer) = timer.as_mut() {
                unsafe { timer.as_mut().check_timer() }
            } else {
                warn!("The raw pointer of timer was none, id = {}", id)
            }
        }
    }

    fn add_timer(&self, timer: &mut Timer) {
        self.timers
            .borrow_mut()
            .insert(timer.id(), NonNull::new(timer));
    }

    fn delete_timer(&self, id: u16) {
        self.timers.borrow_mut().remove(&id);
    }
}

/// Timing trigger `timeout()` sginal.
#[extends_object]
pub struct Timer {
    duration: Duration,
    last_strike: SystemTime,
}

impl Default for Timer {
    fn default() -> Self {
        let mut timer = Self {
            duration: Default::default(),
            last_strike: SystemTime::now(),
            object: Default::default(),
        };

        TimerHub::instance().add_timer(&mut timer);

        timer
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        TimerHub::instance().delete_timer(self.id())
    }
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        let mut timer: Timer = Object::new(&[]);
        timer.duration = duration;
        timer
    }

    pub fn check_timer(&mut self) {
        if let Ok(duration) = SystemTime::now().duration_since(self.last_strike) {
            if duration > self.duration {
                emit!(self.timeout());
                self.last_strike = SystemTime::now();
            }
        }
    }
}

pub trait TimerSignal: ActionExt {
    signals! {
        /// Triggered when timer reaches the time interval.
        timeout();
    }
}
impl TimerSignal for Timer {}

impl ObjectSubclass for Timer {
    const NAME: &'static str = "Timer";

    type Type = Timer;

    type ParentType = Object;
}

impl ObjectImpl for Timer {}
