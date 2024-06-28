use super::{
    list_item::{ItemType, ListItem, RenderCtx},
    list_view_object::ListViewObject, Painter,
};
use crate::views::{cell::Cell, node::{node_render::NodeRender, Status}};
use tlib::{global::AsAny, object::ObjectId};

pub struct ListNode {
    id: ObjectId,
    status: Status,

    cells: Vec<Cell>,
    node_render: NodeRender,
}

impl ListNode {
    #[inline]
    pub fn id(&self) -> ObjectId {
        self.id
    }
}

impl ListNode {
    #[inline]
    pub(crate) fn create_from_obj(obj: &dyn ListViewObject) -> Self {
        Self {
            id: 0,
            status: Status::Default,
            cells: obj.cells(),
            node_render: obj.node_render(),
        }
    }

    #[inline]
    pub(crate) fn set_id(&mut self, id: ObjectId) {
        self.id = id;
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
}

impl ListItem for ListNode {
    #[inline]
    fn item_type(&self) -> ItemType {
        ItemType::Node
    }

    fn render(&self, painter: &mut Painter, render_ctx: RenderCtx) {
        let geometry = render_ctx.geometry;
        let background = render_ctx.background;

        self.node_render.render(painter, geometry, background, self.status);

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
