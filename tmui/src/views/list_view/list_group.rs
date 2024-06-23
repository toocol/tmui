use super::{list_item::{ItemType, ListItem}, list_node::ListNode};

#[derive(Default)]
pub struct ListGroup {
    nodes: Vec<ListNode>,
}

impl ListGroup {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl ListItem for ListGroup {
    #[inline]
    fn item_type(&self) -> ItemType {
        ItemType::Group
    }

    fn render(&mut self) {
        todo!()
    }
}