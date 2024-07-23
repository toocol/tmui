use std::time::Instant;
use tlib::payload::PayloadWeight;

use crate::{
    animation::{frame_animator::FrameAnimatorMgr, mgr::AnimationMgr},
    application,
    application_window::ApplicationWindow,
    graphics::board::Board,
    loading::LoadingMgr,
    opti::tracker::Tracker,
    platform::logic_window::LogicWindow,
    primitive::{cpu_balance::CpuBalance, frame::Frame, Message},
    widget::{widget_ext::WidgetExt, widget_inner::WidgetInnerExt},
};

pub const FRAME_INTERVAL: u128 = 16000;

pub(crate) struct FrameMgr {
    frame: Frame,
    last_frame: Instant,
    frame_cnt: i32,
}

impl FrameMgr {
    pub(crate) fn new() -> Self {
        Self {
            frame: Frame::empty_frame(),
            last_frame: Instant::now(),
            frame_cnt: 0,
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
                *resized = false;
                application::request_high_load(false);
            } else if window.shared_widget_size_changed() {
                let _track = Tracker::start("shared_widget_resize");
                logic_window.resize(size_record.0, size_record.1, true);
            }
            window.set_shared_widget_size_changed(false);

            self.last_frame = Instant::now();
            self.frame_cnt += 1;

            self.frame = self.frame.next();
            AnimationMgr::with(|m| m.borrow_mut().process(self.frame));
            LoadingMgr::with(|m| m.borrow_mut().process(self.frame));
            FrameAnimatorMgr::with(|m| m.borrow_mut().process(self.frame));

            let update = board.invalidate_visual(self.frame);
            window.set_resize_redraw(false);
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
