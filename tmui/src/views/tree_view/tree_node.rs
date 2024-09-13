use super::{tree_store::TreeStore, tree_view_object::TreeViewObject, TreeView};
use crate::views::cell::cell_index::CellIndex;
use crate::views::cell::cell_render::CellRender;
use crate::views::cell::Cell;
use crate::views::node::node_render::NodeRender;
use crate::views::node::{RenderCtx, Status};
use crate::{application::is_ui_thread, prelude::*};
use crate::{graphics::painter::Painter, views::tree_view::tree_store::TreeStoreSignals};
use log::warn;
use std::{ptr::NonNull, sync::atomic::Ordering};
use tlib::global::SemanticExt;
use tlib::nonnull_ref;
use tlib::{
    figure::Rect,
    nonnull_mut,
    types::StaticType,
    values::{FromValue, ToValue},
};

#[allow(clippy::vec_box)]
pub struct TreeNode {
    pub(crate) store: ObjectId,
    id: ObjectId,

    parent: Option<NonNull<TreeNode>>,
    is_root: bool,
    extensible: bool,
    expanded: bool,
    removed: bool,
    children: Vec<Box<TreeNode>>,
    children_id_holder: Vec<ObjectId>,
    idx: usize,
    level: i32,
    status: Status,

    cells: Vec<Cell>,
    node_render: NodeRender,
}

/// Implement the [Send] trait to enable the creation of a TreeNode from another thread.
unsafe impl Send for TreeNode {}

impl TreeNode {
    pub fn create(store_id: ObjectId, level: i32, obj: &dyn TreeViewObject) -> Box<TreeNode> {
        let mut node = Self::create_from_obj(obj, store_id);

        node.level = level;

        node
    }

    #[inline]
    pub fn children(&self) -> &[Box<TreeNode>] {
        self.children.as_ref()
    }

    #[inline]
    pub fn children_mut(&mut self) -> &mut [Box<TreeNode>] {
        self.children.as_mut()
    }

    pub fn add_node(&mut self, obj: &dyn TreeViewObject) -> Option<&mut TreeNode> {
        if self.removed {
            return None;
        }
        if !self.extensible {
            return None;
        }
        let node = Self::create_from_obj(obj, self.store);

        self.add_node_inner(node)
    }

    pub fn add_node_directly(&mut self, mut node: Box<TreeNode>) -> Option<&mut TreeNode> {
        if self.removed {
            return None;
        }
        node.store = self.store;

        self.add_node_directly_inner(node)
    }

    pub fn get_value<T: 'static + StaticType + FromValue>(&self, cell_idx: impl CellIndex) -> Option<T> {
        self.cells
            .get(cell_idx.index())
            .or_else(|| {
                warn!("Undefined cell of tree view node, cell index: {:?}", cell_idx);
                None
            })
            .and_then(|cell| {
                if !T::static_type().is_a(cell.type_()) {
                    warn!(
                        "Value type mismatched of cell, expected: {:?}, get: {:?} ",
                        cell.type_().name(),
                        T::static_type().name()
                    );
                    return None;
                }

                Some(cell.value().get::<T>())
            })
    }

    pub fn set_value<T: StaticType + ToValue>(&mut self, cell_idx: impl CellIndex, val: T) {
        if let Some(cell) = self.cells.get_mut(cell_idx.index()) {
            if !T::static_type().is_a(cell.type_()) {
                warn!(
                    "Value type mismatched of cell, expected: {:?}, get: {:?} ",
                    cell.type_().name(),
                    T::static_type().name()
                );
                return;
            }

            cell.set_value(val.to_value());

            if is_ui_thread() && cell.is_render_cell() {
                self.notify_update();
            }
        } else {
            warn!("Undefined cell of tree view node, cell index: {:?}", cell_idx);
        }
    }

    pub fn get_cell_render(&self, cell_idx: usize) -> Option<&dyn CellRender> {
        self.cells
            .get(cell_idx)
            .or_else(|| {
                warn!("Undefined cell of tree view node, cell index: {}", cell_idx);
                None
            })
            .and_then(|cell| cell.get_render())
    }

    pub fn get_cell_render_mut(&mut self, cell_idx: usize) -> Option<&mut dyn CellRender> {
        self.cells
            .get_mut(cell_idx)
            .or_else(|| {
                warn!("Undefined cell of tree view node, cell index: {}", cell_idx);
                None
            })
            .and_then(|cell| cell.get_render_mut())
    }

    #[inline]
    pub fn id(&self) -> ObjectId {
        self.id
    }

    #[inline]
    pub fn level(&self) -> i32 {
        self.level
    }

    #[inline]
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    #[inline]
    pub fn notify_update(&self) {
        let store = TreeStore::store_ref(self.store).unwrap();
        emit!(store.notify_update())
    }

    #[inline]
    pub fn is_extensible(&self) -> bool {
        self.extensible
    }

    #[inline]
    pub fn is_expanded(&self) -> bool {
        self.expanded
    }

    #[inline]
    pub fn is_hovered(&self) -> bool {
        self.status.contains(Status::Hovered)
    }

    #[inline]
    pub fn is_selected(&self) -> bool {
        self.status.contains(Status::Selected)
    }

    #[inline]
    pub fn remove(&mut self) {
        self.removed = true;
        let _hold = self.remove_node_inner();

        let child_id = self.id();
        let child_expanded = self.is_expanded();
        let mut parent = self.get_parent();
        let mut deleted = vec![child_id];
        deleted.extend_from_slice(self.get_children_ids());
        TreeStore::store_mut(self.store).unwrap().node_deleted(
            nonnull_mut!(parent),
            child_id,
            child_expanded,
            deleted,
        );
    }

    #[inline]
    pub fn clear(&mut self) {
        for c in self.children.iter_mut() {
            c.remove()
        }
    }

    #[inline]
    pub fn get_view(&mut self) -> &mut TreeView {
        TreeStore::store_mut(self.store)
            .unwrap()
            .get_view()
            .downcast_mut::<TreeView>()
            .unwrap()
    }

    #[inline]
    pub fn get_view_ref(&self) -> &TreeView {
        TreeStore::store_ref(self.store)
            .unwrap()
            .get_view_ref()
            .downcast_ref::<TreeView>()
            .unwrap()
    }

    /// Sort the node if the [`sort_proxy`](TreeStore::sort_proxy) is not `None`.
    ///
    /// @params </br>
    /// recur:  
    ///     - [`true`] sort the node and it's chidlren nodes.
    ///     - [`false`] sort the node only.
    #[inline]
    pub fn sort(&mut self, recur: bool) {
        self.sort_inner(recur, true, None);
    }

    /// If the node is extensible, shuffle the `expanded` status of node.
    #[inline]
    pub fn shuffle_expand(&mut self) {
        if !self.extensible {
            return;
        }
        self.expanded = !self.expanded;

        self.notify_grand_child_expand(self.expanded, self.id, self.children_id_holder.clone());

        if is_ui_thread() {
            TreeStore::store_mut(self.store)
                .unwrap()
                .node_expanded(self);
        }
    }

    /// Get the area of TreeNode with the specific coordinate.
    /// 
    /// If the node is not in the screen image, return `None`
    pub fn rect(&self, coord: Coordinate) -> Option<Rect> {
        let (image, y_offset) = TreeStore::store_ref(self.store).unwrap().get_image();
        let mut idx = 0;
        let mut find = false;
        for (i, node) in image.iter().enumerate() {
            if nonnull_ref!(node).id == self.id {
                idx = i;
                find = true;
                break;
            }
        }

        if find {
            let view = self
                .get_view_ref()
                .downcast_ref::<TreeView>()
                .unwrap()
                .get_image();
            let view_rect = view.contents_rect(Some(coord));
            let line_height = view.line_height();
            let line_spacing = view.line_spacing();

            let y_start = view_rect.y()
                - (y_offset as f32 / 10. * (line_height + line_spacing) as f32) as i32;
            let height = line_height + line_spacing;
            let y = y_start + idx as i32 * (height);

            Some(Rect::new(view_rect.x(), y, view_rect.width(), height))
        } else {
            None
        }
    }
}

impl TreeNode {
    #[inline]
    pub(crate) fn empty() -> Self {
        Self {
            id: 0,
            store: 0,
            parent: None,
            is_root: true,
            extensible: true,
            expanded: true,
            removed: false,
            children: vec![],
            children_id_holder: vec![],
            idx: 0,
            level: -1,
            status: Status::empty(),
            cells: vec![],
            node_render: NodeRender::default(),
        }
    }

    #[inline]
    pub(crate) fn create_from_obj(obj: &dyn TreeViewObject, store: ObjectId) -> Box<Self> {
        Self {
            id: TreeStore::store_ref(store)
                .unwrap()
                .id_increment
                .fetch_add(1, Ordering::Acquire),
            store,
            parent: None,
            is_root: false,
            extensible: obj.extensible(),
            expanded: true,
            removed: false,
            children: vec![],
            children_id_holder: vec![],
            idx: 0,
            level: 0,
            status: Status::empty(),
            cells: obj.cells(),
            node_render: obj.node_render(),
        }
        .boxed()
    }

    #[inline]
    pub(crate) fn render_node(
        &self,
        painter: &mut Painter,
        render_ctx: RenderCtx,
        ident_length: i32,
    ) {
        let mut geometry: FRect = render_ctx.geometry;

        self.node_render
            .render(painter, render_ctx, self.status);

        let tl = geometry.top_left();
        geometry.set_x(tl.x() + (ident_length * self.level) as f32);
        geometry.set_width(geometry.width() - (ident_length * self.level) as f32);

        let gapping = geometry.width() / self.render_cell_size() as f32;
        let mut offset = geometry.x();

        for cell in self.cells.iter() {
            if let Some(cell_render) = cell.get_render() {
                let mut cell_rect = geometry;

                cell_rect.set_x(offset);
                if let Some(width) = cell_render.width() {
                    cell_rect.set_width(width as f32);
                } else {
                    cell_rect.set_width(gapping);
                }
                if let Some(height) = cell_render.height() {
                    cell_rect.set_height(height as f32);
                }

                offset += cell_rect.width();

                cell.render_cell(painter, cell_rect);
            }
        }
    }

    #[inline]
    pub(crate) fn get_parent(&self) -> Option<NonNull<TreeNode>> {
        self.parent
    }

    #[inline]
    pub(crate) fn get_children_ids(&self) -> &Vec<ObjectId> {
        &self.children_id_holder
    }

    #[inline]
    pub(crate) fn render_cell_size(&self) -> usize {
        let mut size = 0;
        self.cells.iter().for_each(|cell| {
            if cell.get_render().is_some() {
                size += 1;
            }
        });
        size
    }

    pub(crate) fn add_node_inner(&mut self, mut node: Box<TreeNode>) -> Option<&mut TreeNode> {
        if !self.extensible {
            return None;
        }

        debug_assert!(node.store != 0);

        node.parent = NonNull::new(self);
        node.idx = self.children.len();
        node.level = self.level + 1;

        let store = TreeStore::store_mut(self.store).unwrap();
        store.add_node_cache(&mut node);

        let id = node.id;
        let ids = vec![id];

        self.children.push(node);

        if !self.is_expanded() && is_ui_thread() {
            self.shuffle_expand()
        }

        // Every node has hold their children/grand children's ids.
        // Assuming existence parent `Node_1` hold children id (2,3,4),
        //
        // Case 1:
        // `Node_2`,`Node_3`,`Node_4` are chidlren of `Node_1`,
        // Now add new `Node_5` to `Node_2`, we expect `Node_1` (2,5,3,4)
        //
        // Case 2:
        // `Node_2`, `Node_4` are children of `Node_1`, `Node_3` is child of `Node_2`,
        // Now add new `Node_5` to `Node_2`, we expect `Node_1` (2,3,5,4)
        let anchor = if let Some(last) = self.children_id_holder.last() {
            *last
        } else {
            self.id
        };
        self.children_id_holder.push(id);
        self.notify_grand_child_add(anchor, &ids);

        if is_ui_thread() {
            let mut idx = store.node_added(self, id, &ids);

            if idx != usize::MAX {
                if let Some(id) = self.need_sort() {
                    idx = self.sort_inner(false, true, Some(id)).unwrap();
                }
                let image_len = store.image_len() as i32;
                let buffer_len = store.buffer_len() as i32;
                let scroll_to = (idx as i32 - image_len / 2)
                    .max(0)
                    .min((buffer_len - store.get_window_lines()).max(0));
                store.scroll_to(scroll_to, true);
            }
        }

        store.get_node_mut(id)
    }

    #[inline]
    pub(crate) fn clear_directly(&mut self) {
        self.children.clear();
        self.children_id_holder.clear();
    }

    pub(crate) fn add_node_directly_inner(
        &mut self,
        mut node: Box<TreeNode>,
    ) -> Option<&mut TreeNode> {
        if !self.extensible {
            return None;
        }

        debug_assert!(node.store != 0);

        node.parent = NonNull::new(self);
        node.idx = self.children.len();

        let store = TreeStore::store_mut(self.store).unwrap();
        store.add_node_cache(&mut node);

        let id = node.id;
        let mut ids = vec![id];
        ids.extend_from_slice(&node.children_id_holder);

        self.children.push(node);

        if !self.is_expanded() && is_ui_thread() {
            self.shuffle_expand()
        }

        let anchor = if let Some(last) = self.children_id_holder.last() {
            *last
        } else {
            self.id
        };
        self.children_id_holder.extend_from_slice(&ids);
        self.notify_grand_child_add(anchor, &ids);

        if is_ui_thread() {
            let mut idx = store.node_added(self, id, &ids);

            if idx != usize::MAX {
                if let Some(id) = self.need_sort() {
                    idx = self.sort_inner(false, true, Some(id)).unwrap();
                }
                let image_len = store.image_len() as i32;
                let buffer_len = store.buffer_len() as i32;
                let scroll_to = (idx as i32 - image_len / 2)
                    .max(0)
                    .min((buffer_len - store.get_window_lines()).max(0));
                store.scroll_to(scroll_to, true);
            }
        }

        store.get_node_mut(id)
    }

    pub(crate) fn notify_grand_child_add(&mut self, anchor: ObjectId, id: &Vec<ObjectId>) {
        if self.parent.is_some() {
            let parent = nonnull_mut!(self.parent);

            if is_ui_thread() {
                let mut idx = 0;
                for &i in parent.children_id_holder.iter() {
                    idx += 1;
                    if i == anchor {
                        break;
                    }
                }

                parent.children_id_holder.splice(idx..idx, id.clone());
            } else {
                parent.children_id_holder.extend_from_slice(id);
            }

            parent.notify_grand_child_add(anchor, id);
        }
    }

    pub(crate) fn notify_grand_child_remove(&mut self, ids: &Vec<ObjectId>) {
        if self.parent.is_some() {
            let parent = nonnull_mut!(self.parent);

            if parent.children_id_holder.is_empty() {
                return;
            }

            let mut idx = 0;
            for (i, c) in parent.children_id_holder.iter().enumerate() {
                if *c == ids[0] {
                    idx = i;
                    break;
                }
            }

            parent.children_id_holder.drain(idx..idx + ids.len());

            parent.notify_grand_child_remove(ids);
        }
    }

    pub(crate) fn notify_grand_child_expand(
        &mut self,
        expand: bool,
        id: ObjectId,
        ids: Vec<ObjectId>,
    ) {
        if self.parent.is_none() {
            return;
        }
        let parent = nonnull_mut!(self.parent);

        let mut idx = 0;
        for (i, c) in parent.children_id_holder.iter().enumerate() {
            if *c == id {
                idx = i + 1;
                break;
            }
        }

        if expand {
            parent.children_id_holder.splice(idx..idx, ids.clone());
        } else {
            parent.children_id_holder.drain(idx..idx + ids.len());
        }

        parent.notify_grand_child_expand(expand, id, ids);
    }

    pub(crate) fn notify_grand_chlild_update(&mut self, anchor: ObjectId, ids: Vec<ObjectId>) {
        if self.parent.is_none() {
            return;
        }
        let parent = nonnull_mut!(self.parent);

        let mut idx = 0;
        for (i, c) in parent.children_id_holder.iter().enumerate() {
            if *c == anchor {
                idx = i + 1;
                break;
            }
        }

        parent
            .children_id_holder
            .splice(idx..idx + ids.len(), ids.clone());

        parent.notify_grand_chlild_update(anchor, ids)
    }

    pub(crate) fn remove_node_inner(&mut self) -> Option<Box<TreeNode>> {
        if self.parent.is_some() {
            let parent = nonnull_mut!(self.parent);

            debug_assert!(parent.children.len() > self.idx);
            let hold = parent.children.remove(self.idx);

            for (i, c) in parent.children.iter_mut().enumerate() {
                c.idx = i;
            }

            let mut ids_to_remove = vec![self.id()];
            ids_to_remove.extend_from_slice(&self.children_id_holder);

            let store = TreeStore::store_mut(self.store).unwrap();
            for &id in ids_to_remove.iter() {
                store.remove_node_cache(id)
            }

            if self.expanded {
                self.notify_grand_child_remove(&ids_to_remove);
            } else {
                self.notify_grand_child_remove(&vec![self.id()]);
            }

            return Some(hold);
        }
        None
    }

    #[inline]
    pub(crate) fn add_status(&mut self, status: Status) {
        self.status.insert(status)
    }

    #[inline]
    pub(crate) fn remove_status(&mut self, status: Status) {
        self.status.remove(status)
    }

    pub(crate) fn sort_inner(
        &mut self,
        recur: bool,
        upward: bool,
        focus: Option<ObjectId>,
    ) -> Option<usize> {
        if self.children.is_empty() {
            return None;
        }

        if let Some(ref sort_proxy) = TreeStore::store_ref(self.store)
            .expect("Invalid store id.")
            .sort_proxy
        {
            if recur {
                self.children.iter_mut().for_each(|node| {
                    node.sort_inner(recur, false, None);
                });
            }

            self.children.sort_by(|a, b| sort_proxy.cmp(a, b));

            let mut children_ids: Vec<ObjectId> = vec![];
            self.children
                .iter_mut()
                .enumerate()
                .for_each(|(idx, node)| {
                    node.idx = idx;
                    children_ids.push(node.id);
                    if node.is_expanded() {
                        children_ids.extend(&node.children_id_holder);
                    }
                });
            self.children_id_holder = children_ids;

            if upward {
                self.notify_grand_chlild_update(self.id, self.children_id_holder.clone());

                if is_ui_thread() {
                    TreeStore::store_mut(self.store)
                        .unwrap()
                        .node_updated(self, focus)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn need_sort(&self) -> Option<ObjectId> {
        if self.children.len() <= 1 {
            return None;
        }
        if let Some(ref sort_proxy) = TreeStore::store_ref(self.store)
            .expect("Invalid store")
            .sort_proxy
        {
            let last = &self.children[self.children.len() - 2];
            let added = &self.children[self.children.len() - 1];
            if sort_proxy.cmp(last, added) == std::cmp::Ordering::Greater {
                Some(added.id)
            } else {
                None
            }
        } else {
            None
        }
    }
}
