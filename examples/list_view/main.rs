pub mod list_view_holder;

use list_view_holder::ListViewHolder;
use tmui::{
    application::Application,
    application_window::ApplicationWindow,
    icons::svg_dom::SvgDom,
    prelude::*,
    views::{
        cell::{
            cell_render::{SvgCellRender, TextCellRender},
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
        let dom = SvgDom::from_file("examples/resources/sword_rose.svg");
        vec![
            Cell::string()
                .value(self.name.clone())
                .cell_render(
                    TextCellRender::builder()
                        .color(Color::BLACK)
                        .selection_color(Some(Color::WHITE))
                        .halign(Align::Center)
                        .valign(Align::Center)
                        .build(),
                )
                .build(),
            Cell::string()
                .value(self.name.clone())
                .cell_render(
                    TextCellRender::builder()
                        .color(Color::BLACK)
                        .valign(Align::Center)
                        .build(),
                )
                .build(),
            Cell::svg()
                .cell_render(SvgCellRender::builder().dom(Some(dom)).build())
                .build(),
        ]
    }

    #[inline]
    fn node_render(&self) -> NodeRender {
        NodeRender::builder().build()
    }
}
