use crate::views::node::RenderCtx;
use super::Painter;
use tlib::global::AsAny;

pub trait ListItem: AsAny + Send + Sync {
    fn item_type(&self) -> ItemType;

    fn render(&self, painter: &mut Painter, render_ctx: RenderCtx);
}
pub trait ListItemCast: AsAny {
    #[inline]
    fn downcast_ref<T: 'static + ListItem>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }

    #[inline]
    fn downcast_mut<T: 'static + ListItem>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut()
    }
}
impl<T: ListItem> ListItemCast for T {}
impl ListItemCast for dyn ListItem {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Node,
    Separator,
}
