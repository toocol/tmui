use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    views::list_view::{list_view_object::ListViewObject, ListView},
    widget::WidgetImpl,
};

#[extends(Popup, internal = true)]
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

        self.list.set_hexpand(true);
        self.list.set_vexpand(true);

        let scroll_bar = self.list.scroll_bar_mut();
        scroll_bar.set_overlaid(true);
        scroll_bar.set_visible_in_valid(true);
    }
}

impl WidgetImpl for DropdownList {}

impl PopupImpl for DropdownList {
    fn calculate_position(&self, base_rect: Rect, mut point: Point) -> Point {
        point
    }
}

impl DropdownList {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }

    #[inline]
    pub fn clear_options(&mut self) {
        self.list.clear();
    }

    #[inline]
    pub fn add_option(&mut self, option: &dyn ListViewObject) {
        self.list.add_node(option);
    }

    #[inline]
    pub fn scroll_to(&mut self, idx: usize) {
        self.list.scroll_to(idx)
    }
}
