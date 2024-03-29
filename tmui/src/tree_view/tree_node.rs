use super::{
    cell::{cell_render::CellRender, Cell},
    node_render::NodeRender,
    tree_store::TreeStore,
    tree_view_object::TreeViewObject,
};
use crate::{application::is_ui_thread, prelude::*};
use crate::{graphics::painter::Painter, tree_view::tree_store::TreeStoreSignals};
use log::warn;
use std::{ptr::NonNull, sync::atomic::Ordering};
use tlib::{
    figure::Rect,
    global::SemanticExt,
    nonnull_mut,
    object::IdGenerator,
    types::StaticType,
    values::{FromValue, ToValue},
};

static TREE_NODE_ID_INCREMENT: IdGenerator = IdGenerator::new(1);

pub struct TreeNode {
    id: ObjectId,
    pub(crate) store: ObjectId,

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
        let mut node = Self::create_from_obj(obj);

        node.store = store_id;
        node.level = level;

        Box::new(node)
    }

    pub fn add_node(&mut self, obj: &dyn TreeViewObject) -> Option<&mut TreeNode> {
        if self.removed {
            return None;
        }
        if !self.extensible {
            return None;
        }
        let mut node = Self::create_from_obj(obj);

        node.store = self.store;

        self.add_node_inner(node.boxed())
    }

    pub fn add_node_directly(&mut self, mut node: Box<TreeNode>) -> Option<&mut TreeNode> {
        if self.removed {
            return None;
        }
        node.store = self.store;

        self.add_node_directly_inner(node)
    }

    pub fn get_value<T: 'static + StaticType + FromValue>(&self, cell_idx: usize) -> Option<T> {
        self.cells
            .get(cell_idx)
            .or_else(|| {
                warn!("Undefined cell of tree view node, cell index: {}", cell_idx);
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

    pub fn set_value<T: StaticType + ToValue>(&mut self, cell_idx: usize, val: T) {
        if let Some(cell) = self.cells.get_mut(cell_idx) {
            if !T::static_type().is_a(cell.type_()) {
                warn!(
                    "Value type mismatched of cell, expected: {:?}, get: {:?} ",
                    cell.type_().name(),
                    T::static_type().name()
                );
                return;
            }

            cell.set_value(val.to_value())
        } else {
            warn!("Undefined cell of tree view node, cell index: {}", cell_idx);
        }
    }

    pub fn get_cell_render(&self, cell_idx: usize) -> Option<&dyn CellRender> {
        self.cells
            .get(cell_idx)
            .or_else(|| {
                warn!("Undefined cell of tree view node, cell index: {}", cell_idx);
                None
            })
            .and_then(|cell| Some(cell.get_render()))
    }

    pub fn get_cell_render_mut(&mut self, cell_idx: usize) -> Option<&mut dyn CellRender> {
        self.cells
            .get_mut(cell_idx)
            .or_else(|| {
                warn!("Undefined cell of tree view node, cell index: {}", cell_idx);
                None
            })
            .and_then(|cell| Some(cell.get_render_mut()))
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
        self.status == Status::Hovered
    }

    #[inline]
    pub fn is_selected(&self) -> bool {
        self.status == Status::Selected
    }

    #[inline]
    pub fn remove(&mut self) {
        self.removed = true;
        let _hold = self.remove_node_inner();

        let child_id = self.id();
        let child_expanded = self.is_expanded();
        let mut parent = self.get_parent();
        let mut deleted = vec![child_id];
        deleted.extend_from_slice(&self.get_children_ids());
        TreeStore::store_mut(self.store).unwrap().node_deleted(
            nonnull_mut!(parent),
            child_id,
            child_expanded,
            deleted,
        );
    }
}

impl TreeNode {
    #[inline]
    pub(crate) fn empty() -> Self {
        Self {
            id: TREE_NODE_ID_INCREMENT.fetch_add(1, Ordering::Acquire),
            store: 0,
            parent: None,
            is_root: true,
            extensible: true,
            expanded: true,
            removed: false,
            children: vec![],
            children_id_holder: vec![],
            idx: 0,
            level: 0,
            status: Status::Default,
            cells: vec![],
            node_render: NodeRender::default(),
        }
    }

    #[inline]
    pub(crate) fn create_from_obj(obj: &dyn TreeViewObject) -> Self {
        Self {
            id: TREE_NODE_ID_INCREMENT.fetch_add(1, Ordering::Acquire),
            store: 0,
            parent: None,
            is_root: false,
            extensible: obj.extensible(),
            expanded: true,
            removed: false,
            children: vec![],
            children_id_holder: vec![],
            idx: 0,
            level: 0,
            status: Status::Default,
            cells: obj.cells(),
            node_render: obj.node_render(),
        }
    }

    #[inline]
    pub(crate) fn render_node(
        &self,
        painter: &mut Painter,
        geometry: Rect,
        background: Color,
        ident_length: i32,
    ) {
        let mut geometry: FRect = geometry.into();

        self.node_render
            .render(painter, geometry, background, self.status);

        let tl = geometry.top_left();
        geometry.set_x(tl.x() + (ident_length * self.level) as f32);
        geometry.set_width(geometry.width() - (ident_length * self.level) as f32);

        let gapping = geometry.width() / self.cells.len() as f32;

        for (i, cell) in self.cells.iter().enumerate() {
            let mut cell_rect = geometry.clone();

            cell_rect.set_x(gapping * i as f32);
            cell_rect.set_width(gapping);

            cell.render_cell(painter, geometry);
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
            self.node_shuffle_expand()
        }

        let anchor = if let Some(last) = self.children_id_holder.last() {
            *last
        } else {
            self.id
        };
        if !is_ui_thread() && anchor + 1 != id {
            panic!("Cross thread construction of `TreeView` must strictly follow the order.")
        }
        self.children_id_holder.push(id);
        self.notify_grand_child_add(anchor, &ids);

        if is_ui_thread() {
            let idx = store.node_added(self, id, &ids);

            if idx != usize::MAX {
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
            self.node_shuffle_expand()
        }

        let anchor = if let Some(last) = self.children_id_holder.last() {
            *last
        } else {
            self.id
        };
        if !is_ui_thread() && anchor + 1 != id {
            panic!("Cross thread construction of `TreeView` must strictly follow the order.")
        }
        self.children_id_holder.extend_from_slice(&ids);
        self.notify_grand_child_add(anchor, &ids);

        if is_ui_thread() {
            let idx = store.node_added(self, id, &ids);

            if idx != usize::MAX {
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

    pub(crate) fn notify_grand_child_remove(&mut self, ids: Vec<ObjectId>) {
        if self.parent.is_some() {
            let parent = nonnull_mut!(self.parent);

            if parent.children_id_holder.len() == 0 {
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
                self.notify_grand_child_remove(ids_to_remove);
            } else {
                self.notify_grand_child_remove(vec![self.id()]);
            }

            return Some(hold);
        }
        None
    }

    #[inline]
    pub(crate) fn node_shuffle_expand(&mut self) {
        self.expanded = !self.expanded;

        self.notify_grand_child_expand(
            self.expanded,
            self.id(),
            self.children_id_holder.to_owned(),
        );

        TreeStore::store_mut(self.store)
            .unwrap()
            .node_expanded(self);
    }

    #[inline]
    pub(crate) fn set_status(&mut self, status: Status) {
        self.status = status
    }
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Status {
    #[default]
    Default = 0,
    Selected = 1,
    Hovered = 2,
}
