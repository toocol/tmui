use crate::application::double_click_interval;
use std::time::Instant;
use tlib::namespace::{MouseButton, KeyboardModifier};

pub(crate) struct RuntimeTrack {
    pub(crate) modifier: KeyboardModifier,

    pub(crate) mouse_position: (i32, i32),
    mouse_button_state: MouseButton,

    last_clicked_time: Instant,
    last_clicked_button: MouseButton,
    click_count: i32,
}

impl RuntimeTrack {
    #[inline]
    pub fn new() -> Self {
        Self {
            modifier: KeyboardModifier::NoModifier,
            mouse_position: (0, 0),
            mouse_button_state: MouseButton::NoButton,
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
    pub fn mouse_button_state(&self) -> MouseButton {
        self.mouse_button_state
    }

    #[inline]
    pub fn receive_mouse_click(&mut self, mouse_button: MouseButton) {
        let eligible = self.last_clicked_time.elapsed().as_millis()
            <= double_click_interval() as u128
            && self.last_clicked_button == mouse_button;

        self.last_clicked_time = Instant::now();
        self.last_clicked_button = mouse_button;
        self.mouse_button_state = self.mouse_button_state.or(mouse_button);

        if self.click_count == 0 || eligible {
            self.click_count += 1;
        } else {
            self.click_count = 1;
        }
    }

    #[inline]
    pub fn receive_mouse_release(&mut self, button: MouseButton) {
        self.mouse_button_state = self.mouse_button_state.remove(button)
    }
}
