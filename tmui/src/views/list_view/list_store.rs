use super::{list_item::ListItem, WidgetHnd};

pub struct ListStore {
    view: WidgetHnd,
    items: Vec<Box<dyn ListItem>>,
}
