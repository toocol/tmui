use std::time::Duration;

use log::warn;
use tlib::{
    connect,
    events::{KeyEvent, MouseEvent},
    figure::FontCalculation,
    run_after,
    skia_safe::ClipOp,
    timer::Timer,
};

use super::{Input, InputWrapper};
use crate::{
    application,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{widget_inner::WidgetInnerExt, WidgetImpl},
};

const TEXT_DEFAULT_WIDTH: i32 = 150;
const TEXT_DEFAULT_PADDING: f32 = 3.;

#[extends(Widget)]
#[run_after]
pub struct Text {
    input_wrapper: InputWrapper<String>,
    clear_focus: bool,

    cursor_position: usize,
    cursor_visible: bool,
    #[derivative(Default(value = "true"))]
    cursor_blink: bool,
    blink_timer: Timer,

    #[derivative(Default(value = "TEXT_DEFAULT_PADDING"))]
    text_padding: f32,
    text_window: FRect,
    text_draw_position: FPoint,

    font_dimension: (f32, f32),
    fixed_font: bool,
}

impl ObjectSubclass for Text {
    const NAME: &'static str = "Text";
}

impl ObjectImpl for Text {
    fn initialize(&mut self) {
        self.input_wrapper.init(self.id());
        self.set_mouse_tracking(true);
        self.font_changed();
        self.set_border_color(Color::GREY_MEDIUM);
        self.set_borders(1., 1., 1., 1.);

        connect!(self.blink_timer, timeout(), self, blink_event());
    }
}

impl WidgetImpl for Text {
    #[inline]
    fn run_after(&mut self) {
        self.parent_run_after();

        self.calc_text_geometry();
    }

    #[inline]
    fn enable_focus(&self) -> bool {
        true
    }

    fn paint(&mut self, painter: &mut Painter) {
        painter.save();
        painter.clip_rect_global(self.text_window, ClipOp::Intersect);

        if self.is_enable() {
            self.draw_enable(painter)
        } else {
            self.draw_disable(painter)
        }

        painter.restore();
    }

    fn font_changed(&mut self) {
        let font = self.font().to_skia_font();
        self.font_dimension = font.calc_font_dimension();
        self.fixed_font = font.is_fixed_font();

        let size = self.size();

        if size.width() == 0 {
            self.set_fixed_width(TEXT_DEFAULT_WIDTH);
            self.set_detecting_width(TEXT_DEFAULT_WIDTH);
        }

        if size.height() == 0 {
            let height = self.calc_widget_height() as i32;
            self.set_fixed_height(height);
            self.set_detecting_height(height);
        }

        if self.window_id() != 0 && self.window().initialized() {
            self.window().layout_change(self);
        }
    }

    fn on_get_focus(&mut self) {
        self.update();
        self.check_blink_timer(true);
    }

    fn on_lose_focus(&mut self) {
        self.update();
        self.clear_focus = true;
        self.check_blink_timer(false);
    }

    fn on_key_pressed(&mut self, event: &KeyEvent) {}

    fn on_mouse_pressed(&mut self, event: &MouseEvent) {}

    fn on_mouse_move(&mut self, event: &MouseEvent) {}
}

impl Input for Text {
    type Value = String;

    #[inline]
    fn input_type(&self) -> super::InputType {
        super::InputType::Text
    }

    #[inline]
    fn input_wrapper(&self) -> &InputWrapper<Self::Value> {
        &self.input_wrapper
    }
}

impl Text {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }

    #[inline]
    pub fn set_text_padding(&mut self, text_padding: f32) {
        if text_padding < 0.
            || text_padding * 2. + self.font_dimension.1 > self.size().height() as f32
        {
            warn!("The value of text padding was invalid, ignore padding set.");
            return;
        }

        let font = self.font();
        self.text_padding = text_padding;
    }

    #[inline]
    pub fn set_blink_cursor(&mut self, cursor_blink: bool) {
        if self.cursor_blink == cursor_blink {
            return;
        }

        self.check_blink_timer(self.is_focus());
    }
}

impl Text {
    fn draw_enable(&mut self, painter: &mut Painter) {
        let mut rect = self.contents_rect(Some(Coordinate::Widget));

        {
            let val_ref = self.value_ref();
            let val = val_ref.as_str();
        }

        self.draw_cursor(painter);
    }

    fn draw_disable(&mut self, painter: &mut Painter) {}

    fn draw_cursor(&mut self, painter: &mut Painter) {
        let cursor_x = self.text_window.x() + self.cursor_position as f32 * self.font_dimension.0;

        if self.cursor_visible {
            painter.set_color(Color::BLACK);
        } else {
            painter.set_color(self.background());
        }

        painter.set_line_width(1.);
        painter.draw_line_f_global(
            cursor_x,
            self.text_window.top(),
            cursor_x,
            self.text_window.bottom(),
        )
    }

    fn calc_text_geometry(&mut self) {
        let rect: FRect = self.rect().into();
        let font_height = self.font_dimension.1;
        let calced_height = self.calc_widget_height();
        let mut window = FRect::default();

        window.set_x(rect.x() + self.text_padding);
        if calced_height == rect.height() {
            window.set_y(rect.y() + self.text_padding);
        } else {
            let offset = (rect.height() - font_height) / 2.;
            window.set_y(rect.y() + offset.floor());
        }

        window.set_width(rect.width() - 2. * self.text_padding);
        window.set_height(font_height);

        self.text_window = window;
        self.text_draw_position = self.text_window.top_left();
    }

    #[inline]
    fn calc_widget_height(&self) -> f32 {
        (self.font_dimension.1 + 2. * self.text_padding).ceil()
    }

    fn check_blink_timer(&mut self, is_focus: bool) {
        if self.cursor_blink && is_focus {
            if self.blink_timer.is_active() {
                return;
            }
            self.blink_timer.start(Duration::from_millis(
                application::cursor_blinking_time() as u64
            ));

            if !self.cursor_visible {
                self.cursor_visible = true;
                self.update();
            }
        } else {
            if !self.blink_timer.is_active() {
                return;
            }
            self.blink_timer.stop();

            if self.cursor_visible {
                self.cursor_visible = false;
                self.update();
            }
        }
    }

    fn blink_event(&mut self) {
        self.cursor_visible = !self.cursor_visible;
        self.update();
    }
}
