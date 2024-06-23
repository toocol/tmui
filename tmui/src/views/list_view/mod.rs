pub mod list_group;
pub mod list_item;
pub mod list_node;
pub mod list_store;
pub mod list_view_image;
pub mod list_view_object;

use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget, Layout(ScrollArea), internal = true)]
pub struct ListView {}

impl ObjectSubclass for ListView {
    const NAME: &'static str = "ListView";
}

impl ObjectImpl for ListView {}

impl WidgetImpl for ListView {}

