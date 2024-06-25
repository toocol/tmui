use super::list_store::ListStore;
use crate::{
    prelude::*,
    scroll_bar::ScrollBar,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};
use std::ptr::NonNull;

#[extends(Widget)]
#[loadable]
pub struct ListViewImage {
    store: Box<ListStore>,
    scroll_bar: Option<NonNull<ScrollBar>>,

    y_offset: f32,
}

impl ObjectSubclass for ListViewImage {
    const NAME: &'static str = "ListViewImage";
}

impl ObjectImpl for ListViewImage {}

impl WidgetImpl for ListViewImage {
    fn paint(&mut self, painter: &mut Painter) {
        let rect = self.contents_rect(Some(Coordinate::Widget));

        for item in self.store.get_image() {
            item.render(painter, rect)
        }
    }
}

impl ListViewImage {
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
