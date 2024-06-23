use crate::views::{cell::Cell, node::node_render::NodeRender};

pub trait ListViewObject {
    /// The data cell of [`ListNode`](super::list_node::ListNode) represented by the struct which implemented TreeViewObject.
    fn cells(&self) -> Vec<Cell>;

    /// Get the [`NodeRender`]
    fn node_render(&self) -> NodeRender;
}
