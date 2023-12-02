use super::cell::Cell;

pub trait TreeViewObject {
    /// The data cell of [`TreeNode`](super::tree_node::TreeNode) represented by the struct which implemented TreeViewObject. 
    fn cells(&self) -> Vec<Cell>;

    /// Whether the [`TreeNode`](super::tree_node::TreeNode) represented by the struct which 
    /// implemented [`TreeViewObject`] have child nodes or not.
    fn extensible(&self) -> bool;
}
