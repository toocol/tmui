use super::{
    list_item::{ItemType, ListItemCast, RenderCtx}, list_separator::GroupSeparator, list_store::{ListStore, ListStoreSignals}
};
use crate::{
    font::FontCalculation,
    prelude::*,
    scroll_bar::ScrollBar,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{RegionClear, WidgetImpl},
};
use std::ptr::NonNull;
use tlib::{connect, disconnect, nonnull_mut, run_after};

#[extends(Widget)]
#[loadable]
#[run_after]
pub struct ListViewImage {
    store: Box<ListStore>,
    scroll_bar: Option<NonNull<ScrollBar>>,

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

        connect!(
            self.store,
            internal_scroll_value_changed(),
            self,
            internal_scroll_value_changed(i32)
        );
        connect!(
            self.store,
            items_len_changed(),
            self,
            on_items_changed(usize)
        );
        connect!(
            nonnull_mut!(self.scroll_bar),
            value_changed(),
            self,
            scroll_bar_value_changed(i32)
        );

        self.on_items_changed(self.store.get_items_len());
    }

    #[inline]
    fn paint(&mut self, painter: &mut Painter) {
        self.draw_image(painter)
    }

    #[inline]
    fn font_changed(&mut self) {
        self.on_font_changed();

        self.calc_window_lines();
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

    #[inline]
    pub(crate) fn set_line_spacing(&mut self, line_spacing: i32) {
        self.line_spacing = line_spacing;
    }
}

impl ListViewImage {
    #[inline]
    fn draw_image(&mut self, painter: &mut Painter) {
        let mut rect = self.contents_rect_f(Some(Coordinate::Widget));
        self.clear(painter, rect);

        let background = self.opaque_background();
        rect.set_x(rect.x() + self.indent_length as f32);
        rect.set_width(rect.width() - self.indent_length as f32);

        let (image, y_offset) = self.store.get_image();
        let mut offset =
            rect.y() - (y_offset as f32 / 10. * (self.line_height + self.line_spacing) as f32);

        for item in image {
            rect.set_y(offset);
            match item.item_type() {
                ItemType::Node => {
                    offset += self.line_height as f32 + self.line_spacing as f32;
                    rect.set_height(self.line_height as f32);
                }
                ItemType::Separator => {
                    let separator_height = item.downcast_ref::<GroupSeparator>().unwrap().separator_height() as f32;
                    offset += separator_height;
                    rect.set_height(separator_height);
                }
            }

            item.render(painter, RenderCtx::new(rect, background))
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
        scroll_bar.set_single_step(5);
        scroll_bar.set_page_step(window_lines * 10);
        scroll_bar.set_visible_area(window_lines * 10);
    }

    #[inline]
    fn scroll_bar_value_changed(&mut self, value: i32) {
        if self.store.scroll_to(value, false) {
            self.update();
        }
    }

    fn internal_scroll_value_changed(&mut self, value: i32) {
        let scroll_bar = nonnull_mut!(self.scroll_bar);
        disconnect!(scroll_bar, value_changed(), null, null);
        scroll_bar.set_value(value);
        connect!(
            scroll_bar,
            value_changed(),
            self,
            scroll_bar_value_changed(i32)
        );

        self.update();
    }

    #[inline]
    pub fn on_items_changed(&mut self, len: usize) {
        nonnull_mut!(self.scroll_bar)
            .set_range(0, (len as i32 - self.store.get_window_lines()) * 10);
    }
}
