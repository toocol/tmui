use tlib::{nonnull_mut, run_after};

use super::{list_group::ListGroup, list_item::{ItemType, ListItemCast, RenderCtx}, list_store::ListStore};
use crate::{
    font::FontCalculation, prelude::*, scroll_bar::ScrollBar, tlib::object::{ObjectImpl, ObjectSubclass}, widget::WidgetImpl
};
use std::ptr::NonNull;

#[extends(Widget)]
#[loadable]
#[run_after]
pub struct ListViewImage {
    store: Box<ListStore>,
    scroll_bar: Option<NonNull<ScrollBar>>,

    y_offset: f32,

    indent_length: i32,
    #[derivative(Default(value = "1"))]
    line_height: i32,
    line_spacing: i32,
}

impl ObjectSubclass for ListViewImage {
    const NAME: &'static str = "ListViewImage";
}

impl ObjectImpl for ListViewImage {}

impl WidgetImpl for ListViewImage {
    #[inline]
    fn run_after(&mut self) {
        self.font_changed();
    }

    #[inline]
    fn paint(&mut self, painter: &mut Painter) {
        self.draw_image(painter)
    }

    #[inline]
    fn font_changed(&mut self) {
        self.on_font_changed()
    }
}

impl ListViewImage {
    #[inline]
    pub(crate) fn new(scroll_bar: &mut ScrollBar) -> Box<Self> {
        let mut img: Box<ListViewImage> = Object::new(&[]);
        img.scroll_bar = NonNull::new(scroll_bar);
        img
    }

    #[inline]
    pub(crate) fn set_scroll_bar(&mut self, scroll_bar: &mut ScrollBar) {
        self.scroll_bar = NonNull::new(scroll_bar)
    }

    #[inline]
    pub(crate) fn store(&self) -> &ListStore {
        &self.store
    }

    #[inline]
    pub(crate) fn store_mut(&mut self) -> &mut ListStore {
        &mut self.store
    }
}

impl ListViewImage {
    #[inline]
    fn draw_image(&mut self, painter: &mut Painter) {
        let mut rect = self.contents_rect_f(Some(Coordinate::Widget));
        let background = self.opaque_background();
        let mut offset = rect.y();
        rect.set_x(rect.x() + self.indent_length as f32);

        for item in self.store.get_image() {
            rect.set_y(offset);
            match item.item_type() {
                ItemType::Node => {
                    offset += self.line_height as f32 + self.line_spacing as f32;
                    rect.set_height(self.line_height as f32);
                },
                ItemType::Group => {
                    let len = item.downcast_ref::<ListGroup>().unwrap().len() as f32;
                    let height = (self.line_height + self.line_spacing) as f32 * len;
                    offset += height;
                    rect.set_height(height);
                }
            }

            item.render(
                painter,
                RenderCtx::new(
                    rect,
                    background,
                    self.line_height as f32,
                    self.line_spacing as f32,
                ),
            )
        }
    }

    #[inline]
    fn on_font_changed(&mut self) {
        let (_, h) = self.font().calc_font_dimension();

        self.line_height = h as i32;
    }

    fn calc_window_lines(&mut self) {
        let size = self.size();
        let window_lines = size.height() / (self.line_height + self.line_spacing);
        self.store.set_window_lines(window_lines);

        let scroll_bar = nonnull_mut!(self.scroll_bar);
        scroll_bar.set_single_step(1);
        scroll_bar.set_page_step(window_lines);
        scroll_bar.set_visible_area(window_lines);
    }
}
