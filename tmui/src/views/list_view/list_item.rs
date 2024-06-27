use super::Painter;
use tlib::{
    figure::{Color, FRect},
    global::AsAny,
    object::ObjectId,
};

pub trait ListItem: AsAny {
    fn id(&self) -> ObjectId;

    fn item_type(&self) -> ItemType;

    fn render(&self, painter: &mut Painter, render_ctx: RenderCtx);
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

#[derive(Debug, Clone, Copy)]
pub struct RenderCtx {
    pub(crate) geometry: FRect,
    pub(crate) background: Color,
    pub(crate) line_height: f32,
    pub(crate) line_spacing: f32,
}
impl RenderCtx {
    #[inline]
    pub(crate) fn new(
        geometry: FRect,
        background: Color,
        line_height: f32,
        line_spacing: f32,
    ) -> Self {
        Self {
            geometry,
            background,
            line_height,
            line_spacing,
        }
    }
}
