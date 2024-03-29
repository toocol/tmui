use super::{Input, InputSignals, InputWrapper};
use crate::{
    application,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{widget_inner::WidgetInnerExt, WidgetImpl},
};
use log::warn;
use std::time::Duration;
use tlib::{
    connect,
    events::{KeyEvent, MouseEvent},
    figure::FontCalculation,
    namespace::KeyCode,
    run_after,
    skia_safe::ClipOp,
    timer::Timer,
    typedef::SkiaFont,
};

const TEXT_DEFAULT_WIDTH: i32 = 150;
const TEXT_DEFAULT_PADDING: f32 = 3.;

#[extends(Widget)]
#[run_after]
pub struct Text {
    input_wrapper: InputWrapper<String>,
    clear_focus: bool,

    cursor_index: usize,
    cursor_position: f32,
    cursor_visible: bool,
    #[derivative(Default(value = "true"))]
    cursor_blink: bool,
    blink_timer: Timer,

    #[derivative(Default(value = "TEXT_DEFAULT_PADDING"))]
    text_padding: f32,
    text_window: FRect,
    text_draw_position: Option<FPoint>,
    #[derivative(Default(value = "Color::BLACK"))]
    text_color: Color,
    max_length: Option<usize>,

    font_dimension: (f32, f32),
    fixed_font: bool,
    skia_font: Option<SkiaFont>,
    letter_spacing: f32,
}

impl InputSignals for Text {}

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

        println!("Text mem size: {}", std::mem::size_of_val(self));
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

        self.skia_font = Some(font);
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

    fn on_key_pressed(&mut self, event: &KeyEvent) {
        let mut need_stop_timer = false;

        match event.key_code() {
            KeyCode::KeyBackspace => {
                need_stop_timer = true;
            }
            KeyCode::KeyTab => {}
            KeyCode::KeyLeft => {
                need_stop_timer = true;

                if self.cursor_index > 0 {
                    self.cursor_index -= 1;
                    self.calc_cursor_postion();
                }
            }
            KeyCode::KeyRight => {
                need_stop_timer = true;

                if self.cursor_index < self.value_ref().len() {
                    self.cursor_index += 1;
                    self.calc_cursor_postion();
                }
            }
            _ => {
                let text = event.text();
                if text.is_empty() {
                    return;
                }
                if let Some(max_length) = self.max_length {
                    if self.value_ref().len() >= max_length {
                        return;
                    }
                }

                self.input_wrapper
                    .value_ref_mut()
                    .insert_str(self.cursor_index, text);
                self.cursor_index += 1;

                self.calc_cursor_postion();

                need_stop_timer = true;

                emit!(self.value_changed());
            }
        }
        self.update();

        if need_stop_timer {
            if self.blink_timer.is_active() {
                self.blink_timer.stop();
            }
            self.cursor_visible = true;
        }
    }

    fn on_key_released(&mut self, _: &KeyEvent) {
        if !self.blink_timer.is_active() {
            self.blink_timer.start(Duration::from_millis(
                application::cursor_blinking_time() as u64
            ))
        }
    }

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

        self.text_padding = text_padding;

        if self.window().initialized() {
            self.calc_text_geometry();
            self.update();
        }
    }

    #[inline]
    pub fn set_blink_cursor(&mut self, cursor_blink: bool) {
        if self.cursor_blink == cursor_blink {
            return;
        }

        self.check_blink_timer(self.is_focus());
    }

    #[inline]
    pub fn set_letter_spacing(&mut self, letter_spacing: f32) {
        self.letter_spacing = letter_spacing;

        self.update();
    }

    #[inline]
    pub fn set_text_color(&mut self, text_color: Color) {
        self.text_color = text_color;

        self.update();
    }

    #[inline]
    pub fn set_max_length(&mut self, max_length: usize) {
        self.max_length = Some(max_length)
    }
}

impl Text {
    fn draw_enable(&mut self, painter: &mut Painter) {
        // Draw text:
        self.draw_text(painter);

        // Draw cursor:
        self.draw_cursor(painter);
    }

    fn draw_disable(&mut self, painter: &mut Painter) {}

    fn draw_text(&self, painter: &mut Painter) {
        let val_ref = self.value_ref();

        painter.fill_rect_global(self.text_window, self.background());

        painter.set_color(self.text_color);
        painter.draw_paragraph_global(
            val_ref.as_str(),
            *self.text_draw_position(),
            self.letter_spacing,
            f32::MAX,
            None,
            false,
        );
    }

    fn draw_cursor(&self, painter: &mut Painter) {
        if !self.cursor_visible {
            return;
        }

        painter.set_color(self.text_color);
        painter.set_line_width(1.);
        painter.draw_line_f_global(
            self.cursor_position,
            self.text_window.top(),
            self.cursor_position,
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
        if let Some(ref mut pos) = self.text_draw_position {
            pos.set_y(window.y());
        } else {
            self.text_draw_position = Some(self.text_window.top_left());
        };

        self.calc_cursor_postion();
    }

    #[inline]
    fn calc_widget_height(&self) -> f32 {
        (self.font_dimension.1 + 2. * self.text_padding).ceil()
    }

    #[inline]
    fn calc_cursor_postion(&mut self) {
        let mut pos = {
            let str_ref = self.value_ref();
            let str = str_ref.as_str();

            self.text_draw_position().x()
                + self
                    .skia_font()
                    .calc_text_dimension(&str[0..self.cursor_index], self.letter_spacing)
                    .0
        };

        if pos > self.text_window.right() - 1. {
            let offset = pos - self.text_window.right() + 1.;
            self.text_draw_position_mut().offset(-offset, 0.);

            pos = self.text_window.right() - 1.;
        }

        if pos < self.text_window.left() {
            let offset = self.text_window.left() - pos;
            self.text_draw_position_mut().offset(offset, 0.);

            pos = self.text_window.left();
        }

        self.cursor_position = pos;
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

    #[inline]
    fn blink_event(&mut self) {
        self.cursor_visible = !self.cursor_visible;
        self.update();
    }

    #[inline]
    fn text_draw_position(&self) -> &FPoint {
        self.text_draw_position
            .as_ref()
            .expect("Fatal error: `text_draw_position` of `Text` was None.")
    }

    #[inline]
    fn text_draw_position_mut(&mut self) -> &mut FPoint {
        self.text_draw_position
            .as_mut()
            .expect("Fatal error: `text_draw_position` of `Text` was None.")
    }

    #[inline]
    fn skia_font(&self) -> &SkiaFont {
        self.skia_font
            .as_ref()
            .expect("Fatal error: `skia_font` of `Text` was None.")
    }
}
