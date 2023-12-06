use super::{
    cell::{cell_render::CellRender, Cell},
    node_render::NodeRender,
    tree_store::TreeStore,
    tree_view_object::TreeViewObject,
};
use crate::graphics::painter::Painter;
use crate::prelude::*;
use log::warn;
use std::ptr::NonNull;
use tlib::{
    figure::Rect,
    global::SemanticExt,
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
    children: Vec<Box<TreeNode>>,
    children_id_holder: Vec<ObjectId>,
    idx: usize,
    level: i32,
    status: Status,

    cells: Vec<Cell>,
    node_render: NodeRender,
}

pub trait TreeNodeSignals: ActionExt {
    signals!(
        TreeNode:

        hover_changed();

        selection_changed();
    );
}
impl TreeNodeSignals for TreeNode {}

impl TreeNode {
    pub fn add_node(&mut self, obj: &dyn TreeViewObject) -> Option<&mut TreeNode> {
        let mut node = Self::create_from_obj(obj);

        node.store = self.store.clone();

        self.add_node_inner(node.boxed())
    }

    pub fn remove(&mut self) {
        self.remove_node_inner()
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
    pub fn is_extensible(&self) -> bool {
        self.extensible
    }

    #[inline]
    pub fn node_clicked(&mut self) {
        self.expanded = !self.expanded;

        self.notify_grand_child_expand(self.expanded, self.id(), self.children_id_holder.to_owned())
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
            object: Object::default(),
            store: None,
            parent: None,
            is_root: false,
            extensible: obj.extensible(),
            expanded: true,
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
        mut geometry: Rect,
        background: Color,
        ident_length: i32,
    ) {
        self.node_render
            .render(painter, geometry, background, self.status);

        let tl = geometry.top_left();
        geometry.set_x(tl.x() + ident_length * self.level);
        geometry.set_width(geometry.width() - ident_length * self.level);
        for cell in self.cells.iter() {
            cell.render_cell(painter, geometry);
        }
    }

    #[inline]
    pub(crate) fn get_children_ids(&self) -> &Vec<ObjectId> {
        &self.children_id_holder
    }

    pub(crate) fn add_node_inner(&mut self, mut node: Box<TreeNode>) -> Option<&mut TreeNode> {
        if !self.extensible {
            return None;
        }

        debug_assert!(node.store.is_some());

        node.parent = NonNull::new(self);
        node.idx = self.children.len();
        node.level = self.level + 1;

        let store = nonnull_mut!(self.store);
        store.add_node_cache(&mut node);

        let id = node.id();
        self.children_id_holder.push(id);
        self.notify_grand_child_add(id);

        self.children.push(node);

        store.get_node_mut(id)
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

    pub(crate) fn notify_grand_child_remove(&mut self, ids: Vec<ObjectId>) {
        if self.parent.is_some() {
            let parent = nonnull_mut!(self.parent);
            if parent.is_root {
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
        if parent.is_root {
            return;
        }

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

    pub(crate) fn remove_node_inner(&mut self) {
        if self.parent.is_some() {
            let parent = nonnull_mut!(self.parent);

            debug_assert!(parent.children.len() > self.idx);
            parent.children.remove(self.idx);

            nonnull_mut!(self.store).remove_node_cache(self.id());

            for (i, c) in parent.children.iter_mut().enumerate() {
                c.idx = i;
            }

            let mut ids_to_remove = self.get_children_ids().to_owned();
            ids_to_remove.push(self.id());
            self.notify_grand_child_remove(ids_to_remove);
        }
    }

    pub(crate) fn initialize_buffer(&mut self, buffer: &mut Vec<Option<NonNull<TreeNode>>>) {
        if !self.is_root {
            buffer.push(NonNull::new(self));
        }
        if self.expanded {
            for c in self.children.iter_mut() {
                c.initialize_buffer(buffer)
            }
        }
    }

    pub(crate) fn set_status(&mut self, status: Status) {
        self.status = status
    }
}

impl ObjectSubclass for TreeNode {
    const NAME: &'static str = "TreeViewNode";
}
impl ObjectImpl for TreeNode {}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Status {
    #[default]
    Default = 0,
    Selected = 1,
    Hovered = 2,
}
