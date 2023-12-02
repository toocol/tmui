use super::{
    cell::{cell_render::CellRender, Cell},
    tree_store::TreeStore,
    tree_view_object::TreeViewObject,
};
use crate::graphics::painter::Painter;
use crate::prelude::*;
use log::warn;
use std::ptr::NonNull;
use tlib::{
    figure::Rect,
    nonnull_mut,
    object::ObjectSubclass,
    signals,
    types::StaticType,
    values::{FromValue, ToValue},
};

#[extends(Object)]
pub struct TreeNode {
    pub(crate) store: Option<NonNull<TreeStore>>,

    parent: Option<NonNull<TreeNode>>,
    is_root: bool,
    extensible: bool,
    expanded: bool,
    selected: bool,
    children: Vec<TreeNode>,
    children_id_holder: Vec<ObjectId>,
    idx: usize,
    level: usize,

    cells: Vec<Cell>,
}

pub trait TreeNodeSignals: ActionExt {
    signals!(
        TreeNodeSignals:

        /// @param: id of TreeNode [`u32`]
        /// @param: whther is selected [`bool`]
        selected();

        /// @param: id of TreeNode [`u32`]
        /// param: whther is expanded [`bool`]
        expanded();
    );
}
impl TreeNodeSignals for TreeNode {}

impl TreeNode {
    pub fn add_node(&mut self, obj: &dyn TreeViewObject) {
        let mut node = Self::create_from_obj(obj);

        node.store = self.store.clone();

        self.add_node_inner(node);
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

    pub fn get_render(&self, cell_idx: usize) -> Option<&dyn CellRender> {
        self.cells
            .get(cell_idx)
            .or_else(|| {
                warn!("Undefined cell of tree view node, cell index: {}", cell_idx);
                None
            })
            .and_then(|cell| Some(cell.get_render()))
    }

    pub fn get_render_mut(&mut self, cell_idx: usize) -> Option<&mut dyn CellRender> {
        self.cells
            .get_mut(cell_idx)
            .or_else(|| {
                warn!("Undefined cell of tree view node, cell index: {}", cell_idx);
                None
            })
            .and_then(|cell| Some(cell.get_render_mut()))
    }

    pub fn is_expanded(&self) -> bool {
        self.expanded
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }
}

impl TreeNode {
    #[inline]
    pub(crate) fn empty() -> Self {
        Self {
            object: Object::default(),
            store: None,
            parent: None,
            is_root: true,
            extensible: true,
            expanded: true,
            selected: false,
            children: vec![],
            children_id_holder: vec![],
            idx: 0,
            level: 0,
            cells: vec![],
        }
    }

    #[inline]
    pub(crate) fn create_from_obj(obj: &dyn TreeViewObject) -> Self {
        Self {
            object: Object::default(),
            store: None,
            parent: None,
            is_root: false,
            extensible: obj.extensible(),
            expanded: true,
            selected: false,
            children: vec![],
            children_id_holder: vec![],
            idx: 0,
            level: 0,
            cells: obj.cells(),
        }
    }

    #[inline]
    pub(crate) fn render_node(&self, painter: &mut Painter, geometry: Rect) {
        for cell in self.cells.iter() {
            // TODO: calculate the cell's geometry;
            cell.render_cell(painter, geometry);
        }
    }

    #[inline]
    pub(crate) fn get_children_ids(&self) -> &Vec<ObjectId> {
        &self.children_id_holder
    }

    pub(crate) fn add_node_inner(&mut self, mut node: TreeNode) {
        if !self.extensible {
            return;
        }

        debug_assert!(node.store.is_some());

        node.parent = NonNull::new(self);
        node.idx = self.children.len();
        node.level = self.level + 1;

        nonnull_mut!(self.store).add_node_cache(&mut node);

        self.children_id_holder.push(node.id());
        self.notify_grand_child_add(node.id());

        self.children.push(node);
    }

    pub(crate) fn notify_grand_child_add(&mut self, id: ObjectId) {
        if self.parent.is_some() {
            let parent = nonnull_mut!(self.parent);
            if parent.is_root {
                return;
            }

            parent.children_id_holder.push(id);

            parent.notify_grand_child_add(id);
        }
    }

    pub(crate) fn initialize_buffer(
        &mut self,
        buffer: &mut Vec<Option<NonNull<TreeNode>>>,
    ) {
        if !self.is_root {
            buffer.push(NonNull::new(self));
        }
        if self.expanded {
            for c in self.children.iter_mut() {
                c.initialize_buffer(buffer)
            }
        }
    }
}

impl Drop for TreeNode {
    fn drop(&mut self) {
        if self.parent.is_some() {
            let parent = nonnull_mut!(self.parent);

            parent.children.remove(self.idx);

            nonnull_mut!(self.store).remove_node_cache(self.id());

            for (i, c) in parent.children.iter_mut().enumerate() {
                c.idx = i;
            }
        }
    }
}

impl ObjectSubclass for TreeNode {
    const NAME: &'static str = "TreeViewNode";
}
impl ObjectImpl for TreeNode {}
