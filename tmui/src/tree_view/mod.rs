pub mod cell;
pub mod node_render;
pub mod tree_node;
pub mod tree_store;
pub mod tree_view_image;
pub mod tree_view_object;

use self::{tree_store::TreeStore, tree_view_image::TreeViewImage};
use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};
use tlib::{connect, signals};

#[extends(Widget, Layout(ScrollArea), internal = true)]
pub struct TreeView {}

impl ObjectSubclass for TreeView {
    const NAME: &'static str = "TreeView";
}

impl ObjectImpl for TreeView {
    fn construct(&mut self) {
        self.parent_construct();

        let mut image = TreeViewImage::new(self.get_scroll_bar_mut());

        connect!(self, background_changed(), image, set_background(Color));

        self.set_area(image);
    }
}

impl WidgetImpl for TreeView {}

impl TreeView {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }

    #[inline]
    pub fn get_store(&self) -> &TreeStore {
        self.get_image().get_store()
    }

    #[inline]
    pub fn get_store_mut(&mut self) -> &mut TreeStore {
        self.get_image_mut().get_store_mut()
    }

    #[inline]
    pub fn set_indent_length(&mut self, indent_length: i32) {
        self.get_image_mut().set_indent_length(indent_length)
    }

    #[inline]
    pub fn set_line_spacing(&mut self, line_spacing: i32) {
        self.get_image_mut().set_line_spacing(line_spacing)
    }
}

impl TreeView {
    #[inline]
    pub(crate) fn get_image(&self) -> &TreeViewImage {
        self.get_area_cast::<TreeViewImage>().unwrap()
    }

    #[inline]
    pub(crate) fn get_image_mut(&mut self) -> &mut TreeViewImage {
        self.get_area_cast_mut::<TreeViewImage>().unwrap()
    }
}

pub trait TreeViewSignals: ActionExt {
    signals!(
        TreeViewSignals:

        selection_changed();

        row_activated();
    );
}
impl TreeViewSignals for TreeView {}
