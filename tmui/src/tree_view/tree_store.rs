use super::{
    tree_node::{Status, TreeNode},
    tree_view_object::TreeViewObject,
};
use crate::prelude::*;
use log::warn;
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    ptr::NonNull,
    sync::atomic::{AtomicPtr, Ordering},
};
use tlib::{
    global::SemanticExt,
    nonnull_mut, nonnull_ref,
    object::{ObjectId, ObjectOperation, ObjectSubclass},
    signals,
};

#[extends(Object, ignore_default = true)]
pub struct TreeStore {
    root: Box<TreeNode>,

    nodes_buffer: Vec<Option<NonNull<TreeNode>>>,
    nodes_cache: HashMap<ObjectId, Option<NonNull<TreeNode>>>,

    window_lines: i32,
    current_line: i32,

    hovered_node: Option<NonNull<TreeNode>>,
    selected_node: Option<NonNull<TreeNode>>,
}

pub trait TreeStoreSignals: ActionExt {
    signals!(
        TreeStore:

        notify_update();

        notify_update_rect();
    );
}
impl TreeStoreSignals for TreeStore {}

impl Default for TreeStore {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl TreeStore {
    #[inline]
    pub(crate) fn store_map() -> &'static mut HashMap<ObjectId, AtomicPtr<TreeStore>> {
        static mut STORE_MAP: Lazy<HashMap<ObjectId, AtomicPtr<TreeStore>>> =
            Lazy::new(|| HashMap::new());
        unsafe { &mut STORE_MAP }
    }

    #[inline]
    pub(crate) fn store_ref(id: ObjectId) -> Option<&'static TreeStore> {
        Self::store_map()
            .get(&id)
            .and_then(|ptr| unsafe { ptr.load(Ordering::Acquire).as_ref() })
    }

    #[inline]
    pub(crate) fn store_mut(id: ObjectId) -> Option<&'static mut TreeStore> {
        Self::store_map()
            .get(&id)
            .and_then(|ptr| unsafe { ptr.load(Ordering::Acquire).as_mut() })
    }

    /// add the child node to the node with the specified `parent_id`.
    ///
    /// if success, return the mutable reference of the child node.
    #[inline]
    pub fn add_node(
        &mut self,
        parent_id: ObjectId,
        obj: &dyn TreeViewObject,
    ) -> Option<&mut TreeNode> {
        if self.nodes_cache.contains_key(&parent_id) {
            let mut tree_node = TreeNode::create_from_obj(obj);

            tree_node.store = self.id();

            nonnull_mut!(self.nodes_cache.get_mut(&parent_id).unwrap())
                .add_node_inner(tree_node.boxed())
        } else {
            warn!(
                "`TreeView` add node failed, can not find the parent node, id = {}",
                parent_id
            );
            None
        }
    }

    pub fn remove_node(&mut self, id: ObjectId) {
        if self.nodes_cache.contains_key(&id) {
            nonnull_mut!(self.nodes_cache.get_mut(&id).unwrap()).remove_node_inner()
        } else {
            warn!(
                "`TreeView` remove node failed, can not find the node, id = {}",
                id
            );
        }
    }

    #[inline]
    pub fn root(&self) -> &TreeNode {
        self.root.as_ref()
    }

    #[inline]
    pub fn root_mut(&mut self) -> &mut TreeNode {
        self.root.as_mut()
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

        let mut root = Box::new(TreeNode::empty());
        nodes_map.insert(root.id(), NonNull::new(root.as_mut()));

        let mut store = Self {
            object: Default::default(),
            root,
            nodes_buffer: vec![],
            nodes_cache: nodes_map,
            window_lines: 0,
            current_line: 0,
            hovered_node: None,
            selected_node: None,
        };

        store.root_mut().store = store.id();

        store
    }

    #[inline]
    pub(crate) fn prepare_store(&mut self) {
        Self::store_map().insert(self.id(), AtomicPtr::new(self));
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
    pub(crate) fn get_image(&self) -> &[Option<NonNull<TreeNode>>] {
        let start = self.current_line as usize;
        let end = (start + self.window_lines as usize).min(self.nodes_buffer.len());

        &self.nodes_buffer[start..end]
    }

    #[inline]
    pub(crate) fn image_len(&self) -> usize {
        let start = self.current_line as usize;
        let end = (start + self.window_lines as usize).min(self.nodes_buffer.len());
        end - start
    }

    pub(crate) fn node_added(
        &mut self,
        node: &TreeNode,
        child_id: ObjectId,
        added: &Vec<ObjectId>,
    ) {
        if !self.root().get_children_ids().contains(&child_id) {
            return;
        }

        let children = node.get_children_ids();
        let mut anchor = 0;
        for (i, n) in children.iter().enumerate() {
            if *n == added[0] {
                if i == 0 {
                    anchor = children[0]
                } else {
                    anchor = children[i - 1]
                }
            }
        }

        let mut idx = 0;
        for c in self.nodes_buffer.iter() {
            idx += 1;
            if nonnull_ref!(c).id() == anchor {
                break;
            }
        }

        let insert: Vec<Option<NonNull<TreeNode>>> = added
            .iter()
            .map(|id| *self.nodes_cache.get(id).unwrap())
            .collect();

        self.nodes_buffer.splice(idx..idx, insert);

        emit!(self.notify_update_rect(), idx);
    }

    pub(crate) fn node_expanded(&mut self, node: &TreeNode) {
        if !node.is_extensible() {
            return;
        }
        let expanded = node.is_expanded();
        let children = node.get_children_ids();

        let mut start_idx = 0usize;

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

            start_idx = idx;
        } else {
            if children.len() == 0 {
                return;
            }

            let mut idx = 0;
            for (i, c) in self.nodes_buffer.iter().enumerate() {
                idx = i;

                if nonnull_ref!(c).id() == children[0] {
                    break;
                }
            }

            self.nodes_buffer.drain(idx..idx + children.len());
        }

        emit!(self.notify_update_rect(), start_idx);
    }

    #[inline]
    pub(crate) fn set_window_lines(&mut self, window_lines: i32) {
        self.window_lines = window_lines;
    }

    #[inline]
    pub(crate) fn hover_node(&mut self, idx: usize) {
        if idx >= self.image_len() {
            let mut old_hover = self.hovered_node.take();
            if old_hover.is_some() {
                let node = nonnull_mut!(old_hover);
                if node.is_hovered() {
                    node.set_status(Status::Default);

                    emit!(self.notify_update());
                }
            }
            return;
        }

        let mut node_ptr = self.get_image()[idx];
        let node = nonnull_mut!(node_ptr);
        if node.is_hovered() {
            return;
        }

        if self.hovered_node.is_some() {
            let node = nonnull_mut!(self.hovered_node);
            if !node.is_selected() {
                node.set_status(Status::Default);
            }
        }

        if !node.is_selected() {
            node.set_status(Status::Hovered);
            self.hovered_node = node_ptr;
        }

        emit!(self.notify_update());
    }

    pub(crate) fn click_node(&mut self, idx: usize) {
        if idx >= self.image_len() {
            let mut old_select = self.selected_node.take();
            if old_select.is_some() {
                let node = nonnull_mut!(old_select);
                node.set_status(Status::Default);

                emit!(self.notify_update());
            }
            return;
        }

        let mut node_ptr = self.get_image()[idx];
        let node = nonnull_mut!(node_ptr);

        if self.selected_node.is_some() {
            let node = nonnull_mut!(self.selected_node);
            node.set_status(Status::Default);
        }

        node.set_status(Status::Selected);
        self.selected_node = node_ptr;

        node.node_clicked();
        self.node_expanded(node);

        emit!(self.notify_update());
    }
}

impl ObjectSubclass for TreeStore {
    const NAME: &'static str = "TreeViewStore";
}
impl ObjectImpl for TreeStore {}
