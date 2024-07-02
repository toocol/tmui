pub mod list_view_holder;

use list_view_holder::ListViewHolder;
use tmui::{
    application::Application,
    application_window::ApplicationWindow,
    prelude::*,
    views::{
        cell::{
            cell_render::TextCellRender,
            Cell,
        },
        list_view::list_view_object::ListViewObject,
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
    window.child(ListViewHolder::new())
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