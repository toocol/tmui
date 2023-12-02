pub mod cell;
pub mod tree_node;
pub mod tree_store;
pub mod tree_view_image;
pub mod tree_view_object;

use self::tree_view_image::TreeViewImage;
use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};
use tlib::signals;

#[extends(Widget, Layout(ScrollArea), internal = true)]
pub struct TreeView {}

impl ObjectSubclass for TreeView {
    const NAME: &'static str = "TreeView";
}

impl ObjectImpl for TreeView {
    fn construct(&mut self) {
        self.parent_construct();

        let image = TreeViewImage::new(self.get_scroll_bar_mut());

        self.set_area(image);
    }
}

impl WidgetImpl for TreeView {}

impl TreeView {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
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
