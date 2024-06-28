use tmui::{
    application::Application,
    application_window::ApplicationWindow,
    prelude::*,
    views::{
        cell::{
            cell_render::TextCellRender,
            Cell,
        },
        list_view::{list_view_object::ListViewObject, ListView},
        node::node_render::NodeRender,
    },
};

fn main() {
    log4rs::init_file("examples/log4rs.yaml", Default::default()).unwrap();

    let app = Application::builder()
        .width(1280)
        .height(800)
        .title("List View")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(window: &mut ApplicationWindow) {
    let mut list_view = ListView::new();

    list_view.set_hexpand(true);
    list_view.set_vexpand(true);
    list_view.set_hscale(0.3);

    for i in 0..1000000 {
        list_view.add_node(&Node {
            name: format!("Node_{}", i),
        })
    }

    window.child(list_view)
}

struct Node {
    name: String,
}
impl ListViewObject for Node {
    #[inline]
    fn cells(&self) -> Vec<Cell> {
        vec![
            Cell::string()
                .value(self.name.clone())
                .cell_render(TextCellRender::builder().color(Color::BLACK).build())
                .build(),
            Cell::string()
                .value(self.name.clone())
                .cell_render(TextCellRender::builder().color(Color::BLACK).build())
                .build(),
        ]
    }

    #[inline]
    fn node_render(&self) -> NodeRender {
        NodeRender::builder().build()
    }
}
