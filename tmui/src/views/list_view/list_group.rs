use super::{list_node::ListNode, list_view_object::ListViewObject};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default)]
pub struct ListGroup {
    #[derivative(Default(value = "Some(vec![])"))]
    nodes: Option<Vec<ListNode>>,
}

impl ListGroup {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn add_node(&mut self, obj: &dyn ListViewObject) {
        self.nodes.as_mut().unwrap().push(ListNode::from(obj))
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.nodes.as_ref().unwrap().len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl ListGroup {
    #[inline]
    pub(crate) fn take_nodes(&mut self) -> Vec<ListNode> {
        self.nodes.take().unwrap()
    }
}
