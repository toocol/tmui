use tlib::figure::Color;
use crate::views::{
    cell::{cell_render::TextCellRender, Cell},
    list_view::list_view_object::ListViewObject,
    node::node_render::NodeRender,
};
use super::SelectBounds;

pub struct SelectOption<T: SelectBounds> {
    val: T,
    selected: bool,
    cell_render: Option<Box<TextCellRender>>,
    node_render: Option<NodeRender>,
}

impl<T: SelectBounds> SelectOption<T> {
    #[inline]
    pub fn new(val: T, selected: bool) -> Self {
        Self {
            val,
            selected,
            cell_render: None,
            node_render: None,
        }
    }

    #[inline]
    pub fn new_with_render(
        val: T,
        selected: bool,
        cell_render: Option<Box<TextCellRender>>,
        node_render: Option<NodeRender>,
    ) -> Self {
        Self {
            val,
            selected,
            cell_render,
            node_render,
        }
    }

    #[inline]
    pub fn set_value(&mut self, value: T) {
        self.val = value
    }
    #[inline]
    pub fn value(&self) -> T {
        self.val.clone()
    }

    #[inline]
    pub fn is_selected(&self) -> bool {
        self.selected
    }
    #[inline]
    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected
    }

    #[inline]
    pub fn set_cell_render(&mut self, cell_render: Box<TextCellRender>) {
        self.cell_render = Some(cell_render)
    }

    #[inline]
    pub fn set_node_render(&mut self, node_render: NodeRender) {
        self.node_render = Some(node_render)
    }
}

impl<T: SelectBounds> ListViewObject for SelectOption<T> {
    #[inline]
    fn cells(&self) -> Vec<Cell> {
        let cell_render = if let Some(ref cell_render) = self.cell_render {
            cell_render.clone()
        } else {
            TextCellRender::builder().color(Color::BLACK).build()
        };

        vec![Cell::string()
            .value(self.val.to_string())
            .cell_render(cell_render)
            .build()]
    }

    #[inline]
    fn node_render(&self) -> NodeRender {
        if let Some(ref node_render) = self.node_render {
            *node_render
        } else {
            NodeRender::builder().build()
        }
    }
}
