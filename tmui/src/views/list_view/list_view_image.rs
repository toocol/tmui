use crate::{
   prelude::*,
   tlib::object::{ObjectImpl, ObjectSubclass},
   widget::WidgetImpl,
};

#[extends(Widget)]
#[loadable]
pub struct ListViewImage {}

impl ObjectSubclass for ListViewImage {
   const NAME: &'static str = "ListViewImage";
}

impl ObjectImpl for ListViewImage {}

impl WidgetImpl for ListViewImage {}