use super::{tree_node::TreeNode, tree_view_object::TreeViewObject};
use crate::prelude::*;
use log::warn;
use std::{collections::HashMap, ptr::NonNull};
use tlib::{
    nonnull_mut, nonnull_ref,
    object::{ObjectId, ObjectOperation, ObjectSubclass},
};

#[extends(Object, ignore_default = true)]
pub struct TreeStore {
    root: TreeNode,

    nodes_buffer: Vec<Option<NonNull<TreeNode>>>,
    nodes_cache: HashMap<ObjectId, Option<NonNull<TreeNode>>>,

    window_lines: i32,
    current_line: i32,
}

impl Default for TreeStore {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl TreeStore {
    /// add the child node to the node with the specified `parent_id`.
    #[inline]
    pub fn add_node(&mut self, parent_id: ObjectId, obj: &dyn TreeViewObject) {
        if self.nodes_cache.contains_key(&parent_id) {
            let mut tree_node = TreeNode::create_from_obj(obj);

            tree_node.store = NonNull::new(self);

            nonnull_mut!(self.nodes_cache.get_mut(&parent_id).unwrap()).add_node_inner(tree_node);
        } else {
            warn!(
                "`TreeView` add node failed, can not find the parent node, id = {}",
                parent_id
            );
        }
    }

    #[inline]
    pub fn root(&self) -> &TreeNode {
        &self.root
    }

    #[inline]
    pub fn root_mut(&mut self) -> &mut TreeNode {
        &mut self.root
    }

    #[inline]
    pub fn get_node(&self, id: ObjectId) -> Option<&TreeNode> {
        self.nodes_cache
            .get(&id)
            .and_then(|n| Some(nonnull_ref!(n)))
    }

    #[inline]
    pub fn get_node_mut(&mut self, id: ObjectId) -> Option<&mut TreeNode> {
        self.nodes_cache
            .get_mut(&id)
            .and_then(|n| Some(nonnull_mut!(n)))
    }
}

impl TreeStore {
    #[inline]
    pub(crate) fn new() -> Self {
        let mut nodes_map = HashMap::new();

        let mut root = TreeNode::empty();
        nodes_map.insert(root.id(), NonNull::new(&mut root));

        let mut store = Self {
            object: Default::default(),
            root,
            nodes_buffer: vec![],
            nodes_cache: nodes_map,
            window_lines: 0,
            current_line: 0,
        };

        store.root_mut().store = NonNull::new(&mut store);

        store
    }

    #[inline]
    pub(crate) fn add_node_cache(&mut self, node: &mut TreeNode) {
        self.nodes_cache.insert(node.id(), NonNull::new(node));
    }

    #[inline]
    pub(crate) fn remove_node_cache(&mut self, id: ObjectId) {
        self.nodes_cache.remove(&id);
    }

    #[inline]
    pub(crate) fn initialize_buffer(&mut self) {
        let mut buffer = vec![];

        self.root_mut().initialize_buffer(&mut buffer);

        self.nodes_buffer = buffer;
    }

    #[inline]
    pub(crate) fn get_image(&self) -> Vec<Option<NonNull<TreeNode>>> {
        let mut image = vec![];

        let start = self.current_line as usize;
        let end = (start + self.window_lines as usize).min(self.nodes_buffer.len());

        image.extend_from_slice(&self.nodes_buffer[start..end]);

        image
    }

    pub(crate) fn node_expanded(&mut self, node: &TreeNode) {
        let expanded = node.is_expanded();
        let children = node.get_children_ids();

        if expanded {
            let mut idx = 0;
            for c in self.nodes_buffer.iter() {
                idx += 1;
                if nonnull_ref!(c).id() == node.id() {
                    break;
                }
            }

            let insert: Vec<Option<NonNull<TreeNode>>> = children
                .iter()
                .map(|id| *self.nodes_cache.get(id).unwrap())
                .collect();

            self.nodes_buffer.splice(idx..idx, insert);
        } else {

            self.nodes_buffer.retain(|c| {
                if children.contains(&nonnull_ref!(c).id()) {
                    return false;
                }
                true
            });

        }
    }
}

impl ObjectSubclass for TreeStore {
    const NAME: &'static str = "TreeViewStore";
}
impl ObjectImpl for TreeStore {}
