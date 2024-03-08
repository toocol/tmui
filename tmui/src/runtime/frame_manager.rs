use log::debug;
use std::time::Instant;
use tlib::payload::PayloadWeight;

use crate::{
    animation::manager::AnimationManager,
    application,
    application_window::ApplicationWindow,
    graphics::board::Board,
    loading::LoadingManager,
    opti::tracker::Tracker,
    platform::logic_window::LogicWindow,
    primitive::{cpu_balance::CpuBalance, frame::Frame, Message},
    widget::widget_ext::WidgetExt,
};

pub const FRAME_INTERVAL: u128 = 16000;

pub(crate) struct FrameManager {
    frame: Frame,
    last_frame: Instant,
    frame_cnt: i32,
    time_distribution: (i32, i32, i32, i32),
    log_instant: Instant,
}

impl FrameManager {
    pub(crate) fn new() -> Self {
        Self {
            frame: Frame::empty_frame(),
            last_frame: Instant::now(),
            frame_cnt: 0,
            time_distribution: (0, 0, 0, 0),
            log_instant: Instant::now(),
        }
    }

    #[inline]
    pub(crate) fn process<T, M>(
        &mut self,
        board: &mut Board,
        window: &mut ApplicationWindow,
        logic_window: &mut LogicWindow<T, M>,
        cpu_balance: &mut CpuBalance,
        resized: &mut bool,
        size_record: &(u32, u32),
    ) -> bool
    where
        T: 'static + Copy + Sync + Send,
        M: 'static + Copy + Sync + Send,
    {
        let elapsed = self.last_frame.elapsed();

        if elapsed.as_micros() >= FRAME_INTERVAL || Board::is_force_update() {
            if *resized {
                let _track = Tracker::start("window_resize");
                logic_window.resize(size_record.0, size_record.1, false);
                board.resize();
                logic_window.gl_resize(size_record.0, size_record.1);
                *resized = false;
                application::request_high_load(false);
            } else if window.shared_widget_size_changed() {
                let _track = Tracker::start("shared_widget_resize");
                logic_window.resize(size_record.0, size_record.1, true);
            }
            window.set_shared_widget_size_changed(false);

            self.last_frame = Instant::now();
            let frame_time = elapsed.as_micros() as f32 / 1000.;
            self.frame_cnt += 1;
            match frame_time as i32 {
                0..=16 => self.time_distribution.0 += 1,
                17..=19 => self.time_distribution.1 += 1,
                20..=24 => self.time_distribution.2 += 1,
                _ => self.time_distribution.3 += 1,
            }
            if self.log_instant.elapsed().as_secs() >= 1 {
                debug!(
                    "frame time distribution rate: [<17ms: {}%, 17-20ms: {}%, 20-25ms: {}%, >=25ms: {}%], frame time: {}ms",
                    self.time_distribution.0 as f32 / self.frame_cnt as f32 * 100., self.time_distribution.1 as f32 / self.frame_cnt as f32 * 100., self.time_distribution.2 as f32 / self.frame_cnt as f32 * 100., self.time_distribution.3 as f32 / self.frame_cnt as f32 * 100., frame_time
                    );
                self.log_instant = Instant::now();
            }

            self.frame = self.frame.next();
            AnimationManager::with(|m| m.borrow_mut().process(self.frame));
            LoadingManager::with(|m| m.borrow_mut().process(self.frame));

            let update = board.invalidate_visual();
            if window.minimized() {
                window.set_minimized(false);
            }
            if update {
                logic_window
                    .is_gl_backend()
                    .then(|| logic_window.gl_swap_buffers())
                    .or_else(|| {
                        if let Some(window_id) = window.winit_id() {
                            let msg = Message::VSync(window_id, Instant::now());
                            cpu_balance.add_payload(msg.payload_wieght());
                            window.send_message(msg);
                        };
                        None
                    });
            }
            update
        } else {
            false
        }
    }
}
