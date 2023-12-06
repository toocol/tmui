use tmui::{
    application::Application,
    application_window::ApplicationWindow,
    prelude::*,
    tree_view::{
        cell::{cell_render::TextCellRender, Cell},
        node_render::NodeRender,
        tree_view_object::TreeViewObject,
        TreeView,
    },
};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("Tree View")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let mut tree_view = TreeView::new();
    tree_view.set_hexpand(true);
    tree_view.set_vexpand(true);
    tree_view.set_hscale(0.3);
    tree_view.set_halign(Align::Center);
    tree_view.set_line_spacing(10);

    let group1 = tree_view
        .get_store_mut()
        .root_mut()
        .add_node(&Group { name: "group-1" });

    if let Some(group1) = group1 {
        for i in 0..10 {
            let content = format!("content_{}", i);
            group1.add_node(&Content { val: content });
        }

        if let Some(group2) = group1.add_node(&Group { name: "group-2" }) {
            for i in 0..30000 {
                let content = format!("sub_content_{}", i);
                group2.add_node(&Content { val: content });
            }
        }

        for i in 10..15 {
            let content = format!("content_{}", i);
            group1.add_node(&Content { val: content });
        }

        if let Some(group3) = group1.add_node(&Group { name: "group-3" }) {
            for i in 0..30000 {
                let content = format!("sub_content_{}", i);
                group3.add_node(&Content { val: content });
            }
        }
    }

    window.child(tree_view)
}

pub struct Group {
    name: &'static str,
}
impl TreeViewObject for Group {
    #[inline]
    fn cells(&self) -> Vec<Cell> {
        vec![Cell::string()
            .value(self.name.to_string())
            .cell_render(TextCellRender::builder().color(Color::BLACK).build())
            .build()]
    }

    #[inline]
    fn extensible(&self) -> bool {
        true
    }

    #[inline]
    fn node_render(&self) -> NodeRender {
        NodeRender::default()
    }
}

pub struct Content {
    val: String,
}
impl TreeViewObject for Content {
    #[inline]
    fn cells(&self) -> Vec<Cell> {
        vec![Cell::string()
            .value(self.val.clone())
            .cell_render(TextCellRender::builder().color(Color::BLACK).build())
            .build()]
    }

    #[inline]
    fn extensible(&self) -> bool {
        false
    }

    #[inline]
    fn node_render(&self) -> NodeRender {
        NodeRender::default()
    }
}
