use tlib::{figure::Rect, global::AsAny, object::ObjectId};

use super::Painter;

pub trait ListItem: AsAny {
    fn id(&self) -> ObjectId;

    fn item_type(&self) -> ItemType;

    fn render(&mut self, painter: &mut Painter, geometry: Rect);
}
pub trait ListItemCast: AsAny {
    #[inline]
    fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }

    #[inline]
    fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut()
    }
}
impl<T: ListItem> ListItemCast for T {}
impl ListItemCast for dyn ListItem {}

#[derive(Debug, Clone, Copy)]
pub enum ItemType {
    Group,
    Node,
}