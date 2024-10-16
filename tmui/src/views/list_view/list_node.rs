use super::{
    list_item::{ItemType, ListItem},
    list_store::ListStore,
    list_view_object::ListViewObject,
    ListView, Painter,
};
use crate::{application::is_ui_thread, views::{
    cell::{cell_index::CellIndex, cell_render::CellRender, Cell},
    node::{node_render::NodeRender, RenderCtx, Status},
}};
use log::warn;
use tlib::{
    global::AsAny,
    object::ObjectId,
    types::StaticType,
    values::{FromValue, ToValue},
};

pub struct ListNode {
    store: ObjectId,
    id: ObjectId,
    status: Status,
    group_managed: bool,

    cells: Vec<Cell>,
    node_render: NodeRender,
}

impl ListNode {
    #[inline]
    pub fn from(obj: &dyn ListViewObject) -> Self {
        Self {
            store: 0,
            id: 0,
            status: Status::empty(),
            group_managed: false,
            cells: obj.cells(),
            node_render: obj.node_render(),
        }
    }

    #[inline]
    pub fn id(&self) -> ObjectId {
        self.id
    }

    #[inline]
    pub fn status(&self) -> Status {
        self.status
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
    pub fn is_group_managed(&self) -> bool {
        self.group_managed
    }

    #[inline]
    pub fn store_ref(&self) -> &ListStore {
        ListStore::store_ref(self.store)
            .expect("Call `store_ref()` after adding this node to `ListStore`.")
    }

    #[inline]
    pub fn store_mut(&mut self) -> &mut ListStore {
        ListStore::store_mut(self.store)
            .expect("Call `store_mut()` after adding this node to `ListStore`.")
    }

    #[inline]
    pub fn get_view(&mut self) -> &mut ListView {
        self.store_mut()
            .get_view()
            .downcast_mut::<ListView>()
            .unwrap()
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
    pub fn notify_update(&mut self) {
        self.store_mut().notify_update()
    }
}

impl ListNode {
    #[inline]
    pub(crate) fn set_store_id(&mut self, id: ObjectId) {
        self.store = id;
    }

    #[inline]
    pub(crate) fn set_id(&mut self, id: ObjectId) {
        self.id = id;
    }

    #[inline]
    pub(crate) fn set_group_managed(&mut self, is: bool) {
        self.group_managed = is;
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

    #[inline]
    pub(crate) fn add_status(&mut self, status: Status) {
        self.status.insert(status)
    }

    #[inline]
    pub(crate) fn remove_status(&mut self, status: Status) {
        self.status.remove(status)
    }
}

impl ListItem for ListNode {
    #[inline]
    fn item_type(&self) -> ItemType {
        ItemType::Node
    }

    fn render(&self, painter: &mut Painter, render_ctx: RenderCtx) {
        let geometry = render_ctx.geometry;

        self.node_render
            .render(painter, render_ctx, self.status);

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
}

impl AsAny for ListNode {
    #[inline]
    fn as_any(&self) -> &dyn tlib::prelude::Any {
        self
    }

    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn tlib::prelude::Any {
        self
    }

    #[inline]
    fn as_any_boxed(self: Box<Self>) -> Box<dyn tlib::prelude::Any> {
        self
    }
}
