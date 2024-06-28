use super::{
    list_item::{ItemType, ListItem, RenderCtx},
    Painter,
};
use derivative::Derivative;
use tlib::global::AsAny;

pub type ListSeparatorRenderFn = Box<dyn Fn(&mut Painter, RenderCtx)>;

#[derive(Derivative)]
#[derivative(Default)]
pub struct GroupSeparator {
    #[derivative(Default(value = "1."))]
    height: f32,
    #[derivative(Default(value = "Box::new(default_separator_render)"))]
    render_fn: ListSeparatorRenderFn,
}

impl GroupSeparator {
    #[inline]
    pub fn draw_separator(&self, painter: &mut Painter, render_ctx: RenderCtx) {
        self.render(painter, render_ctx)
    }

    #[inline]
    pub fn separator_height(&self) -> f32 {
        self.height
    }

    #[inline]
    pub fn set_separator_height(&mut self, height: f32) {
        self.height = height
    }

    #[inline]
    pub fn set_render_fn<F: Fn(&mut Painter, RenderCtx) + 'static>(&mut self, f: F) {
        self.render_fn = Box::new(f)
    }

    #[inline]
    pub fn as_list_item(self: Box<Self>) -> Box<dyn ListItem> {
        self
    }
}

impl ListItem for GroupSeparator {
    #[inline]
    fn item_type(&self) -> ItemType {
        ItemType::Separator
    }

    #[inline]
    fn render(&self, painter: &mut Painter, render_ctx: RenderCtx) {
        self.render_fn.as_ref()(painter, render_ctx);
    }
}

impl AsAny for GroupSeparator {
    #[inline]
    fn as_any(&self) -> &dyn tlib::prelude::Any {
        self
    }

    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn tlib::prelude::Any {
        self
    }

    #[inline]
    fn as_any_boxed(self: Box<Self>) -> Box<dyn tlib::prelude::Any> {
        self
    }
}

fn default_separator_render(painter: &mut Painter, render_ctx: RenderCtx) {}
