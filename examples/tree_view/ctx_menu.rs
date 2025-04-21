use tlib::run_after;
use tmui::{
    graphics::box_shadow::{BoxShadow, ShadowSide},
    prelude::*,
    scroll_area::LayoutMode,
    tlib::object::{ObjectImpl, ObjectSubclass},
    views::{
        cell::{cell_render::TextCellRender, Cell},
        node::node_render::NodeRender,
        tree_view::{tree_view_object::TreeViewObject, TreeView},
    },
    widget::WidgetImpl,
};

use crate::tree_view_holder::SimpContent;

#[extends(Popup)]
#[derive(Childable)]
#[run_after]
pub struct CtxMenu {
    #[child]
    selection_list: Tr<TreeView>,
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
        self.set_box_shadow(BoxShadow::new(
            6.,
            Color::BLACK,
            None,
            Some(ShadowSide::new(&[ShadowSide::RIGHT, ShadowSide::BOTTOM])),
            None,
            None,
        ));

        self.selection_list.set_vexpand(true);
        self.selection_list.set_hexpand(true);
        self.selection_list.set_mouse_tracking(true);
        self.selection_list.set_layout_mode(LayoutMode::Overlay);
        self.selection_list
            .get_store_mut()
            .root_mut()
            .add_node(&Selection {
                name: "New seesion",
                value: 1,
            });
        self.selection_list.register_node_pressed(|node, _| {
            println!("Selection pressed.");
            assert_eq!(node.get_value::<i32>(1).unwrap(), 1);
            let view = node.get_view();

            let tree_view_id = view.get_property("view_id").unwrap().get::<ObjectId>();
            let tree_view = ApplicationWindow::window()
                .find_id_mut(tree_view_id)
                .unwrap()
                .downcast_mut::<TreeView>()
                .unwrap();
            tree_view.root_mut().add_node(&SimpContent {
                name: "New Content",
            });

            view.get_parent_mut().unwrap().hide();
        })
    }

    fn on_property_set(&mut self, name: &str, value: &Value) {
        self.parent_on_property_set(name, value);

        if name == "view_id" {
            self.selection_list.set_property("view_id", value.clone());
        }
    }
}

impl WidgetImpl for CtxMenu {
    fn run_after(&mut self) {
        let root = self.selection_list.root_ancestor();
        let root = self.window().find_id(root).unwrap();
        println!(
            "{}'s root ancestor was {}",
            self.selection_list.name(),
            root.name()
        );
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
    pub fn new() -> Tr<Self> {
        Self::new_alloc()
    }
}

struct Selection {
    name: &'static str,
    value: i32,
}
impl TreeViewObject for Selection {
    #[inline]
    fn cells(&self) -> Vec<Cell> {
        vec![
            Cell::string()
                .value(self.name.to_string())
                .cell_render(TextCellRender::builder().color(Color::BLACK).build())
                .build(),
            Cell::value_cell().value(self.value).build(),
        ]
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
