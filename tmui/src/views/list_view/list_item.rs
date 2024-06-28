use super::Painter;
use tlib::{
    figure::{Color, FRect},
    global::AsAny,
};

pub trait ListItem: AsAny {
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

#[derive(Debug, Clone, Copy)]
pub enum ItemType {
    Node,
    Separator,
}

#[derive(Debug, Clone, Copy)]
pub struct RenderCtx {
    pub(crate) geometry: FRect,
    pub(crate) background: Color,
}
impl RenderCtx {
    #[inline]
    pub(crate) fn new(
        geometry: FRect,
        background: Color,
    ) -> Self {
        Self {
            geometry,
            background,
        }
    }
}
