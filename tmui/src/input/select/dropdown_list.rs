use crate::{
    graphics::box_shadow::BoxShadow, prelude::*, scroll_area::LayoutMode, tlib::object::{ObjectImpl, ObjectSubclass}, views::list_view::{list_view_object::ListViewObject, ListView}, widget::{widget_ext::FocusStrat, WidgetImpl}
};
use tlib::{global_watch, signals};

const MAX_VISIBLE_ITEMS: i32 = 20;

pub trait DropdownListSignals: ActionExt {
    signals!(
        DropdownListSignals:

        /// Emit when list value's selected value chaged.
        ///
        /// @param [`String`]
        value_changed();
    );
}
impl DropdownListSignals for DropdownList {}

#[extends(Popup, internal = true)]
#[derive(Childable)]
#[global_watch(MousePressed)]
pub struct DropdownList {
    #[child]
    list: Box<ListView>,
}

impl ObjectSubclass for DropdownList {
    const NAME: &'static str = "DropdownList";
}

impl ObjectImpl for DropdownList {
    fn construct(&mut self) {
        self.parent_construct();
        self.set_borders(1., 1., 1., 1.);
        self.set_border_color(Color::GREY_LIGHT);
        self.set_box_shadow(BoxShadow::new(6., Color::BLACK, None, None, None));

        self.list.set_layout_mode(LayoutMode::Overlay);
        self.list.set_hexpand(true);
        self.list.set_vexpand(true);
        self.list.register_node_released(|node, _, _| {
            let val = node.get_value::<String>(0).unwrap();
            let dropdown_list = node
                .get_view()
                .get_parent_mut()
                .unwrap()
                .downcast_mut::<DropdownList>()
                .unwrap();
            emit!(dropdown_list.value_changed(), val);
            dropdown_list.trans_focus_take(FocusStrat::Restore);
            dropdown_list.hide();
        });

        let scroll_bar = self.list.scroll_bar_mut();
        scroll_bar.set_visible_in_valid(true);
    }
}

impl WidgetImpl for DropdownList {
    #[inline]
    fn font_changed(&mut self) {
        self.list.set_font(self.font().clone())
    }
}

impl PopupImpl for DropdownList {
    fn calculate_position(&self, base_rect: Rect, _: Point) -> Point {
        let (tl, bl) = (base_rect.top_left(), base_rect.bottom_left());
        let win_size = self.window().size();
        let vr = self.visual_rect();
        if bl.y() as f32 + vr.height() > win_size.height() as f32 {
            Point::new(tl.x(), tl.y() - self.rect().height())
        } else {
            Point::new(bl.x(), bl.y())
        }
    }
}

impl GlobalWatchImpl for DropdownList {
    fn on_global_mouse_pressed(&mut self, evt: &tlib::events::MouseEvent) -> bool {
        if !self.visible() {
            return false;
        }
        let pos: Point = evt.position().into();
        if !self.rect().contains(&pos) {
            self.trans_focus_take(FocusStrat::Restore);
            self.hide();
        }

        self.supervisor().point_effective(&pos)
    }
}

impl DropdownList {
    #[inline]
    pub(crate) fn new() -> Box<Self> {
        Object::new(&[])
    }

    #[inline]
    pub(crate) fn clear_options(&mut self) {
        self.list.clear();
    }

    #[inline]
    pub(crate) fn add_option(&mut self, option: &dyn ListViewObject) {
        self.list.add_node(option);
    }

    #[inline]
    pub(crate) fn scroll_to(&mut self, idx: usize) {
        self.list.scroll_to(idx)
    }

    #[inline]
    pub(crate) fn calc_height(&mut self) {
        let len = (self.list.len() as i32).min(MAX_VISIBLE_ITEMS);
        if len == 0 {
            self.height_request(self.list.get_line_height());
        } else {
            let height = (self.list.get_line_height() + self.list.get_line_spacing()) * len;

            // Add the height of borders.
            self.height_request(height + 2)
        }
    }

    #[inline]
    pub(crate) fn trans_focus_take(&mut self, strat: FocusStrat) {
        self.list.take_over_focus(strat);
    }
}
