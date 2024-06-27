use std::sync::atomic::Ordering;
use tlib::{
    global::AsAny,
    object::{IdGenerator, ObjectId},
};

use super::{
    list_item::{ItemType, ListItem, RenderCtx},
    list_node::ListNode,
    list_view_object::ListViewObject,
    Painter,
};

#[derive(Default)]
pub struct ListGroup {
    id: ObjectId,
    nodes: Vec<ListNode>,
}

impl ListGroup {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn add_node(&mut self, obj: &dyn ListViewObject) {
        self.nodes.push(ListNode::create_from_obj(obj))
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl ListGroup {
    #[inline]
    pub(crate) fn set_id(&mut self, id_increment: &IdGenerator) {
        self.id = id_increment.fetch_add(1, Ordering::SeqCst);

        for node in self.nodes.iter_mut() {
            node.set_id(id_increment.fetch_add(1, Ordering::SeqCst));
        }
    }
}

impl ListItem for ListGroup {
    #[inline]
    fn id(&self) -> ObjectId {
        self.id
    }

    #[inline]
    fn item_type(&self) -> ItemType {
        ItemType::Group
    }

    fn render(&self, painter: &mut Painter, mut render_ctx: RenderCtx) {
        let mut geometry = render_ctx.geometry;
        let mut offset = geometry.y();

        for node in self.nodes.iter() {
            geometry.set_y(offset);
            render_ctx.geometry = geometry;
            offset += render_ctx.line_height + render_ctx.line_spacing;

            node.render(painter, render_ctx);
        }
    }
}

impl AsAny for ListGroup {
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
