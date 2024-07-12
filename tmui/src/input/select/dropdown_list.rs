use crate::{
    graphics::box_shadow::BoxShadow,
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
        self.set_borders(1., 1., 1., 1.);
        self.set_border_color(Color::GREY_LIGHT);
        self.set_box_shadow(BoxShadow::new(6., Color::BLACK, None, None, None));

        self.list.set_hexpand(true);
        self.list.set_vexpand(true);

        let scroll_bar = self.list.scroll_bar_mut();
        scroll_bar.set_overlaid(true);
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
            Point::new(tl.x(), tl.y() - vr.height() as i32)
        } else {
            Point::new(bl.x(), bl.y())
        }
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

    #[inline]
    pub fn calc_height(&mut self) {
        let height = (self.list.get_line_height() + self.list.get_line_spacing()) * self.list.len() as i32;
        self.height_request(height + 1 * 2)
    }
}
