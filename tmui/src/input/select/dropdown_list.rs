#[cfg(not(win_popup))]
use crate::widget::widget_ext::FocusStrat;
use crate::{
    graphics::box_shadow::BoxShadow,
    prelude::*,
    scroll_area::LayoutMode,
    tlib::object::{ObjectImpl, ObjectSubclass},
    views::list_view::{list_node::ListNode, list_view_object::ListViewObject, ListView},
    widget::WidgetImpl,
};
use tlib::signals;

const MAX_VISIBLE_ITEMS: i32 = 20;

#[cfg(not(win_popup))]
pub trait DropdownListSignals: ActionExt {
    signals!(
        DropdownListSignals:

        /// Emit when list value's selected value chaged.
        ///
        /// @param [`String`]
        value_changed(String);
    );
}
#[cfg(not(win_popup))]
impl DropdownListSignals for DropdownList {}

#[cfg(win_popup)]
#[extends(Popup)]
#[derive(Childable)]
#[tlib::win_widget(o2s(DropdownListCrsMsg), s2o(DropdownListCrsMsg))]
pub struct DropdownList {
    #[child]
    list: Box<ListView>,
}

#[cfg(not(win_popup))]
#[extends(Popup)]
#[derive(Childable)]
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

        #[cfg(win_popup)]
        self.set_hexpand(true);

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

            #[cfg(not(win_popup))]
            emit!(dropdown_list, value_changed(val));
            #[cfg(win_popup)]
            dropdown_list.send_cross_win_msg(DropdownListCrsMsg::ValueChanged(val));

            #[cfg(not(win_popup))]
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

#[cfg(not(win_popup))]
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

    #[inline]
    fn on_mouse_click_hide(&mut self) {
        self.trans_focus_take(FocusStrat::Restore);
    }
}

impl DropdownList {
    #[cfg(not(win_popup))]
    #[inline]
    pub(crate) fn new() -> Box<Self> {
        Object::new(&[])
    }

    #[inline]
    pub(crate) fn clear_options(&mut self) {
        self.list.clear();
    }

    #[cfg(not(win_popup))]
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

    #[cfg(not(win_popup))]
    #[inline]
    pub(crate) fn trans_focus_take(&mut self, strat: FocusStrat) {
        self.list.take_over_focus(strat);
    }
}

#[cfg(win_popup)]
pub trait CorrDropdownListSignals: ActionExt {
    signals!(
        CorrDropdownListSignals:

        value_changed(String);
    );
}
#[cfg(win_popup)]
impl CorrDropdownListSignals for CorrDropdownList {}

#[cfg(win_popup)]
impl CorrDropdownList {
    #[inline]
    pub(crate) fn clear_options(&mut self) {
        self.send_cross_win_msg(DropdownListCrsMsg::ClearOptions);
    }

    #[inline]
    pub(crate) fn add_option(&mut self, option: &dyn ListViewObject) {
        self.send_cross_win_msg(DropdownListCrsMsg::AddOption(ListNode::from(option)));
    }

    #[inline]
    pub(crate) fn scroll_to(&mut self, idx: usize) {
        self.send_cross_win_msg(DropdownListCrsMsg::ScrollTo(idx));
    }

    #[inline]
    pub(crate) fn calc_height(&mut self) {
        self.send_cross_win_msg(DropdownListCrsMsg::CalcHeight);
    }
}

////////////////////////////// Cross window message define/handle
#[cfg(win_popup)]
pub enum DropdownListCrsMsg {
    // Origin to sink:
    ClearOptions,
    AddOption(ListNode),
    ScrollTo(usize),
    CalcHeight,

    // Sink to origin:
    ValueChanged(String),
}

impl CrossWinMsgHandler for CorrDropdownList {
    type T = DropdownListCrsMsg;

    fn handle(&mut self, msg: Self::T) {
        if let DropdownListCrsMsg::ValueChanged(val) = msg {
            emit!(self, value_changed(val))
        }
    }
}

impl CrossWinMsgHandler for DropdownList {
    type T = DropdownListCrsMsg;

    fn handle(&mut self, msg: Self::T) {
        match msg {
            DropdownListCrsMsg::ClearOptions => {
                self.clear_options();
            }
            DropdownListCrsMsg::AddOption(node) => {
                self.list.add_node_directly(node);
            }
            DropdownListCrsMsg::ScrollTo(to) => {
                self.scroll_to(to);
            }
            DropdownListCrsMsg::CalcHeight => {
                self.calc_height();
            }
            _ => (),
        }
    }
}
