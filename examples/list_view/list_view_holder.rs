use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    views::list_view::{list_group::ListGroup, ListView},
    widget::WidgetImpl,
};

use crate::Node;

#[extends(Widget)]
#[derive(Childable)]
#[async_task(name = "BuildListTask", value = "()")]
pub struct ListViewHolder {
    #[child]
    list_view: Box<ListView>,
}

impl ObjectSubclass for ListViewHolder {
    const NAME: &'static str = "ListViewHolder";
}

impl ObjectImpl for ListViewHolder {
    fn initialize(&mut self) {
        self.set_hexpand(true);
        self.set_vexpand(true);

        self.list_view.set_hexpand(true);
        self.list_view.set_vexpand(true);
        self.list_view.set_hscale(0.3);

        self.list_view.register_node_enter(|node, _| {
            println!("Node enter, {}", node.id());
        });
        self.list_view.register_node_leave(|node, _| {
            println!("Node leave, {}", node.id());
        });
        self.list_view.register_node_pressed(|node, _| {
            println!("Node pressed, {}", node.id());
        });
        self.list_view.register_node_released(|node, _| {
            println!("Node released, {}", node.id());
        });
        self.list_view.register_free_area_pressed(|w, _| {
            println!("Free area pressed, {}", w.name());
        });
        self.list_view.register_free_area_released(|w, _| {
            println!("Free area released, {}", w.name());
        });

        self.list_view.start_loading();
        let arc = self.list_view.concurrent_store();
        self.build_list_task(
            async move {
                let mut list = arc.lock();
                for i in 0..5 {
                    list.add_node(&Node {
                        name: format!("Node_{}", i),
                    })
                }

                let mut group = ListGroup::new();
                for i in 5..10 {
                    group.add_node(&Node {
                        name: format!("Node_{}", i),
                    })
                }
                list.add_group(group);

                for i in 10..1000000 {
                    list.add_node(&Node {
                        name: format!("Node_{}", i),
                    })
                }
            },
            |w: &mut ListViewHolder, _| {
                w.list_view.stop_loading();
            },
        );
    }
}

impl WidgetImpl for ListViewHolder {}

impl ListViewHolder {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
