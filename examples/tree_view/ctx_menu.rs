use tmui::{
    prelude::*,
    tlib::{
        global_watch,
        object::{ObjectImpl, ObjectSubclass},
    },
    tree_view::{
        cell::{cell_render::TextCellRender, Cell},
        node_render::NodeRender,
        tree_view_object::TreeViewObject,
        TreeView,
    },
    widget::WidgetImpl,
};

#[extends(Popup)]
#[derive(Childable)]
#[global_watch(MouseReleased)]
pub struct CtxMenu {
    #[child]
    selection_list: Box<TreeView>,
}

impl ObjectSubclass for CtxMenu {
    const NAME: &'static str = "CtxMenu";
}

impl ObjectImpl for CtxMenu {
    fn initialize(&mut self) {
        self.width_request(150);
        self.height_request(60);

        self.set_borders(1., 1., 1., 1.);
        self.set_border_color(Color::GREY_LIGHT);

        self.selection_list.set_vexpand(true);
        self.selection_list.set_hexpand(true);
        self.selection_list.set_mouse_tracking(true);
        self.selection_list
            .get_store_mut()
            .root_mut()
            .add_node(&Selection {
                name: "New seesion",
            });
    }
}

impl WidgetImpl for CtxMenu {}

impl GlobalWatchImpl for CtxMenu {
    fn on_global_mouse_released(&mut self, evt: &tlib::events::MouseEvent) -> bool {
        if !self.visible() {
            return false;
        }
        let pos: Point = evt.position().into();
        if !self.rect().contains(&pos) {
            self.hide();
        }

        true
    }
}

impl PopupImpl for CtxMenu {
    fn calculate_position(&self, _: Rect, point: Point) -> Point {
        point
    }

    fn is_modal(&self) -> bool {
        true
    }
}

impl CtxMenu {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}

struct Selection {
    name: &'static str,
}
impl TreeViewObject for Selection {
    #[inline]
    fn cells(&self) -> Vec<Cell> {
        vec![Cell::string()
            .value(self.name.to_string())
            .cell_render(TextCellRender::builder().color(Color::BLACK).build())
            .build()]
    }

    #[inline]
    fn extensible(&self) -> bool {
        false
    }

    #[inline]
    fn node_render(&self) -> NodeRender {
        NodeRender::builder().build()
    }
}
