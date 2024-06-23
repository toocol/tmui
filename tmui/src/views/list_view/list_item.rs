pub trait ListItem {
    fn item_type(&self) -> ItemType;

    fn render(&mut self);
}

#[derive(Debug, Clone, Copy)]
pub enum ItemType {
    Group,
    Node,
}