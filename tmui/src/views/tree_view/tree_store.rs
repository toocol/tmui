use super::{tree_node::TreeNode, tree_view_object::TreeViewObject};
use crate::{prelude::*, views::node::Status};
use log::warn;
use nohash_hasher::IntMap;
use once_cell::sync::Lazy;
use std::{
    ptr::{addr_of_mut, NonNull},
    sync::atomic::{AtomicPtr, Ordering},
};
use tlib::{
    compare::Compare,
    namespace::MouseButton,
    nonnull_mut, nonnull_ref,
    object::{IdGenerator, ObjectId, ObjectOperation, ObjectSubclass},
    signals,
};

#[extends(Object, ignore_default = true)]
pub struct TreeStore {
    view: WidgetHnd,
    root: Box<TreeNode>,

    /// The buffer represent the view of nodes.
    nodes_buffer: Vec<Option<NonNull<TreeNode>>>,
    nodes_cache: IntMap<ObjectId, Option<NonNull<TreeNode>>>,

    window_lines: i32,
    current_line: i32,
    y_offset: i32,

    enterd_node: Option<NonNull<TreeNode>>,
    hovered_node: Option<NonNull<TreeNode>>,
    selected_node: Option<NonNull<TreeNode>>,

    pub(crate) id_increment: IdGenerator,
    pub(crate) sort_proxy: Option<Compare<TreeNode>>,
}

pub trait TreeStoreSignals: ActionExt {
    signals!(
        TreeStore:

        notify_update();

        /// @param [`usize`]
        notify_update_rect(usize);

        /// @param [`usize`]
        buffer_len_changed(usize);

        /// @param [`i32`]
        internal_scroll_value_changed(i32);
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
    pub(crate) fn store_map() -> &'static mut IntMap<ObjectId, AtomicPtr<TreeStore>> {
        static mut STORE_MAP: Lazy<IntMap<ObjectId, AtomicPtr<TreeStore>>> =
            Lazy::new(IntMap::default);
        unsafe { addr_of_mut!(STORE_MAP).as_mut().unwrap() }
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
            let tree_node = TreeNode::create_from_obj(obj, self.id());

            nonnull_mut!(self.nodes_cache.get_mut(&parent_id).unwrap()).add_node_inner(tree_node)
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
            nonnull_mut!(self.nodes_cache.get_mut(&id).unwrap()).remove()
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
        self.nodes_cache.get(&id).map(|n| nonnull_ref!(n))
    }

    #[inline]
    pub fn get_node_mut(&mut self, id: ObjectId) -> Option<&mut TreeNode> {
        self.nodes_cache.get_mut(&id).map(|n| nonnull_mut!(n))
    }

    #[inline]
    pub fn set_sort_proxy(&mut self, compare: Compare<TreeNode>) {
        self.sort_proxy = Some(compare);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.root_mut().clear_directly();

        self.nodes_buffer.clear();
        self.nodes_cache.clear();
        self.enterd_node = None;
        self.hovered_node = None;
        self.selected_node = None;
    }
}

impl TreeStore {
    #[inline]
    pub(crate) fn new() -> Self {
        let mut nodes_map = IntMap::default();

        let mut root = Box::new(TreeNode::empty());
        nodes_map.insert(root.id(), NonNull::new(root.as_mut()));

        let mut store = Self {
            view: None,
            object: Default::default(),
            root,
            nodes_buffer: vec![],
            nodes_cache: nodes_map,
            window_lines: 0,
            current_line: 0,
            y_offset: 0,
            enterd_node: None,
            hovered_node: None,
            selected_node: None,
            id_increment: IdGenerator::new(1),
            sort_proxy: None,
        };

        store.root_mut().store = store.id();

        store
    }

    #[inline]
    pub(crate) fn get_view(&mut self) -> &mut dyn WidgetImpl {
        nonnull_mut!(self.view)
    }

    #[inline]
    pub(crate) fn get_view_ref(&self) -> &dyn WidgetImpl {
        nonnull_ref!(self.view)
    }

    #[inline]
    pub(crate) fn set_view(&mut self, view: WidgetHnd) {
        self.view = view;
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

        if self.enterd_node.is_some() && nonnull_ref!(self.enterd_node).id() == id {
            self.enterd_node = None;
        }

        if self.selected_node.is_some() && nonnull_ref!(self.selected_node).id() == id {
            self.selected_node = None;
        }

        if self.hovered_node.is_some() && nonnull_ref!(self.hovered_node).id() == id {
            self.hovered_node = None;
        }
    }

    #[inline]
    pub(crate) fn y_offset(&self) -> i32 {
        self.y_offset
    }

    #[inline]
    pub(crate) fn get_image(&self) -> (&[Option<NonNull<TreeNode>>], i32) {
        let start = if self.y_offset == 0 {
            self.current_line
        } else {
            (self.current_line - 1).max(0)
        } as usize;

        let end = (start + self.window_lines as usize).min(self.nodes_buffer.len());

        (&self.nodes_buffer[start..end], self.y_offset)
    }

    #[inline]
    pub(crate) fn get_image_mut(&mut self) -> &mut [Option<NonNull<TreeNode>>] {
        let start = self.current_line as usize;
        let end = (start + self.window_lines as usize).min(self.nodes_buffer.len());

        &mut self.nodes_buffer[start..end]
    }

    #[inline]
    pub(crate) fn get_image_node(&mut self, idx: usize) -> Option<&mut TreeNode> {
        let image = self.get_image_mut();
        if idx >= image.len() {
            return None;
        }

        Some(nonnull_mut!(image[idx]))
    }

    #[inline]
    pub(crate) fn get_image_node_ptr(&mut self, idx: usize) -> Option<NonNull<TreeNode>> {
        let image = self.get_image_mut();
        if idx >= image.len() {
            return None;
        }

        image[idx]
    }

    #[inline]
    pub(crate) fn image_len(&self) -> usize {
        let start = self.current_line as usize;
        let end = (start + self.window_lines as usize).min(self.nodes_buffer.len());
        end - start
    }

    #[inline]
    pub(crate) fn buffer_len(&self) -> usize {
        self.nodes_buffer.len()
    }

    /// Add node and it's children to the nodes buffer.
    ///
    /// @param node: the parent node of the added node. <br>
    /// @param child_id: the id of the added node. <br>
    /// @param added: all the ids of added node and it's children's id.
    ///
    /// @return
    /// - `normal`: the index of added node in nodes buffer.
    /// - `usize::MAX`: add failed.         
    pub(crate) fn node_added(
        &mut self,
        node: &TreeNode,
        child_id: ObjectId,
        added: &[ObjectId],
    ) -> usize {
        if !self.root().get_children_ids().contains(&child_id) {
            return usize::MAX;
        }
        if !node.is_expanded() {
            return usize::MAX;
        }

        let children = node.get_children_ids();
        let mut anchor = node.id();
        for (i, n) in children.iter().enumerate() {
            if *n == added[0] {
                if i != 0 {
                    anchor = children[i - 1];
                }
                break;
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

        emit!(self, buffer_len_changed(self.nodes_buffer.len()));
        emit!(self, notify_update_rect(idx));

        // Calculate the index of added node in nodes buffer:
        let mut added_idx = usize::MAX;
        for i in idx..self.nodes_buffer.len() {
            if nonnull_ref!(self.nodes_buffer[i]).id() == child_id {
                added_idx = i;
                break;
            }
        }
        added_idx
    }

    /// Remove node and it's children from the nodes buffer.
    ///
    /// @param node: the parent node of the deleted node. <br>
    /// @param child_id: the id of the deleted node. <br>
    /// @param deleted: all the ids of nodes that should be deleted, container it's children nodes.
    ///
    /// @return
    /// - `normal`: the index of added node in nodes buffer.
    /// - `usize::MAX`: add failed.         
    pub(crate) fn node_deleted(
        &mut self,
        node: &TreeNode,
        child: ObjectId,
        child_expanded: bool,
        deleted: Vec<ObjectId>,
    ) {
        if !node.is_expanded() {
            return;
        }

        let mut idx = 0;
        for c in self.nodes_buffer.iter() {
            if nonnull_ref!(c).id() == child {
                break;
            }
            idx += 1;
        }

        if child_expanded {
            self.nodes_buffer.drain(idx..idx + deleted.len());
        } else {
            self.nodes_buffer.drain(idx..idx + 1);
        }

        emit!(self, buffer_len_changed(self.nodes_buffer.len()));
        emit!(self, notify_update_rect(idx));
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
            if children.is_empty() {
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

        emit!(self, buffer_len_changed(self.nodes_buffer.len()));
        emit!(self, notify_update_rect(start_idx));
    }

    pub(crate) fn node_updated(
        &mut self,
        node: &TreeNode,
        focus: Option<ObjectId>,
    ) -> Option<usize> {
        if !node.is_expanded() {
            return None;
        }

        let mut idx = 0;
        let children = node.get_children_ids();
        if node.id() != 0 {
            for c in self.nodes_buffer.iter() {
                idx += 1;
                if nonnull_ref!(c).id() == node.id() {
                    break;
                }
            }
        }

        let update: Vec<Option<NonNull<TreeNode>>> = children
            .iter()
            .map(|id| *self.nodes_cache.get(id).unwrap())
            .collect();

        let update_len = update.len();
        self.nodes_buffer.splice(idx..idx + update_len, update);
        emit!(self, notify_update_rect(idx));

        focus.map(|focus| {
            for i in idx..idx + update_len {
                if nonnull_ref!(self.nodes_buffer[i]).id() == focus {
                    return i;
                }
            }
            unreachable!()
        })
    }

    #[inline]
    pub(crate) fn set_window_lines(&mut self, window_lines: i32) {
        self.window_lines = window_lines;
    }

    #[inline]
    pub(crate) fn get_window_lines(&self) -> i32 {
        self.window_lines
    }

    #[inline]
    pub(crate) fn hover_node(&mut self, idx: usize) {
        if idx >= self.image_len() {
            let mut old_hover = self.hovered_node.take();
            if old_hover.is_some() {
                let node = nonnull_mut!(old_hover);
                if node.is_hovered() {
                    node.remove_status(Status::Hovered);

                    emit!(self, notify_update());
                }
            }
            return;
        }

        let mut node_ptr = self.get_image().0[idx];
        let node = nonnull_mut!(node_ptr);
        if node.is_hovered() {
            return;
        }

        if self.hovered_node.is_some() {
            let node = nonnull_mut!(self.hovered_node);
            node.remove_status(Status::Hovered);
        }

        node.add_status(Status::Hovered);
        self.hovered_node = node_ptr;

        emit!(self, notify_update());
    }

    pub(crate) fn click_node(&mut self, idx: usize, mouse_button: MouseButton) {
        if idx >= self.image_len() {
            let mut old_select = self.selected_node.take();
            if old_select.is_some() {
                let node = nonnull_mut!(old_select);
                node.remove_status(Status::Selected);

                emit!(self, notify_update());
            }
            return;
        }

        let mut node_ptr = self.get_image().0[idx];
        let node = nonnull_mut!(node_ptr);

        if self.selected_node.is_some() {
            let node = nonnull_mut!(self.selected_node);
            node.remove_status(Status::Selected);
        }

        node.add_status(Status::Selected);
        self.selected_node = node_ptr;

        if mouse_button == MouseButton::LeftButton {
            node.shuffle_expand();
        }

        emit!(self, notify_update());
    }

    /// @param `internal`
    /// - true: The view scrolling triggered internally in TreeView
    ///         requires notifying the scroll bar to change the value.
    ///
    /// @return `true` if scroll value has updated, should update the image.
    #[inline]
    pub(crate) fn scroll_to(&mut self, mut value: i32, internal: bool) -> bool {
        if internal {
            value *= 10;
        }

        let move_to = value / 10;
        if move_to < 0 || move_to > self.nodes_buffer.len() as i32 {
            return false;
        }

        self.y_offset = value % 10;
        if self.current_line == move_to {
            return true;
        }

        // if value was 0, scroll to the begining, first node index was 0
        // scroll to 1, first node index was 1
        self.current_line = move_to;

        if internal {
            emit!(self, internal_scroll_value_changed(value));
        }

        true
    }

    #[inline]
    pub(crate) fn get_entered_node(&self) -> Option<NonNull<TreeNode>> {
        self.enterd_node
    }

    #[inline]
    pub(crate) fn set_entered_node(&mut self, node: &mut TreeNode) {
        self.enterd_node = NonNull::new(node)
    }

    #[inline]
    pub(crate) fn get_nodes_buffer_len(&self) -> usize {
        self.nodes_buffer.len()
    }
}

impl ObjectSubclass for TreeStore {
    const NAME: &'static str = "TreeViewStore";
}
impl ObjectImpl for TreeStore {}
