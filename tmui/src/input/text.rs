use super::{Input, InputEle, InputSignals, InputWrapper, ReflectInputEle, INPUT_FOCUSED_BORDER_COLOR};
use crate::{
    application, cast_do, clipboard::ClipboardLevel, font::{FontCalculation, SkiaParagraphExt}, input::{INPUT_DEFAULT_BORDER_COLOR, INPUT_DEFAULT_BORDER_RADIUS}, input_ele_impl, prelude::*, shortcut::ShortcutRegister, system::System, tlib::object::{ObjectImpl, ObjectSubclass}, widget::{widget_inner::WidgetInnerExt, RegionClear, WidgetImpl}
};
use log::warn;
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};
use tlib::{
    connect,
    events::{KeyEvent, MouseEvent},
    global_watch,
    namespace::{KeyCode, KeyboardModifier},
    run_after, shortcut, signals,
    skia_safe::ClipOp,
    timer::Timer,
};

pub(crate) const TEXT_DEFAULT_MAX_MEMORIES_SIZE: usize = 50;
pub(crate) const TEXT_DEFAULT_WIDTH: i32 = 150;
pub(crate) const TEXT_DEFAULT_PADDING: f32 = 4.;

pub(crate) const TEXT_DEFAULT_DISABLE_COLOR: Color = Color::grey_with(80);
pub(crate) const TEXT_DEFAULT_DISABLE_BACKGROUND: Color = Color::grey_with(240);

pub(crate) const TEXT_DEFAULT_PLACEHOLDER_COLOR: Color = Color::grey_with(130);

type FnRequireInvalidRender = Box<dyn Fn(&mut Painter, FRect)>;
type FnTextWindowCalc = Box<dyn Fn(&TextProps, FRect) -> FRect>;

#[extends(Widget)]
#[run_after]
#[popupable]
#[global_watch(MouseMove)]
pub struct Text {
    input_wrapper: InputWrapper<String>,
    props: TextProps,
}

#[derive(Derivative)]
#[derivative(Default)]
pub(crate) struct TextProps {
    pub(crate) entered: bool,

    //////////////////////////// Revoke/Redo
    pub(crate) revoke_memories: VecDeque<TextMemory>,
    pub(crate) redo_memories: VecDeque<TextMemory>,
    #[derivative(Default(value = "Instant::now()"))]
    pub(crate) last_key_strike: Instant,

    //////////////////////////// Cursor
    pub(crate) cursor_index: usize,
    pub(crate) cursor_visible: bool,
    #[derivative(Default(value = "true"))]
    pub(crate) cursor_blink: bool,
    pub(crate) blink_timer: Timer,
    /// The cursor color, if not set, defaults to the text color.
    pub(crate) caret_color: Option<Color>,
    pub(crate) predict_start: usize,
    pub(crate) predict_start_len: f32,

    //////////////////////////// Text
    #[derivative(Default(value = "TEXT_DEFAULT_PADDING"))]
    pub(crate) text_padding: f32,
    pub(crate) text_window: FRect,
    pub(crate) text_draw_position: Option<FPoint>,
    #[derivative(Default(value = "Color::BLACK"))]
    pub(crate) text_color: Color,
    pub(crate) max_length: Option<usize>,
    pub(crate) letter_spacing: f32,
    pub(crate) placeholder: String,
    pub(crate) unicode_text: bool,
    pub(crate) require_invalid: bool,
    pub(crate) customize_require_invalid_render: Option<FnRequireInvalidRender>,
    pub(crate) require_invalid_border_color: Option<Color>,
    pub(crate) require_invalid_focused_border_color: Option<Color>,

    //////////////////////////// Font
    pub(crate) font_dimension: (f32, f32),

    //////////////////////////// Selection
    pub(crate) drag_status: DragStatus,
    #[derivative(Default(value = "-1"))]
    pub(crate) selection_start: i32,
    #[derivative(Default(value = "-1"))]
    pub(crate) selection_end: i32,
    #[derivative(Default(value = "Color::rgb(51, 167, 255)"))]
    pub(crate) selection_background: Color,
    #[derivative(Default(value = "Color::WHITE"))]
    pub(crate) selection_color: Color,

    pub(crate) fn_calc_text_window: Option<FnTextWindowCalc>,
}
impl TextProps {
    #[inline]
    pub(crate) fn calc_widget_height(&self) -> f32 {
        (self.font_dimension.1 + 2. * self.text_padding).ceil()
    }
}

pub(crate) trait TextPropsAcquire: Input<Value = String> {
    fn props(&self) -> &TextProps;

    fn props_mut(&mut self) -> &mut TextProps;

    fn shown_text(&self) -> Ref<String>;
}
impl TextPropsAcquire for Text {
    #[inline]
    fn props(&self) -> &TextProps {
        &self.props
    }

    #[inline]
    fn props_mut(&mut self) -> &mut TextProps {
        &mut self.props
    }

    #[inline]
    fn shown_text(&self) -> Ref<String> {
        self.input_wrapper.value_ref()
    }
}

pub trait TextSignals: ActionExt {
    signals! {
        TextSignals:

        /// Emit when text's selection was changed.
        selection_changed();
    }
}
impl TextSignals for Text {}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DragStatus {
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
    #[inline]
    fn construct(&mut self) {
        self.parent_construct();

        self.input_wrapper.init(self.id());
        self.register_shortcuts();
        self.text_construct();
    }

    #[inline]
    fn type_register(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<Self, ReflectInputEle>()
    }
}

impl WidgetImpl for Text {
    #[inline]
    fn run_after(&mut self) {
        self.calc_text_geometry();

        self.on_value_changed();
    }

    #[inline]
    fn enable_focus(&self) -> bool {
        true
    }

    #[inline]
    fn paint(&mut self, painter: &mut Painter) {
        if self.is_enable() {
            painter.save();
            painter.clip_rect_global(self.props.text_window, ClipOp::Intersect);

            self.draw_enable(painter);

            painter.restore();
        } else {
            self.draw_disable(painter);
        }

        self.draw_require_invalid(painter);
    }

    #[inline]
    fn font_changed(&mut self) {
        self.handle_font_changed();

        self.calc_text_geometry();
    }

    #[inline]
    fn on_get_focus(&mut self) {
        self.handle_get_focus()
    }

    #[inline]
    fn on_lose_focus(&mut self) {
        self.handle_lose_focus()
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
        if !self.props.entered {
            self.window()
                .set_cursor_shape(SystemCursorShape::ArrowCursor);
        }

        self.handle_mouse_release()
    }

    #[inline]
    fn on_mouse_enter(&mut self, _: &MouseEvent) {
        self.props.entered = true;
        self.window()
            .set_cursor_shape(SystemCursorShape::TextCursor);
    }

    #[inline]
    fn on_mouse_leave(&mut self, _: &MouseEvent) {
        self.props.entered = false;
        let window = self.window();
        if self.id() == window.pressed_widget() {
            return;
        }
        window.set_cursor_shape(SystemCursorShape::ArrowCursor);
    }
}

impl GlobalWatchImpl for Text {
    #[inline]
    fn on_global_mouse_move(&mut self, evt: &MouseEvent) -> bool {
        if !self.is_enable() {
            return false;
        }

        self.handle_mouse_move(evt);

        false
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

    #[inline]
    fn required_handle(&mut self) -> bool {
        self.inner_required_handle()
    }

    #[inline]
    fn check_value(&mut self, val: &Self::Value) -> bool {
        let len = val.chars().count();
        self.props_mut().cursor_index = len;
        true
    }
}

#[allow(private_bounds)]
pub trait TextExt: TextPropsAcquire + WidgetImpl + TextInnerExt {
    #[inline]
    fn set_text_padding(&mut self, text_padding: f32) {
        if text_padding < 0.
            || text_padding * 2. + self.props().font_dimension.1 > self.size().height() as f32
        {
            warn!("The value of text padding was invalid, ignore padding set.");
            return;
        }

        self.props_mut().text_padding = text_padding;

        if self.window().initialized() {
            self.calc_text_geometry();
            self.update();
        }
    }

    #[inline]
    fn set_blink_cursor(&mut self, cursor_blink: bool) {
        if self.props().cursor_blink == cursor_blink {
            return;
        }

        self.check_blink_timer(self.is_focus());
    }

    #[inline]
    fn set_letter_spacing(&mut self, letter_spacing: f32) {
        self.props_mut().letter_spacing = letter_spacing;

        self.update();
    }

    #[inline]
    fn set_text_color(&mut self, text_color: Color) {
        self.props_mut().text_color = text_color;

        self.update();
    }

    #[inline]
    fn set_max_length(&mut self, max_length: usize) {
        self.props_mut().max_length = Some(max_length)
    }

    #[inline]
    fn set_caret_color(&mut self, caret_color: Color) {
        self.props_mut().caret_color = Some(caret_color);

        self.update();
    }

    #[inline]
    fn set_placeholder<Placeholder: ToString>(&mut self, placeholder: Placeholder) {
        self.props_mut().placeholder = placeholder.to_string();

        self.update();
    }

    #[inline]
    fn get_selection(&self) -> Option<String> {
        if self.has_selection() {
            let (start, end) = self.selection_range();
            let (start, end) = (self.map(start), self.map(end));

            let str_ref = self.value_ref();
            Some(str_ref.as_str()[start..end].to_string())
        } else {
            None
        }
    }

    #[inline]
    fn copy(&self) {
        if let Some(selection) = self.get_selection() {
            System::clipboard().set_text(selection, ClipboardLevel::Os)
        }
    }

    #[inline]
    fn cut(&mut self) {
        if !self.has_selection() {
            return;
        }
        self.save_revoke();

        let (start, end) = self.selection_range();
        {
            let (start, end) = (self.map(start), self.map(end));
            System::clipboard().set_text(&self.value_ref()[start..end], ClipboardLevel::Os);
        }
        self.value_remove_range(start, end);

        self.props_mut().cursor_index = start;
        self.clear_selection();
    }

    #[inline]
    fn paste(&mut self) {
        if let Some(cp) = System::clipboard().text(ClipboardLevel::Os) {
            if !self.check_value(&cp) {
                return;
            }
            self.save_revoke();

            if self.has_selection() {
                let (start, end) = self.selection_range();
                self.value_remove_range(start, end);

                self.props_mut().cursor_index = start;
                self.clear_selection();
            }

            let mut cut = false;
            {
                let idx = self.map(self.props().cursor_index);
                self.input_wrapper().value_mut().insert_str(idx, &cp);

                if let Some(max_length) = self.props().max_length {
                    if self.value_chars_count() > max_length {
                        let idx = self.map(max_length);
                        self.input_wrapper().value_mut().replace_range(idx.., "");
                        cut = true;
                    }
                }
            }

            if cut {
                self.props_mut().cursor_index = self.value_chars_count();
            } else {
                self.props_mut().cursor_index += cp.chars().count();
            }

            if !cp.is_empty() {
                emit!(self.value_changed());
            }
            self.update();
        }
    }

    #[inline]
    fn revoke(&mut self) {
        if let Some(mem) = self.props_mut().revoke_memories.pop_back() {
            self.save_redo();

            self.props_mut().cursor_index = mem.cursor_index;
            self.props_mut().selection_start = mem.selection_start;
            self.props_mut().selection_end = mem.selection_end;
            self.props_mut().text_draw_position = Some(mem.text_draw_position);

            let mem_val = mem.value;
            if !mem_val.eq(self.value_ref().as_str()) {
                self.input_wrapper().set_value(mem_val);
                emit!(self.value_changed())
            }

            self.update();
        }
    }

    #[inline]
    fn redo(&mut self) {
        if let Some(mem) = self.props_mut().redo_memories.pop_back() {
            self.save_revoke();

            self.props_mut().cursor_index = mem.cursor_index;
            self.props_mut().selection_start = mem.selection_start;
            self.props_mut().selection_end = mem.selection_end;
            self.props_mut().text_draw_position = Some(mem.text_draw_position);

            let mem_val = mem.value;
            if !mem_val.eq(self.value_ref().as_str()) {
                self.input_wrapper().set_value(mem_val);
                emit!(self.value_changed())
            }

            self.update();
        }
    }

    #[inline]
    fn shift_select(&mut self, code: KeyCode) {
        let start = if self.props().selection_start == -1 {
            Some(self.props().cursor_index as i32)
        } else {
            None
        };

        match code {
            KeyCode::KeyLeft => {
                if self.props().cursor_index > 0 {
                    self.props_mut().cursor_index -= 1;
                    self.adjust_selection_range(start, Some(self.props().cursor_index as i32))
                }
            }
            KeyCode::KeyRight => {
                if self.props().cursor_index < self.value_chars_count() {
                    self.props_mut().cursor_index += 1;
                    self.adjust_selection_range(start, Some(self.props().cursor_index as i32))
                }
            }
            KeyCode::KeyHome => {
                self.props_mut().cursor_index = 0;
                self.adjust_selection_range(start, Some(0));
            }
            KeyCode::KeyEnd => {
                self.props_mut().cursor_index = self.value_chars_count();
                self.adjust_selection_range(start, Some(self.props().cursor_index as i32));
            }
            _ => {}
        }
    }

    #[inline]
    fn clean(&mut self) {
        self.clear_selection();
        *self.input_wrapper().value_mut() = String::new();
        self.update();
    }

    #[inline]
    fn set_customize_require_invalid_render<F: Fn(&mut Painter, FRect) + 'static>(&mut self, f: F) {
        self.props_mut().customize_require_invalid_render = Some(Box::new(f))
    }

    #[inline]
    fn set_require_invalid_border_color(&mut self, color: Color) {
        if self.props().require_invalid && !self.is_focus() {
            self.set_border_color(color);
        }
        self.props_mut().require_invalid_border_color = Some(color);
    }

    #[inline]
    fn set_require_invalid_focused_border_color(&mut self, color: Color) {
        if self.props().require_invalid && self.is_focus() {
            self.set_border_color(color);
        }
        self.props_mut().require_invalid_focused_border_color = Some(color);
    }
}

pub(crate) trait TextShorcutRegister: TextExt {
    fn register_shortcuts(&mut self);
}
#[macro_export]
macro_rules! impl_text_shortcut_register {
    ($ty:ident) => {
        impl TextShorcutRegister for $ty {
            #[rustfmt::skip]
            fn register_shortcuts(&mut self) {
                self.register_shortcut(shortcut!(Control + A), cast_do!($ty::select_all()));

                self.register_shortcut(shortcut!(Control + C), cast_do!($ty::copy()));

                self.register_shortcut(shortcut!(Control + X), cast_do!($ty::cut()));

                self.register_shortcut(shortcut!(Control + V), cast_do!($ty::paste()));

                self.register_shortcut(shortcut!(Control + Z), cast_do!($ty::revoke()));

                self.register_shortcut(shortcut!(Control + Y), cast_do!($ty::redo()));

                self.register_shortcut(shortcut!(Shift + Left), cast_do!($ty::shift_select(KeyCode::KeyLeft)));

                self.register_shortcut(shortcut!(Shift + Right), cast_do!($ty::shift_select(KeyCode::KeyRight)));

                self.register_shortcut(shortcut!(Shift + Home), cast_do!($ty::shift_select(KeyCode::KeyHome)));

                self.register_shortcut(shortcut!(Shift + End), cast_do!($ty::shift_select(KeyCode::KeyEnd)));
            }
        }
    };
}
impl_text_shortcut_register!(Text);

pub(crate) trait TextInnerExt:
    TextPropsAcquire + WidgetImpl + WidgetInnerExt + TextSignals + Sized
{
    fn text_construct(&mut self) {
        self.handle_font_changed();
        self.set_border_color(INPUT_DEFAULT_BORDER_COLOR);
        self.set_border_radius(INPUT_DEFAULT_BORDER_RADIUS);
        self.set_borders(1., 1., 1., 1.);

        connect!(self, value_changed(), self, on_value_changed());
        connect!(self.props().blink_timer, timeout(), self, blink_event());
        connect!(
            self,
            geometry_changed(),
            self,
            handle_geometry_changed(FRect)
        );
    }

    fn draw_enable(&mut self, painter: &mut Painter) {
        // Calculates the position of cursor, and adjust the `text_draw_position`:
        let cursor_x = self.sync_cursor_text_draw();

        // Clear the text window:
        self.clear_global(painter, self.props().text_window);

        // Draw text:
        self.draw_text(painter);

        // Draw cursor:
        self.draw_cursor(painter, cursor_x);
    }

    fn draw_disable(&mut self, painter: &mut Painter) {
        let rect = self.borderless_rect_f();
        painter.fill_rect_global(rect, TEXT_DEFAULT_DISABLE_BACKGROUND);

        self.draw_text(painter);
    }

    fn draw_text(&self, painter: &mut Painter) {
        let val_ref = self.shown_text();

        // Draw placeholder:
        if val_ref.is_empty() {
            if !self.props().placeholder.is_empty() {
                self.draw_text_placeholder(painter);
            }

            return;
        }

        // Draw normal:
        if self.props().selection_start == -1
            || self.props().selection_end == -1
            || self.props().selection_start == self.props().selection_end
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
            painter.set_color(self.props().text_color);
        } else {
            painter.set_color(TEXT_DEFAULT_DISABLE_COLOR);
        }

        self.render_text(painter, val_ref.as_str(), *self.text_draw_position())
    }

    fn draw_text_selected(&self, painter: &mut Painter, val_ref: &Ref<String>) {
        let str = val_ref.as_str();

        /*
         * Calculate the substring and the positions of both the selected and unselected text areas.
         *
         * pre(unselected) - mid(selected) - suf(unselected)
         */
        let (pre, mid, suf) = {
            let font = self.font();
            let (start, end) = self.selection_range();
            let (start, end) = (self.map_shown(start), self.map_shown(end));

            let pre_str = &str[0..start];
            let mid_str = &str[start..end];
            let suf_str = &str[end..];

            let pre_w = font
                .calc_text_dimension(pre_str, self.props().letter_spacing)
                .0;
            let mid_w = font
                .calc_text_dimension(mid_str, self.props().letter_spacing)
                .0;

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
                    FRect::new(
                        mid_point.x(),
                        mid_point.y(),
                        mid_w,
                        self.props().font_dimension.1,
                    ),
                ),
                (suf_str, suf_point),
            )
        };

        if !pre.0.is_empty() {
            painter.set_color(self.props().text_color);
            self.render_text(painter, pre.0, pre.1);
        }
        if !suf.0.is_empty() {
            painter.set_color(self.props().text_color);
            self.render_text(painter, suf.0, suf.1);
        }
        if !mid.0.is_empty() {
            painter.fill_rect_global(mid.2, self.props().selection_background);

            painter.set_color(self.props().selection_color);
            self.render_text(painter, mid.0, mid.1);
        }
    }

    fn draw_text_placeholder(&self, painter: &mut Painter) {
        painter.set_color(TEXT_DEFAULT_PLACEHOLDER_COLOR);

        self.render_text(
            painter,
            &self.props().placeholder,
            *self.text_draw_position(),
        );
    }

    fn draw_cursor(&self, painter: &mut Painter, cursor_x: f32) {
        if !self.props().cursor_visible || self.props().drag_status == DragStatus::Dragging {
            return;
        }
        if self.has_selection() {
            return;
        }

        if let Some(color) = self.props().caret_color {
            painter.set_color(color);
        } else {
            painter.set_color(self.props().text_color);
        }
        painter.set_line_width(1.);
        painter.draw_line_f_global(
            cursor_x,
            self.props().text_window.top(),
            cursor_x,
            self.props().text_window.bottom(),
        )
    }

    fn draw_require_invalid(&mut self, painter: &mut Painter) {
        if self.props().customize_require_invalid_render.is_some() {
            painter.save();
            painter.clip_rect_global(self.props().text_window, ClipOp::Difference);

            self.clear_global(painter, self.borderless_rect_f());

            painter.restore();
        }

        if !self.props().require_invalid {
            return;
        }

        if let Some(ref customize_require_invalid_render) =
            self.props().customize_require_invalid_render
        {
            painter.save();
            painter.save_pen();
            painter.clip_rect_global(self.props().text_window, ClipOp::Difference);
            painter.clip_rect_global(self.borderless_rect_f(), ClipOp::Intersect);

            customize_require_invalid_render(painter, self.borderless_rect_f());

            painter.restore_pen();
            painter.restore();
        }
    }

    fn calc_text_geometry(&mut self) {
        let rect: FRect = self.rect().into();

        self.props_mut().text_window =
            if let Some(ref fn_calc_text_window) = self.props().fn_calc_text_window {
                fn_calc_text_window(self.props(), rect)
            } else {
                let font_height = self.props().font_dimension.1;
                let calced_height = self.calc_widget_height();
                let mut window = FRect::default();

                window.set_x(rect.x() + self.props().text_padding);
                if calced_height == rect.height() {
                    window.set_y(rect.y() + self.props().text_padding);
                } else {
                    let offset = (rect.height() - font_height) / 2.;
                    window.set_y(rect.y() + offset.floor());
                }

                window.set_width(rect.width() - 2. * self.props().text_padding);
                window.set_height(font_height);

                window
            };

        let window = self.props().text_window;
        if let Some(ref mut pos) = self.props_mut().text_draw_position {
            pos.set_y(window.y());
        } else {
            self.props_mut().text_draw_position = Some(window.top_left());
        };
    }

    fn sync_cursor_text_draw(&mut self) -> f32 {
        let mut pos = {
            let str_ref = self.shown_text();
            let str = str_ref.as_str();

            let s = &str[..self.map_shown(self.props().cursor_index)];
            self.text_draw_position().x()
                + self
                    .font()
                    .calc_text_dimension(s, self.props().letter_spacing)
                    .0
        };

        let mut shift = false;

        // When cursor position exceeds the right side of the text window,
        // set the cursor to the end of the text window,
        // meanwhile the drawing position moves left.
        if pos > self.props().text_window.right() {
            let offset = pos - self.props().text_window.right() + 2.;
            self.text_draw_position_mut().offset(-offset, 0.);

            pos = self.props().text_window.right() - 2.;
            if offset != 0. {
                shift = true;
            }
        }
        // When cursor position exceeds the left side of the text window,
        // set the cursor to the beginning of the text window,
        // meanwhile the drawing position moves right.
        else if pos < self.props().text_window.left() {
            let offset = self.props().text_window.left() + 2. - pos;
            self.text_draw_position_mut().offset(offset, 0.);

            pos = self.props().text_window.left() + 2.;
            if offset != 0. {
                shift = true;
            }
        }
        // When a character from the input string has been deleted,
        // and the cursor was at the end of both the input string and the text window,
        // the drawing position moves right.
        else if self.props().cursor_index == self.value_chars_count()
            && self.text_draw_position().x() < self.props().text_window.x()
        {
            let offset = self.props().text_window.right() - 2. - pos;
            self.text_draw_position_mut().offset(offset, 0.);

            pos = self.props().text_window.right() - 2.;
            if offset != 0. {
                shift = true;
            }
        }

        if shift {
            self.calc_predict_index()
        }

        pos
    }

    #[inline]
    fn has_selection(&self) -> bool {
        self.props().selection_start != -1
            && self.props().selection_end != -1
            && self.props().selection_start != self.props().selection_end
    }

    fn check_blink_timer(&mut self, is_focus: bool) {
        if self.props().cursor_blink && is_focus {
            if self.props().blink_timer.is_active() {
                return;
            }
            self.props_mut().blink_timer.start(Duration::from_millis(
                application::cursor_blinking_time() as u64,
            ));

            if !self.props().cursor_visible {
                self.props_mut().cursor_visible = true;
                self.update();
            }
        } else {
            if !self.props().blink_timer.is_active() {
                return;
            }
            self.props_mut().blink_timer.stop();

            if self.props().cursor_visible {
                self.props_mut().cursor_visible = false;
                self.update();
            }
        }
    }

    fn handle_font_changed(&mut self) {
        self.props_mut().font_dimension = if self.props().unicode_text {
            self.font().calc_font_dimension_unicode()
        } else {
            self.font().calc_font_dimension()
        };

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

    fn handle_key_pressed(&mut self, event: &KeyEvent) {
        if event.modifier() == KeyboardModifier::NoModifier {
            if self.props().last_key_strike.elapsed().as_millis() > 1000 {
                self.save_revoke();
            }
            self.props_mut().last_key_strike = Instant::now();
        }

        match event.key_code() {
            KeyCode::KeyBackspace => {
                if self.has_selection() {
                    let (start, end) = self.selection_range();
                    self.value_remove_range(start, end);

                    self.props_mut().cursor_index = start;
                    self.clear_selection();
                    return;
                }

                if self.props().cursor_index == 0 {
                    self.start_blink_timer();
                    return;
                }

                self.props_mut().cursor_index -= 1;
                let idx = self.map(self.props().cursor_index);
                self.input_wrapper().value_mut().remove(idx);

                emit!(self.value_changed());
            }
            KeyCode::KeyLeft => {
                self.clear_selection();
                if self.props().cursor_index > 0 {
                    self.props_mut().cursor_index -= 1;
                } else {
                    self.start_blink_timer();
                    return;
                }
            }
            KeyCode::KeyRight => {
                self.clear_selection();
                if self.props().cursor_index < self.value_chars_count() {
                    self.props_mut().cursor_index += 1;
                } else {
                    self.start_blink_timer();
                    return;
                }
            }
            KeyCode::KeyEnd => {
                self.clear_selection();
                self.props_mut().cursor_index = self.value_chars_count();
            }
            KeyCode::KeyHome => {
                self.clear_selection();
                self.props_mut().cursor_index = 0;
            }
            _ => {
                let modifier = event.modifier();
                if modifier != KeyboardModifier::NoModifier
                    && modifier != KeyboardModifier::ShiftModifier
                {
                    return;
                }

                let text = event.text();
                if text.is_empty() {
                    return;
                }

                // Clear the selection.
                if self.has_selection() {
                    let (start, end) = self.selection_range();
                    self.value_remove_range(start, end);

                    self.clear_selection();
                    self.props_mut().cursor_index = start;
                }

                if let Some(max_length) = self.props().max_length {
                    if self.value_chars_count() >= max_length {
                        self.start_blink_timer();
                        return;
                    }
                }

                let idx = self.map(self.props().cursor_index);
                self.input_wrapper().value_mut().insert_str(idx, text);
                self.props_mut().cursor_index += 1;

                emit!(self.value_changed());
            }
        }

        if self.props().blink_timer.is_active() {
            self.props_mut().blink_timer.stop();
        }
        self.props_mut().cursor_visible = true;
    }

    fn handle_mouse_click(&mut self, event: &MouseEvent) {
        self.props_mut().drag_status = DragStatus::Pending;

        let pos: FPoint = event.position().into();
        let pos = self.map_to_global_f(&pos);

        self.props_mut().cursor_index = self.calc_cursor_index(pos.x());
        if self.props().selection_end != -1 {
            self.clear_selection();
        } else {
            self.adjust_selection_range(Some(self.props().cursor_index as i32), None);
        }

        self.update();
        self.props_mut().cursor_visible = true;
        if self.props().blink_timer.is_active() {
            self.props_mut().blink_timer.stop();
        }
    }

    fn handle_mouse_move(&mut self, event: &MouseEvent) {
        if self.props().drag_status == DragStatus::None {
            return;
        }
        if self.props().selection_start == -1 {
            return;
        }
        self.props_mut().drag_status = DragStatus::Dragging;

        let pos: FPoint = event.position().into();
        self.props_mut().cursor_index = self.calc_cursor_index(pos.x());

        self.adjust_selection_range(None, Some(self.props().cursor_index as i32));

        self.update();
    }

    #[inline]
    fn handle_get_focus(&mut self) {
        if !self.is_enable() {
            return;
        }

        self.check_blink_timer(true);
        if self.props().require_invalid {
            if let Some(invalid_focused_border_color) = self.props().require_invalid_focused_border_color {
                self.set_border_color(invalid_focused_border_color);
            } else {
                self.set_border_color(INPUT_FOCUSED_BORDER_COLOR);
            }
        } else {
            self.set_border_color(INPUT_FOCUSED_BORDER_COLOR);
        }
        self.set_borders(2., 2., 2., 2.);
    }

    #[inline]
    fn handle_lose_focus(&mut self) {
        if !self.is_enable() {
            return;
        }

        self.check_blink_timer(false);
        if self.props().require_invalid {
            if let Some(invalid_border_color) = self.props().require_invalid_border_color {
                self.set_border_color(invalid_border_color)
            } else {
                self.set_border_color(INPUT_DEFAULT_BORDER_COLOR);
            }
        } else {
            self.set_border_color(INPUT_DEFAULT_BORDER_COLOR);
        }
        self.set_borders(1., 1., 1., 1.);
    }

    /// Calculate `cursor_index`(the index of the corresponding character in the text)
    /// based on the given x-coordinate.
    fn calc_cursor_index(&self, x_pos: f32) -> usize {
        let str_ref = self.shown_text();
        let str = str_ref.as_str();
        let chars_cnt = str.chars().count();

        let offset = x_pos - self.text_draw_position().x();
        let mut predict_idx = self.props().cursor_index;

        let mut last_diff = f32::MAX;
        let mut last_idx = predict_idx;

        let mut predict_start = self.props().predict_start;
        let len_start = if predict_start > predict_idx {
            std::mem::swap(&mut predict_start, &mut predict_idx);

            predict_start = self.map_shown(predict_start);

            self.font()
                .calc_text_dimension(&str[..predict_start], self.props().letter_spacing)
                .0
        } else {
            predict_start = self.map_shown(predict_start);

            self.props().predict_start_len
        };

        loop {
            let map_idx = self.map_shown(predict_idx);
            if predict_start > map_idx {
                break;
            }
            let actual_len = self
                .font()
                .calc_text_dimension(&str[predict_start..map_idx], self.props().letter_spacing)
                .0
                + len_start;

            let diff = (offset - actual_len).abs();
            if diff >= last_diff {
                predict_idx = last_idx;
                break;
            }
            if diff < self.props().font_dimension.0 * 0.3 {
                break;
            }

            last_diff = diff;
            last_idx = predict_idx;

            if offset > actual_len {
                if predict_idx == chars_cnt {
                    break;
                }
                predict_idx += (diff / self.props().font_dimension.0).max(1.) as usize;
            } else {
                if predict_idx == 0 {
                    break;
                }
                let sub = (diff / self.props().font_dimension.0).max(1.) as usize;
                if sub >= predict_idx {
                    predict_idx = 0
                } else {
                    predict_idx -= sub;
                }
            }
        }
        predict_idx.min(chars_cnt)
    }

    #[inline]
    fn handle_mouse_double_click(&mut self) {
        self.select_all();
    }

    #[inline]
    fn adjust_selection_range(&mut self, start: Option<i32>, end: Option<i32>) {
        let mut changed = false;

        if let Some(start) = start {
            if self.props().selection_start != start {
                self.props_mut().selection_start = start;
                changed = true;
            }
        }

        if let Some(end) = end {
            if self.props().selection_end != end {
                self.props_mut().selection_end = end;
                changed = true;

                emit!(self.selection_changed());
            }
        }

        if changed {
            self.update();
        }
    }

    #[inline]
    fn select_all(&mut self) {
        let len = self.value_chars_count() as i32;

        self.adjust_selection_range(Some(0), Some(len));
    }

    #[inline]
    fn selection_range(&self) -> (usize, usize) {
        (
            self.props().selection_start.min(self.props().selection_end) as usize,
            self.props().selection_end.max(self.props().selection_start) as usize,
        )
    }

    #[inline]
    fn clear_selection(&mut self) {
        self.adjust_selection_range(Some(-1), Some(-1));
    }

    #[inline]
    fn handle_mouse_release(&mut self) {
        self.props_mut().drag_status = DragStatus::None;

        if self.props().selection_start == self.props().selection_end {
            self.props_mut().selection_start = -1;
            self.props_mut().selection_end = -1;
        }

        self.props_mut().cursor_visible = true;
        self.start_blink_timer();
        self.update();
    }

    #[inline]
    fn handle_geometry_changed(&mut self, rect: FRect) {
        if rect.width() == 0. || rect.height() == 0. {
            return;
        }

        self.calc_text_geometry();
    }

    #[inline]
    fn start_blink_timer(&mut self) {
        if !self.props().blink_timer.is_active() {
            self.props_mut().blink_timer.start(Duration::from_millis(
                application::cursor_blinking_time() as u64,
            ))
        }
    }

    #[inline]
    fn calc_widget_height(&self) -> f32 {
        self.props().calc_widget_height()
    }

    #[inline]
    fn blink_event(&mut self) {
        self.props_mut().cursor_visible = !self.props().cursor_visible;
        self.update();
    }

    #[inline]
    fn text_draw_position(&self) -> &FPoint {
        self.props()
            .text_draw_position
            .as_ref()
            .expect("Fatal error: `text_draw_position` of `Text` was None.")
    }

    #[inline]
    fn text_draw_position_mut(&mut self) -> &mut FPoint {
        self.props_mut()
            .text_draw_position
            .as_mut()
            .expect("Fatal error: `text_draw_position` of `Text` was None.")
    }

    #[inline]
    fn save_revoke(&mut self) {
        let mem = TextMemory::new(self);
        self.props_mut().revoke_memories.push_back(mem);
        if self.props().revoke_memories.len() > TEXT_DEFAULT_MAX_MEMORIES_SIZE {
            self.props_mut().revoke_memories.pop_front();
        }
    }

    #[inline]
    fn save_redo(&mut self) {
        let mem = TextMemory::new(self);
        self.props_mut().redo_memories.push_back(mem);
        if self.props().redo_memories.len() > TEXT_DEFAULT_MAX_MEMORIES_SIZE {
            self.props_mut().redo_memories.pop_front();
        }
    }

    /// Map the logic index `'from'` which represent the sequence of characters
    /// to the actual index in string of value.
    #[inline]
    fn map(&self, from: usize) -> usize {
        let value_ref = self.value_ref();
        let (idx, _) = value_ref
            .char_indices()
            .nth(from)
            .unwrap_or((value_ref.len(), '\0'));
        idx
    }

    /// Map the logic index `'from'` which represent the sequence of characters
    /// to the actual index in string of shown text.
    #[inline]
    fn map_shown(&self, from: usize) -> usize {
        let value_ref = self.shown_text();
        let (idx, _) = value_ref
            .char_indices()
            .nth(from)
            .unwrap_or((value_ref.len(), '\0'));
        idx
    }

    #[inline]
    fn value_chars_count(&self) -> usize {
        self.value_ref().chars().count()
    }

    #[inline]
    fn value_remove_range(&mut self, start: usize, end: usize) {
        let (start, end) = (self.map(start), self.map(end));
        let (offset, _) = self
            .font()
            .calc_text_dimension(&self.value_ref()[start..end], self.props().letter_spacing);

        self.input_wrapper()
            .value_mut()
            .replace_range(start..end, "");
        if start != end {
            emit!(self.value_changed());
        }

        let text_window = self.props().text_window;
        let draw_pos = self.text_draw_position_mut();
        if draw_pos.x() < text_window.left() {
            draw_pos.offset(offset, 0.);
            self.calc_predict_index();
        }

        let draw_pos = self.text_draw_position_mut();
        draw_pos.set_x(draw_pos.x().min(text_window.left()));
    }

    #[inline]
    fn render_text(&self, painter: &mut Painter, text: &str, mut origin: FPoint) {
        painter.prepare_paragraph(text, self.props().letter_spacing, f32::MAX, None, false);

        let paragraph = painter.get_paragraph().unwrap();
        let baseline = paragraph.single_line_baseline();
        let height = paragraph.height();
        if height > baseline {
            origin.offset(0., self.props().text_window.height() - height);
        }

        painter.draw_paragrah_prepared_global(origin);
    }

    #[inline]
    fn on_value_changed(&mut self) {
        let (bytes_len, chars_len) = {
            let val_ref = self.value_ref();
            let val = val_ref.as_str();

            (val.len(), val.chars().count())
        };

        if bytes_len != 0 && chars_len != 0 && self.props().require_invalid {
            self.props_mut().require_invalid = false;
            if self.is_focus() {
                self.set_border_color(INPUT_FOCUSED_BORDER_COLOR);
            } else {
                self.set_border_color(INPUT_DEFAULT_BORDER_COLOR);
            }
        }

        if bytes_len != chars_len && !self.props().unicode_text {
            self.props_mut().font_dimension = self.font().calc_font_dimension_unicode();
            self.props_mut().unicode_text = true;
        } else if bytes_len == chars_len && self.props().unicode_text {
            self.props_mut().font_dimension = self.font().calc_font_dimension();
            self.props_mut().unicode_text = false;
        }

        self.clear_selection();
        self.update();
    }

    #[inline]
    fn calc_predict_index(&mut self) {
        let (a, b) = {
            let value_ref = self.shown_text();
            let str = value_ref.as_str();

            if str.is_empty() {
                (0, 0.)
            } else {
                let predict_start = (((self.props().text_window.x() + 2.
                    - self.text_draw_position().x())
                    / self.props().font_dimension.0) as usize)
                    .min(str.chars().count());
                let predict_pref_len = self
                    .font()
                    .calc_text_dimension(
                        &str[..self.map_shown(predict_start)],
                        self.props().letter_spacing,
                    )
                    .0;
                (predict_start, predict_pref_len)
            }
        };

        self.props_mut().predict_start = a;
        self.props_mut().predict_start_len = b;
    }

    #[inline]
    fn inner_required_handle(&mut self) -> bool {
        if self.value().is_empty() {
            self.props_mut().require_invalid = true;
            if let Some(color) = self.props().require_invalid_border_color {
                self.set_border_color(color);
            }
            self.update();
            false
        } else {
            true
        }
    }
}

#[derive(Debug)]
pub struct TextMemory {
    cursor_index: usize,
    selection_start: i32,
    selection_end: i32,
    value: String,
    text_draw_position: FPoint,
}

impl TextMemory {
    #[inline]
    fn new<T: TextInnerExt>(text: &T) -> Self {
        TextMemory {
            cursor_index: text.props().cursor_index,
            selection_start: text.props().selection_start,
            selection_end: text.props().selection_end,
            value: text.value(),
            text_draw_position: *text.text_draw_position(),
        }
    }
}

impl Text {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}

impl TextExt for Text {}
impl TextInnerExt for Text {}

impl InputEle for Text {
    input_ele_impl!();
    
    #[inline]
    fn on_tab_focused(&mut self) {
        self.select_all()
    }
}