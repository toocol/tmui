pub mod list_group;
pub mod list_item;
pub mod list_node;
pub mod list_store;
pub mod list_view_image;
pub mod list_view_object;
pub mod list_separator;

use list_group::ListGroup;
use list_store::ListStore;
use list_view_image::ListViewImage;
use list_view_object::ListViewObject;
use tlib::connect;

use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{WidgetHndAsable, WidgetImpl},
};

#[extends(Widget, Layout(ScrollArea), internal = true)]
pub struct ListView {}

impl ObjectSubclass for ListView {
    const NAME: &'static str = "ListView";
}

impl ObjectImpl for ListView {
    fn construct(&mut self) {
        self.parent_construct();
        
        let mut img = ListViewImage::new(self.scroll_bar_mut());
        img.store_mut().set_view(self.as_hnd());
        img.set_scroll_bar(self.scroll_bar_mut());

        connect!(self, background_changed(), img, set_background(Color));
        connect!(self, invalid_area_changed(), img, set_invalid_area(FRect));

        self.set_area(img);
    }
}

impl WidgetImpl for ListView {}

impl ListView {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }

    #[inline]
    pub fn add_node(&mut self, obj: &dyn ListViewObject) {
        self.get_store_mut().add_node(obj)
    }

    #[inline]
    pub fn add_group(&mut self, group: ListGroup) {
        self.get_store_mut().add_group(group)
    }

    #[inline]
    pub fn get_store(&self) -> &ListStore {
        self.get_image().store()
    }

    #[inline]
    pub fn get_store_mut(&mut self) -> &mut ListStore {
        self.get_image_mut().store_mut()
    }

    #[inline]
    pub fn set_line_spacint(&mut self, line_spacing: i32) {
        self.get_image_mut().set_line_spacing(line_spacing)
    }
}

impl ListView {
    #[inline]
    pub(crate) fn get_image(&self) -> &ListViewImage {
        self.get_area_cast::<ListViewImage>().unwrap()
    }

    #[inline]
    pub(crate) fn get_image_mut(&mut self) -> &mut ListViewImage {
        self.get_area_cast_mut::<ListViewImage>().unwrap()
    }
}