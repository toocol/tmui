use super::{
    list_node::ListNode,
    list_separator::GroupSeparator,
    list_view_object::ListViewObject,
};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default)]
pub struct ListGroup {
    #[derivative(Default(value = "Some(vec![])"))]
    nodes: Option<Vec<ListNode>>,

    #[derivative(Default(value = "Some(GroupSeparator::default())"))]
    separator: Option<GroupSeparator>,
}

impl ListGroup {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn add_node(&mut self, obj: &dyn ListViewObject) {
        self.nodes
            .as_mut()
            .unwrap()
            .push(ListNode::create_from_obj(obj))
    }

    #[inline]
    pub fn separator(&self) -> &GroupSeparator {
        self.separator.as_ref().unwrap()
    }

    #[inline]
    pub fn separator_mut(&mut self) -> &mut GroupSeparator {
        self.separator.as_mut().unwrap()
    }
}

impl ListGroup {
    #[inline]
    pub(crate) fn take_nodes(&mut self) -> Vec<ListNode> {
        self.nodes.take().unwrap()
    }

    #[inline]
    pub(crate) fn take_separator(&mut self) -> GroupSeparator {
        self.separator.take().unwrap()
    }
}
