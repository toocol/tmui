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


    fn contains_timer(id: u16) -> bool {
        Self::instance().timers.borrow().contains_key(&id)
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
                let timer = unsafe { timer.as_mut() };
                if timer.is_active() {
                    timer.check_timer()
                }
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
    started: bool,
    single_shoot: bool,
    triggered: i32,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            duration: Default::default(),
            last_strike: SystemTime::now(),
            object: Default::default(),
            started: false,
            single_shoot: false,
            triggered: 0,
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        TimerHub::instance().delete_timer(self.id())
    }
}

impl Timer {
    pub fn new() -> Self {
        Object::new(&[])
    }

    pub fn start(&mut self, duration: Duration) {
        if !TimerHub::contains_timer(self.id()) {
            TimerHub::instance().add_timer(self);
        }
        self.started = true;
        self.duration = duration;
        self.last_strike = SystemTime::now();
    }

    pub fn set_single_shot(&mut self, single_shot: bool) {
        self.single_shoot = single_shot
    }

    pub fn is_active(&self) -> bool {
        self.started
    }

    pub fn stop(&mut self) {
        self.started = false
    }

    pub fn check_timer(&mut self) {
        if let Ok(duration) = SystemTime::now().duration_since(self.last_strike) {
            if duration > self.duration {
                emit!(self.timeout());
                self.last_strike = SystemTime::now();

                if self.single_shoot {
                    self.started = false;
                }
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        object::{ObjectImpl, ObjectSubclass},
        prelude::*, connect,
    };
    use super::{TimerHub, Timer};

    #[extends_object]
    #[derive(Default)]
    pub struct Widget {
        num: i32,
    }

    impl ObjectSubclass for Widget {
        const NAME: &'static str = "Widget";

        type Type = Widget;

        type ParentType = Object;
    }

    impl ObjectImpl for Widget {}

    impl Widget {
        pub fn deal_num(&mut self) {
            println!("{}", self.num);
            self.num += 1;
        }
    }

    #[test]
    fn test_timer() {
        let mut action_hub = ActionHub::new();
        action_hub.initialize();

        let mut timer_hub = TimerHub::new();
        timer_hub.initialize();

        let mut widget: Widget = Object::new(&[]);
        let mut timer = Timer::new();

        connect!(timer, timeout(), widget, deal_num());
        timer.start(Duration::from_secs(1));

        loop {
            if widget.num >= 5 {
                break;
            }
            timer_hub.check_timers();
        }
    }
}