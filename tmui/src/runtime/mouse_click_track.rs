use crate::application::double_click_interval;
use std::time::Instant;
use tlib::namespace::MouseButton;

pub(crate) struct MouseClickTrack {
    last_clicked_time: Instant,
    last_clicked_button: MouseButton,
    click_count: i32,
}

impl MouseClickTrack {
    #[inline]
    pub fn new() -> Self {
        Self {
            last_clicked_time: Instant::now(),
            last_clicked_button: MouseButton::NoButton,
            click_count: 0,
        }
    }

    #[inline]
    pub fn click_count(&self) -> i32 {
        self.click_count
    }

    #[inline]
    pub fn receive_mouse_click(&mut self, mouse_button: MouseButton) {
        let eligible = self.last_clicked_time.elapsed().as_millis()
            <= double_click_interval() as u128
            && self.last_clicked_button == mouse_button;

        self.last_clicked_time = Instant::now();
        self.last_clicked_button = mouse_button;

        if self.click_count == 0 || eligible {
            self.click_count += 1;
        } else {
            self.click_count = 1;
        }
    }
}
