use std::sync::atomic::Ordering;
use tlib::{
    figure::Rect,
    global::AsAny,
    object::{IdGenerator, ObjectId},
};

use super::{
    list_item::{ItemType, ListItem},
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

    fn render(&mut self, painter: &mut Painter, geometry: Rect) {
        // TODO: Clac the geometry of each node.
        for node in self.nodes.iter_mut() {
            node.render(painter, geometry)
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
