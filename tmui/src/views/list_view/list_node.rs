use super::{
    list_item::{ItemType, ListItem},
    list_view_object::ListViewObject, Painter,
};
use crate::views::{cell::Cell, node::node_render::NodeRender};
use tlib::{figure::Rect, global::AsAny, object::ObjectId};

pub struct ListNode {
    id: ObjectId,

    cells: Vec<Cell>,
    node_render: NodeRender,
}

impl ListNode {}

impl ListNode {
    #[inline]
    pub(crate) fn create_from_obj(obj: &dyn ListViewObject) -> Self {
        Self {
            id: 0,
            cells: obj.cells(),
            node_render: obj.node_render(),
        }
    }

    #[inline]
    pub(crate) fn set_id(&mut self, id: ObjectId) {
        self.id = id;
    }
}

impl ListItem for ListNode {
    #[inline]
    fn id(&self) -> ObjectId {
        self.id
    }

    #[inline]
    fn item_type(&self) -> ItemType {
        ItemType::Node
    }

    fn render(&mut self, painter: &mut Painter, geometry: Rect) {
        todo!()
    }
}

impl AsAny for ListNode {
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
