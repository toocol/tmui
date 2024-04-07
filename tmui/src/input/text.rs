use super::{Input, InputSignals, InputWrapper};
use crate::{
    application,
    prelude::*,
    shortcut::ShortcutRegister,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{widget_inner::WidgetInnerExt, WidgetImpl},
};
use log::warn;
use std::time::Duration;
use tlib::{
    connect,
    events::{KeyEvent, MouseEvent},
    figure::FontCalculation,
    global_watch,
    namespace::KeyCode,
    run_after, shortcut,
    skia_safe::ClipOp,
    timer::Timer,
    typedef::SkiaFont,
};

const TEXT_DEFAULT_WIDTH: i32 = 150;
const TEXT_DEFAULT_PADDING: f32 = 3.;

const TEXT_DEFAULT_BORDER_COLOR: Color = Color::grey_with(150);

const TEXT_DEFAULT_DISABLE_COLOR: Color = Color::grey_with(80);
const TEXT_DEFAULT_DISABLE_BACKGROUND: Color = Color::grey_with(240);

const TEXT_DEFAULT_PLACEHOLDER_COLOR: Color = Color::grey_with(130);

#[extends(Widget)]
#[run_after]
#[popupable]
#[global_watch(MouseMove)]
pub struct Text {
    input_wrapper: InputWrapper<String>,

    //////////////////////////// Cursor
    cursor_index: usize,
    cursor_visible: bool,
    #[derivative(Default(value = "true"))]
    cursor_blink: bool,
    blink_timer: Timer,
    /// The cursor color, if not set, defaults to the text color.
    caret_color: Option<Color>,

    //////////////////////////// Text
    #[derivative(Default(value = "TEXT_DEFAULT_PADDING"))]
    text_padding: f32,
    text_window: FRect,
    text_draw_position: Option<FPoint>,
    #[derivative(Default(value = "Color::BLACK"))]
    text_color: Color,
    max_length: Option<usize>,
    letter_spacing: f32,
    placeholder: String,

    //////////////////////////// Font
    font_dimension: (f32, f32),
    skia_font: Option<SkiaFont>,

    //////////////////////////// Selection
    drag_status: DragStatus,
    #[derivative(Default(value = "-1"))]
    selection_start: i32,
    #[derivative(Default(value = "-1"))]
    selection_end: i32,
    #[derivative(Default(value = "Color::from_rgb(51, 167, 255)"))]
    selection_background: Color,
    #[derivative(Default(value = "Color::WHITE"))]
    selection_color: Color,
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum DragStatus {
    #[default]
    None,
    Pending,
    Dragging,
}

impl InputSignals for Text {}

impl ObjectSubclass for Text {
    const NAME: &'static str = "Text";
}

impl ObjectImpl for Text {
    fn initialize(&mut self) {
        self.input_wrapper.init(self.id());
        self.font_changed();
        self.set_border_color(TEXT_DEFAULT_BORDER_COLOR);
        self.set_borders(1., 1., 1., 1.);
        self.register_shortcuts();

        if self.is_enable() {
            self.cursor_index = self.input_wrapper.value_ref().len();
        }

        connect!(self.blink_timer, timeout(), self, blink_event());
        connect!(
            self,
            geometry_changed(),
            self,
            handle_geometry_changed(Rect)
        );
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

    #[inline]
    fn paint(&mut self, painter: &mut Painter) {
        if self.is_enable() {
            painter.save();
            painter.clip_rect_global(self.text_window, ClipOp::Intersect);

            self.draw_enable(painter);

            painter.restore();
        } else {
            self.draw_disable(painter);
        }
    }

    #[inline]
    fn font_changed(&mut self) {
        self.handle_font_changed();
    }

    #[inline]
    fn on_get_focus(&mut self) {
        if !self.is_enable() {
            return;
        }

        self.check_blink_timer(true);
        self.update();
    }

    #[inline]
    fn on_lose_focus(&mut self) {
        if !self.is_enable() {
            return;
        }

        self.check_blink_timer(false);
        self.update();
    }

    #[inline]
    fn on_key_pressed(&mut self, event: &KeyEvent) {
        if !self.is_enable() {
            return;
        }

        self.handle_key_pressed(event);

        self.update();
    }

    #[inline]
    fn on_key_released(&mut self, _: &KeyEvent) {
        if !self.is_enable() {
            return;
        }

        self.start_blink_timer();
    }

    #[inline]
    fn on_mouse_pressed(&mut self, event: &MouseEvent) {
        if !self.is_enable() {
            return;
        }

        match event.n_press() {
            1 => self.handle_mouse_click(event),
            2 => self.handle_mouse_double_click(),
            _ => {}
        }
    }

    #[inline]
    fn on_mouse_released(&mut self, _: &MouseEvent) {
        if !self.is_enable() {
            return;
        }

        self.handle_mouse_release()
    }
}

impl GlobalWatchImpl for Text {
    #[inline]
    fn on_global_mouse_move(&mut self, evt: &MouseEvent) {
        if !self.is_enable() {
            return;
        }

        self.handle_mouse_move(evt)
    }
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

    #[inline]
    pub fn set_caret_color(&mut self, caret_color: Color) {
        self.caret_color = Some(caret_color);

        self.update();
    }

    #[inline]
    pub fn set_placeholder<Placeholder: ToString>(&mut self, placeholder: Placeholder) {
        self.placeholder = placeholder.to_string();

        self.update();
    }
}

impl Text {
    fn draw_enable(&mut self, painter: &mut Painter) {
        // Calculates the position of cursor, and adjust the `text_draw_position`:
        let cursor_x = self.sync_cursor_text_draw();

        // Clear the text window:
        painter.fill_rect_global(self.text_window, self.background());

        // Draw text:
        self.draw_text(painter);

        // Draw cursor:
        self.draw_cursor(painter, cursor_x);
    }

    fn draw_disable(&mut self, painter: &mut Painter) {
        let rect = self.borderless_rect();
        painter.fill_rect_global(rect, TEXT_DEFAULT_DISABLE_BACKGROUND);

        self.draw_text(painter);
    }

    fn draw_text(&self, painter: &mut Painter) {
        let val_ref = self.value_ref();

        // Draw placeholder:
        if val_ref.len() == 0 {
            if self.placeholder.len() > 0 {
                self.draw_text_placeholder(painter);
            }

            return;
        }

        // Draw normal:
        if self.selection_start == -1
            || self.selection_end == -1
            || self.selection_start == self.selection_end
        {
            self.draw_text_normal(painter, &val_ref)
        }
        // Draw selected:
        else {
            self.draw_text_selected(painter, &val_ref)
        }
    }

    fn draw_text_normal(&self, painter: &mut Painter, val_ref: &Ref<String>) {
        if self.is_enable() {
            painter.set_color(self.text_color);
        } else {
            painter.set_color(TEXT_DEFAULT_DISABLE_COLOR);
        }

        painter.draw_paragraph_global(
            val_ref.as_str(),
            *self.text_draw_position(),
            self.letter_spacing,
            f32::MAX,
            None,
            false,
        );
    }

    fn draw_text_selected(&self, painter: &mut Painter, val_ref: &Ref<String>) {
        let str = val_ref.as_str();

        /*
         * Calculate the substring and the positions of both the selected and unselected text areas.
         *
         * pre(unselected) - mid(selected) - suf(unselected)
         */
        let (pre, mid, suf) = {
            let font = self.skia_font();
            let (start, end) = (
                self.selection_start.min(self.selection_end) as usize,
                self.selection_end.max(self.selection_start) as usize,
            );

            let pre_str = &str[0..start];
            let mid_str = &str[start..end];
            let suf_str = &str[end..];

            let pre_w = font.calc_text_dimension(pre_str, self.letter_spacing).0;
            let mid_w = font.calc_text_dimension(mid_str, self.letter_spacing).0;

            let pre_point = *self.text_draw_position();
            let mut mid_point = pre_point;
            mid_point.offset(pre_w, 0.);
            let mut suf_point = mid_point;
            suf_point.offset(mid_w, 0.);

            (
                (pre_str, pre_point),
                (
                    mid_str,
                    mid_point,
                    FRect::new(mid_point.x(), mid_point.y(), mid_w, self.font_dimension.1),
                ),
                (suf_str, suf_point),
            )
        };

        if pre.0.len() > 0 {
            painter.set_color(self.text_color);
            painter.draw_paragraph_global(pre.0, pre.1, self.letter_spacing, f32::MAX, None, false);
        }
        if suf.0.len() > 0 {
            painter.set_color(self.text_color);
            painter.draw_paragraph_global(suf.0, suf.1, self.letter_spacing, f32::MAX, None, false);
        }
        if mid.0.len() > 0 {
            painter.fill_rect_global(mid.2, self.selection_background);

            painter.set_color(self.selection_color);
            painter.draw_paragraph_global(mid.0, mid.1, self.letter_spacing, f32::MAX, None, false);
        }
    }

    fn draw_text_placeholder(&self, painter: &mut Painter) {
        painter.set_color(TEXT_DEFAULT_PLACEHOLDER_COLOR);

        painter.draw_paragraph_global(
            &self.placeholder,
            *self.text_draw_position(),
            self.letter_spacing,
            f32::MAX,
            None,
            false,
        );
    }

    fn draw_cursor(&self, painter: &mut Painter, cursor_x: f32) {
        if !self.cursor_visible || self.drag_status == DragStatus::Dragging {
            return;
        }

        if let Some(color) = self.caret_color {
            painter.set_color(color);
        } else {
            painter.set_color(self.text_color);
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
        if let Some(ref mut pos) = self.text_draw_position {
            pos.set_y(window.y());
        } else {
            self.text_draw_position = Some(self.text_window.top_left());
        };
    }

    fn sync_cursor_text_draw(&mut self) -> f32 {
        let mut pos = {
            let str_ref = self.value_ref();
            let str = str_ref.as_str();

            self.text_draw_position().x()
                + self
                    .skia_font()
                    .calc_text_dimension(&str[0..self.cursor_index], self.letter_spacing)
                    .0
        };

        // When cursor position exceeds the right side of the text window,
        // set the cursor to the end of the text window,
        // meanwhile the drawing position moves left.
        if pos > self.text_window.right() {
            let offset = pos - self.text_window.right() + 2.;
            self.text_draw_position_mut().offset(-offset, 0.);

            pos = self.text_window.right() - 2.;
        }
        // When cursor position exceeds the left side of the text window,
        // set the cursor to the beginning of the text window,
        // meanwhile the drawing position moves right.
        else if pos < self.text_window.left() {
            let offset = self.text_window.left() + 2. - pos;
            self.text_draw_position_mut().offset(offset, 0.);

            pos = self.text_window.left() + 2.;
        }
        // When a character from the input string has been deleted,
        // and the cursor was at the end of both the input string and the text window,
        // the drawing position moves right.
        else if self.cursor_index == self.value_ref().len()
            && self.text_draw_position().x() < self.text_window.x()
        {
            let offset = self.text_window.right() - 2. - pos;
            self.text_draw_position_mut().offset(offset, 0.);

            pos = self.text_window.right() - 2.;
        }

        pos
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

    fn handle_font_changed(&mut self) {
        let font = self.font().to_skia_font();
        self.font_dimension = font.calc_font_dimension();

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

    fn handle_key_pressed(&mut self, event: &KeyEvent) {
        match event.key_code() {
            KeyCode::KeyBackspace => {
                if self.cursor_index == 0 {
                    self.start_blink_timer();
                    return;
                }
                self.cursor_index -= 1;
                self.input_wrapper.value_ref_mut().remove(self.cursor_index);

                emit!(self.value_changed());
            }
            KeyCode::KeyLeft => {
                if self.cursor_index > 0 {
                    self.cursor_index -= 1;
                } else {
                    self.start_blink_timer();
                    return;
                }
            }
            KeyCode::KeyRight => {
                if self.cursor_index < self.value_ref().len() {
                    self.cursor_index += 1;
                } else {
                    self.start_blink_timer();
                    return;
                }
            }
            KeyCode::KeyEnd => {
                let idx = self.value_ref().len();
                self.cursor_index = idx;
            }
            KeyCode::KeyHome => {
                self.cursor_index = 0;
            }
            _ => {
                let text = event.text();
                if text.is_empty() {
                    return;
                }
                if let Some(max_length) = self.max_length {
                    if self.value_ref().len() >= max_length {
                        self.start_blink_timer();
                        return;
                    }
                }

                self.input_wrapper
                    .value_ref_mut()
                    .insert_str(self.cursor_index, text);
                self.cursor_index += 1;

                emit!(self.value_changed());
            }
        }

        if self.blink_timer.is_active() {
            self.blink_timer.stop();
        }
        self.cursor_visible = true;
    }

    fn register_shortcuts(&mut self) {
        self.register_shortcut(shortcut!(Control + A), |w| {
            w.downcast_mut::<Text>().unwrap().select_all()
        });

        self.register_shortcut(shortcut!(Control + C), |w| {
        });

        self.register_shortcut(shortcut!(Control + V), |w| {
        });
    }

    fn handle_mouse_click(&mut self, event: &MouseEvent) {
        self.drag_status = DragStatus::Pending;

        let pos: FPoint = event.position().into();
        let pos = self.map_to_global_f(&pos);

        self.cursor_index = self.calc_cursor_index(pos.x());
        if self.selection_end != -1 {
            self.selection_start = -1;
            self.selection_end = -1;
        } else {
            self.selection_start = self.cursor_index as i32;
        }

        self.update();
        self.cursor_visible = true;
        if self.blink_timer.is_active() {
            self.blink_timer.stop();
        }
    }

    fn handle_mouse_move(&mut self, event: &MouseEvent) {
        if self.drag_status == DragStatus::None {
            return;
        }
        if self.selection_start == -1 {
            return;
        }
        self.drag_status = DragStatus::Dragging;

        let pos: FPoint = event.position().into();
        self.cursor_index = self.calc_cursor_index(pos.x());
        self.selection_end = self.cursor_index as i32;

        self.update();
    }

    /// Calculate `cursor_index`(the index of the corresponding character in the text)
    /// based on the given x-coordinate.
    fn calc_cursor_index(&self, x_pos: f32) -> usize {
        let str_ref = self.value_ref();
        let str = str_ref.as_str();

        let offset = x_pos - self.text_draw_position().x();
        let mut predict_idx = ((offset / self.font_dimension.0) as usize).min(str.len());

        let mut last_diff = f32::MAX;
        let mut last_idx = predict_idx;

        loop {
            let actual_len = self
                .skia_font()
                .calc_text_dimension(&str[0..predict_idx], self.letter_spacing)
                .0;

            let diff = (offset - actual_len).abs();
            if diff >= last_diff {
                predict_idx = last_idx;
                break;
            }
            if diff < self.font_dimension.0 * 0.3 {
                break;
            }

            last_diff = diff;
            last_idx = predict_idx;

            if offset > actual_len {
                if predict_idx == str_ref.len() {
                    break;
                }
                predict_idx += 1;
            } else {
                if predict_idx == 0 {
                    break;
                }
                predict_idx -= 1;
            }
        }
        predict_idx
    }

    #[inline]
    fn handle_mouse_double_click(&mut self) {
        self.select_all();
    }

    #[inline]
    fn select_all(&mut self) {
        let len = self.value_ref().len() as i32;
        self.selection_start = 0;
        self.selection_end = len;
        self.update();
    }

    #[inline]
    fn handle_mouse_release(&mut self) {
        self.drag_status = DragStatus::None;

        self.start_blink_timer();
    }

    #[inline]
    fn handle_geometry_changed(&mut self, rect: Rect) {
        if rect.width() == 0 || rect.height() == 0 {
            return;
        }

        self.calc_text_geometry();
    }

    #[inline]
    fn start_blink_timer(&mut self) {
        if !self.blink_timer.is_active() {
            self.blink_timer.start(Duration::from_millis(
                application::cursor_blinking_time() as u64
            ))
        }
    }

    #[inline]
    fn calc_widget_height(&self) -> f32 {
        (self.font_dimension.1 + 2. * self.text_padding).ceil()
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
