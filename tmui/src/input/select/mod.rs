pub mod dropdown_list;
pub mod select_option;

use super::{Input, InputBounds, InputEle, InputSignals, InputWrapper, ReflectInputEle, INPUT_FOCUSED_BORDER_COLOR};
use crate::{
    asset::Asset, font::FontCalculation, input::INPUT_DEFAULT_BORDER_COLOR, input_ele_impl, prelude::*, svg::{svg_attr::SvgAttr, svg_str::SvgStr}, tlib::object::{ObjectImpl, ObjectSubclass}, widget::{widget_inner::WidgetInnerExt, WidgetImpl}
};
#[cfg(not(win_popup))]
use crate::widget::widget_ext::FocusStrat;
#[cfg(not(win_popup))]
use dropdown_list::{DropdownList, DropdownListSignals};
#[cfg(win_popup)]
use dropdown_list::{CorrDropdownList, CorrDropdownListSignals};
use select_option::SelectOption;
use tlib::{
    connect, events::MouseEvent, global::PrecisionOps, namespace::MouseButton, run_after,
    skia_safe::FontMgr, typedef::SkiaSvgDom,
};

const MINIMUN_WIDTH: i32 = 25;
#[cfg(win_popup)]
const MINIMUN_HEIGHT: i32 = 10;
const TEXT_MARGIN: i32 = 3;

const ARROW_PADDING: f32 = 2.;
const ARROW_SIZE: f32 = 10.;

pub trait SelectBounds: InputBounds + ToString + From<String> {}
impl<T: InputBounds + ToString + From<String>> SelectBounds for T {}

#[extends(Widget)]
#[popupable]
#[run_after]
pub struct Select<T: SelectBounds> {
    input_wrapper: InputWrapper<T>,
    maximum_text: String,
    dom: Option<SkiaSvgDom>,
}

impl<T: SelectBounds> ObjectSubclass for Select<T> {
    const NAME: &'static str = "Select";
}

impl<T: SelectBounds> ObjectImpl for Select<T> {
    fn construct(&mut self) {
        self.parent_construct();
        self.set_border_radius(2.);
        self.set_borders(1., 1., 1., 1.);
        self.set_border_color(INPUT_DEFAULT_BORDER_COLOR);
        self.set_fixed_width(MINIMUN_WIDTH);
        self.set_detecting_width(MINIMUN_WIDTH);

        self.input_wrapper.init(self.id());

        #[cfg(not(win_popup))]
        {
            let mut dropdown_list = DropdownList::new();
            dropdown_list.width_request(MINIMUN_WIDTH);
            connect!(
                dropdown_list,
                value_changed(),
                self,
                dropdown_list_value_changed(String)
            );
            self.add_popup(dropdown_list);
        }

        #[cfg(win_popup)]
        {
            let mut dropdown_list = crate::input::select::dropdown_list::CorrDropdownList::new();
            dropdown_list.width_request(MINIMUN_WIDTH);
            connect!(
                dropdown_list,
                value_changed(),
                self,
                dropdown_list_value_changed(String)
            );
            self.add_popup(dropdown_list);
        }

        let size = ARROW_SIZE as u32;
        let arrow = SvgStr::get::<Asset>(
            "arrow_down_small.svg",
            SvgAttr::new(size, size, Color::BLACK),
        )
        .unwrap();
        self.dom = Some(
            SkiaSvgDom::from_str(arrow, FontMgr::default())
                .expect("`Select` create svg dom `arrow_down_small` failed."),
        );
    }

    #[inline]
    fn type_register(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<Self, ReflectInputEle>()
    }
}

impl<T: SelectBounds> WidgetImpl for Select<T> {
    #[inline]
    fn paint(&mut self, painter: &mut Painter) {
        self.draw_text(painter);

        self.draw_arrow(painter)
    }

    #[inline]
    fn run_after(&mut self) {
        self.font_changed();
    }

    #[inline]
    fn enable_focus(&self) -> bool {
        true
    }

    #[inline]
    fn font_changed(&mut self) {
        self.on_font_changed()
    }

    #[inline]
    fn on_mouse_pressed(&mut self, event: &MouseEvent) {
        if event.mouse_button() != MouseButton::LeftButton {
            return;
        }

        self.show_popup(event.position().into());
        #[cfg(not(win_popup))]
        self.dropdown_list_mut()
            .trans_focus_take(FocusStrat::TakeOver);
    }

    #[inline]
    fn on_get_focus(&mut self) {
        self.set_borders(2., 2., 2., 2.);
        self.set_border_color(INPUT_FOCUSED_BORDER_COLOR);
    }

    #[inline]
    fn on_lose_focus(&mut self) {
        self.set_borders(1., 1., 1., 1.);
        self.set_border_color(INPUT_DEFAULT_BORDER_COLOR);
    }
}

impl<T: SelectBounds> InputSignals for Select<T> {}

impl<T: SelectBounds> Input for Select<T> {
    type Value = T;

    #[inline]
    fn input_type(&self) -> super::InputType {
        super::InputType::Select
    }

    #[inline]
    fn input_wrapper(&self) -> &super::InputWrapper<Self::Value> {
        &self.input_wrapper
    }

    #[inline]
    fn required_handle(&mut self) -> bool {
        true
    }
}

impl<T: SelectBounds> Select<T> {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }

    #[inline]
    pub fn set_options(&mut self, options: &[SelectOption<T>]) {
        if options.is_empty() {
            return;
        }
        self.dropdown_list_mut().clear_options();
        self.maximum_text = String::new();

        let default_val = options.first().unwrap().value();
        let mut max_width = 0;
        let mut idx = 0;
        self.set_value(default_val);

        for (i, option) in options.iter().enumerate() {
            let value = option.value();
            let val_str = value.to_string();
            let (w, _) = self.font().calc_text_dimension(&val_str, 0.).ceil();
            if w as i32 > max_width {
                max_width = w as i32;
                self.maximum_text = val_str;
            }

            self.dropdown_list_mut().add_option(option);

            if option.is_selected() {
                idx = i;

                self.set_value(value);
            }
        }

        let width = max_width + ARROW_SIZE.ceil() as i32 + ARROW_PADDING as i32 * 2 + TEXT_MARGIN;
        self.set_fixed_width(width);
        self.set_detecting_width(width);

        let dropdown_list = self.dropdown_list_mut();
        dropdown_list.width_request(width);
        dropdown_list.calc_height();

        self.dropdown_list_mut().scroll_to(idx);

        self.window().layout_change(self);

        self.update();
    }

    #[cfg(not(win_popup))]
    #[inline]
    pub fn dropdown_list(&self) -> &DropdownList {
        self.get_popup_ref()
            .unwrap()
            .as_any()
            .downcast_ref::<DropdownList>()
            .unwrap()
    }

    #[cfg(not(win_popup))]
    #[inline]
    pub fn dropdown_list_mut(&mut self) -> &mut DropdownList {
        self.get_popup_mut()
            .unwrap()
            .as_any_mut()
            .downcast_mut::<DropdownList>()
            .unwrap()
    }

    #[cfg(win_popup)]
    #[inline]
    pub fn dropdown_list(&self) -> &CorrDropdownList {
        self.get_popup_ref()
            .unwrap()
            .as_any()
            .downcast_ref::<CorrDropdownList>()
            .unwrap()
    }

    #[cfg(win_popup)]
    pub fn dropdown_list_mut(&mut self) -> &mut CorrDropdownList {
        self.get_popup_mut()
            .unwrap()
            .as_any_mut()
            .downcast_mut::<CorrDropdownList>()
            .unwrap()
    }
}

impl<T: SelectBounds> Select<T> {
    #[inline]
    fn draw_text(&mut self, painter: &mut Painter) {
        let text = self.value().to_string();
        let pos = self.text_rect().top_left();
        painter.set_color(Color::BLACK);
        painter.draw_paragraph_global(&text, pos, 0., f32::MAX, Some(1), false)
    }

    #[inline]
    fn draw_arrow(&mut self, painter: &mut Painter) {
        if let Some(ref dom) = self.dom {
            painter.save();
            let pos = self.arrow_pos();
            painter.translate(pos.x(), pos.y());
            painter.draw_dom(dom);
            painter.restore();
        }
    }

    #[inline]
    fn text_rect(&self) -> Rect {
        let mut rect = self.rect();

        rect.set_x(rect.x() + TEXT_MARGIN);
        rect.set_y(rect.y() + TEXT_MARGIN);

        let arrow_width = (ARROW_SIZE + ARROW_PADDING * 2.) as i32;
        rect.set_width(rect.width() - TEXT_MARGIN - arrow_width);
        rect.set_height(rect.height() - TEXT_MARGIN * 2);

        rect
    }

    #[inline]
    fn arrow_pos(&self) -> FPoint {
        let mut rect = self.rect_f();

        rect.set_x(rect.x() + (rect.width() - ARROW_SIZE - ARROW_PADDING * 2.));
        rect.set_y(rect.y() + (rect.height() - ARROW_SIZE) / 2.);

        rect.top_left()
    }

    #[inline]
    fn on_font_changed(&mut self) {
        let (_, h) = self.font().calc_font_dimension().ceil();
        let height = h as i32 + TEXT_MARGIN * 2;
        self.set_fixed_height(height);
        self.set_detecting_height(height);

        let font = self.font().clone();
        let dropdown_list = self.dropdown_list_mut();
        dropdown_list.set_font(font);
        dropdown_list.calc_height();

        self.window().layout_change(self);
    }

    #[inline]
    fn dropdown_list_value_changed(&mut self, val: String) {
        self.set_value(T::from(val));
        self.update_rect(CoordRect::new(self.text_rect(), Coordinate::World));
        self.set_render_styles(true);
    }
}

input_ele_impl!(Select<T: SelectBounds>);