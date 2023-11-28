use tlib::signals;
use crate::{
   prelude::*,
   tlib::object::{ObjectImpl, ObjectSubclass},
   widget::WidgetImpl,
};

#[extends(Widget, Layout(ScrollArea), internal = true)]
pub struct TreeView {}

impl ObjectSubclass for TreeView {
   const NAME: &'static str = "TreeView";
}

impl ObjectImpl for TreeView {}

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