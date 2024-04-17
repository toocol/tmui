#![allow(dead_code)]
use crate::{
    emit,
    object::{ObjectImpl, ObjectSubclass},
    prelude::*,
    signal, signals,
};
use log::warn;
use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    ptr::NonNull,
    time::{Duration, SystemTime},
};

thread_local! {
    static INSTANCE: Box<TimerHub> = TimerHub::new();
}

/// `TimerHub` hold all raw pointer of [`Timer`]
pub struct TimerHub {
    timers: RefCell<HashMap<ObjectId, Option<NonNull<Timer>>>>,
    once_timers: RefCell<HashMap<ObjectId, Box<Timer>>>,
}

impl TimerHub {
    pub(crate) fn new() -> Box<Self> {
        Box::new(Self {
            timers: RefCell::new(HashMap::new()),
            once_timers: RefCell::new(HashMap::new()),
        })
    }

    #[inline]
    pub fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&Box<Self>) -> R,
    {
        INSTANCE.with(f)
    }

    fn contains_timer(id: ObjectId) -> bool {
        Self::with(|hub| hub.timers.borrow().contains_key(&id))
    }

    /// Check the timers, if any timer was arrival the interval, emit the [`timeout()`] signal.
    #[inline]
    pub fn check_timers(&self) {
        for (id, timer) in self.timers.borrow_mut().iter_mut() {
            if let Some(timer) = timer.as_mut() {
                let timer = unsafe { timer.as_mut() };
                if timer.is_active() {
                    timer.check_timer();
                }
            } else {
                warn!("The raw pointer of timer was none, id = {}", id)
            }
        }

        self.once_timers.borrow_mut().retain(|_, timer| {
            if timer.is_active() {
                let shoot = timer.check_timer();
                if shoot && timer.once_timer {
                    timer.disconnect_all();
                    return false;
                }
            } else {
                // Just remove the un-actived once timer.
                timer.disconnect_all();
                return false;
            }
            true
        })
    }

    fn add_timer(&self, timer: &mut Timer) {
        self.timers
            .borrow_mut()
            .insert(timer.id(), NonNull::new(timer));
    }

    fn add_once_timer(&self, timer: Box<Timer>) -> RefMut<Timer> {
        let id = timer.id();
        self.once_timers.borrow_mut().insert(id, timer);
        RefMut::map(self.once_timers.borrow_mut(), |map| {
            map.get_mut(&id).unwrap().as_mut()
        })
    }

    fn delete_timer(&self, id: ObjectId) {
        self.timers.borrow_mut().remove(&id);
    }
}

/// Timing trigger `timeout()` sginal.
#[extends(Object)]
pub struct Timer {
    duration: Duration,
    #[derivative(Default(value = "SystemTime::now()"))]
    last_strike: SystemTime,
    started: bool,
    single_shoot: bool,
    once_timer: bool,
    triggered: i32,
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.disconnect_all();
        TimerHub::with(|hub| hub.delete_timer(self.id()))
    }
}

impl Timer {
    /// Normal constructor to build a `Timer`
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }

    /// Create an once timer.
    /// Once timer can only be executed once and will be removed later
    pub fn once<F: FnOnce(RefMut<Self>)>(f: F) {
        let mut timer = Self::new();
        timer.once_timer = true;
        TimerHub::with(|hub| {
            let once = hub.add_once_timer(timer);
            f(once);
        })
    }

    pub fn start(&mut self, duration: Duration) {
        if !TimerHub::contains_timer(self.id()) && !self.once_timer {
            TimerHub::with(|hub| hub.add_timer(self))
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

    pub fn check_timer(&mut self) -> bool {
        if let Ok(duration) = SystemTime::now().duration_since(self.last_strike) {
            if duration > self.duration {
                emit!(Timer::check_timer => self.timeout());
                self.last_strike = SystemTime::now();

                if self.single_shoot {
                    self.started = false;
                }
                return true;
            }
        }
        false
    }
}

pub trait TimerSignal: ActionExt {
    signals! {
        TimerSignal:

        /// Triggered when timer reaches the time interval.
        timeout();
    }
}
impl TimerSignal for Timer {}

impl ObjectSubclass for Timer {
    const NAME: &'static str = "Timer";
}

impl ObjectImpl for Timer {}

#[cfg(test)]
mod tests {
    use super::Timer;
    use crate::{
        connect,
        object::{ObjectImpl, ObjectSubclass},
        prelude::*,
        timer::TimerHub,
    };
    use std::time::{Duration, Instant};

    #[extends(Object)]
    pub struct Widget {
        num: i32,
    }

    impl ObjectSubclass for Widget {
        const NAME: &'static str = "Widget";
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
        ActionHub::initialize();

        let mut widget: Box<Widget> = Object::new(&[]);
        let mut timer = Timer::new();

        connect!(timer, timeout(), widget, deal_num());
        timer.start(Duration::from_millis(200));

        Timer::once(|mut timer| {
            connect!(timer, timeout(), widget, deal_num());
            timer.start(Duration::from_millis(1));
        });


        loop {
            if widget.num >= 5 {
                break;
            }
            TimerHub::with(|hub| hub.check_timers())
        }
        drop(timer);
    }

    #[test]
    fn test_sleep() {
        for _ in 0..10 {
            let mut now = Instant::now();
            std::thread::park_timeout(Duration::from_nanos(1));
            println!(
                "park_timeout: {}ms",
                now.elapsed().as_micros() as f32 / 1000.
            );
            now = Instant::now();

            std::thread::sleep(Duration::from_nanos(1));
            println!("sleep: {}ms", now.elapsed().as_micros() as f32 / 1000.);
            println!("---");
        }
    }
}
