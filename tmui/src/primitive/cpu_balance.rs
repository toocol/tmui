use once_cell::sync::Lazy;
use std::time::{Duration, Instant};

const DEFAULT_PAYLOAD_THRESHOLD: usize = 40;
const PAYLOAD_INTERVAL: usize = 1;

const FRAME_DURATION: Duration = Duration::from_millis(16);
const PAYLOAD_RESET_DURATION: Duration = Duration::from_secs(PAYLOAD_INTERVAL as u64);
const MICROS_10_DURATION: Duration = Duration::from_micros(10);
const MILLIS_1_DURATION: Duration = Duration::from_millis(1);

/// Execute the cpu sleep strategy depends on program status.
pub(crate) struct CpuBalance {
    loop_start_instant: Instant,
    payload_instant: Instant,
    payload: usize,
    indicate: usize,
    high_load: bool,
}

impl CpuBalance {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            loop_start_instant: Instant::now(),
            payload_instant: Instant::now(),
            payload: 0,
            indicate: 0,
            high_load: false,
        }
    }

    #[inline]
    /// SAFETY: This function can only invoke in [`ApplicationBuilder::cpu_payload_threshold()`](tmui::application::ApplicationBuilder::cpu_payload_threshold())
    /// at the step of application generate.
    pub(crate) fn set_payload_threshold(threshold: usize) {
        *Self::payload_threshold() = threshold;
    }

    #[inline]
    /// SAFETY: When application start running, PAYLOAD_THRESHOLD will not change anymore.
    pub(crate) fn payload_threshold() -> &'static mut usize {
        static mut PAYLOAD_THRESHOLD: Lazy<usize> = Lazy::new(|| DEFAULT_PAYLOAD_THRESHOLD);
        unsafe { &mut PAYLOAD_THRESHOLD }
    }

    /// Add payload to CpuBalance
    #[inline]
    pub(crate) fn add_payload(&mut self) {
        self.payload += 1;
    }

    /// Check if the payload has reached the threshhold.
    #[inline]
    pub(crate) fn payload_check(&mut self) {
        let threshold = *Self::payload_threshold();
        self.high_load = self.payload >= threshold * PAYLOAD_INTERVAL;

        if self.payload_instant.elapsed() >= PAYLOAD_RESET_DURATION {
            self.indicate += 1;
            self.payload_instant = Instant::now();

            if self.payload < self.indicate * threshold * PAYLOAD_INTERVAL {
                self.indicate = 0;
                self.payload = 0;
            }
        }
    }

    /// Invoke when ui main each loop start.
    #[inline]
    pub(crate) fn loop_start(&mut self) {
        self.loop_start_instant = Instant::now();
    }

    /// Cpu sleep based on program status. <br>
    /// @param update Did any components render in the previous frame
    #[inline]
    pub(crate) fn sleep(&self, update: bool) {
        let cost = self.loop_start_instant.elapsed();
        if cost >= FRAME_DURATION {
            return;
        }

        if self.high_load {
            tlib::timer::sleep(MICROS_10_DURATION);
        } else if update {
            tlib::timer::sleep(FRAME_DURATION - cost);
        } else {
            std::thread::sleep(MILLIS_1_DURATION);
        }
    }
}
