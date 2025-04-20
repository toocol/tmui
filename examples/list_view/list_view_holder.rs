use std::thread;

use tlib::namespace::MouseButton;
use tmui::{
    container::ScaleStrat,
    prelude::*,
    scroll_area::LayoutMode,
    tlib::object::{ObjectImpl, ObjectSubclass},
    views::list_view::{list_group::ListGroup, ListView},
    widget::WidgetImpl,
};

use crate::Node;

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
#[async_task(name = "BuildListTaskI", value = "()")]
#[async_task(name = "BuildListTaskII", value = "()")]
#[async_task(name = "BuildListTaskIII", value = "()")]
pub struct ListViewHolder {
    #[children]
    list_view_1: Tr<ListView>,

    #[children]
    list_view_2: Tr<ListView>,

    #[children]
    list_view_3: Tr<ListView>,
}

impl ObjectSubclass for ListViewHolder {
    const NAME: &'static str = "ListViewHolder";
}

impl ObjectImpl for ListViewHolder {
    fn initialize(&mut self) {
        self.set_hexpand(true);
        self.set_vexpand(true);
        self.set_spacing(20);
        self.set_scale_strat(ScaleStrat::Direct);

        self.list_view_1.set_hexpand(true);
        self.list_view_1.set_vexpand(true);
        self.list_view_1.set_hscale(0.3);

        self.list_view_2.set_hexpand(true);
        self.list_view_2.set_vexpand(true);
        self.list_view_2.set_hscale(0.3);
        self.list_view_2.set_layout_mode(LayoutMode::Overlay);
        self.list_view_2
            .scroll_bar_mut()
            .set_background(Color::TRANSPARENT);
        self.list_view_2
            .scroll_bar_mut()
            .set_color(Color::GREY_LIGHT.with_a(155));
        self.list_view_2
            .scroll_bar_mut()
            .set_active_color(Some(Color::GREY_MEDIUM.with_a(155)));
        self.list_view_2.scroll_bar_mut().set_slider_radius(5.);
        self.list_view_2.scroll_bar_mut().set_auto_hide(true);
        self.list_view_2.scroll_bar_mut().set_visible_in_valid(true);

        self.list_view_3.set_hexpand(true);
        self.list_view_3.set_vexpand(true);
        self.list_view_3.set_hscale(0.3);
        self.list_view_3.scroll_bar_mut().set_auto_hide(true);

        self.list_view_1.register_node_enter(|node, _, _| {
            println!("Node enter, {}", node.id());
        });
        self.list_view_1.register_node_leave(|node, _, _| {
            println!("Node leave, {}", node.id());
        });
        self.list_view_1.register_node_pressed(|node, mutex, evt| {
            println!("Node pressed, {}", node.id());

            if evt.mouse_button() == MouseButton::RightButton {
                let idx = mutex.add_node(&Node {
                    name: "New Node".to_string(),
                });
                node.get_view().scroll_to(idx);
            }
        });
        self.list_view_1.register_node_released(|node, _, _| {
            println!("Node released, {}", node.id());
        });

        self.list_view_2.register_free_area_pressed(|w, _| {
            println!("Free area pressed, {}", w.name());
            let view = w.downcast_mut::<ListView>().unwrap();
            let arc = view.concurrent_store();
            println!("List view prepare async add node.");
            async_do!(move {
                let mut list = arc.lock();
                for i in 10..1000000 {
                    list.add_node(&Node {
                        name: format!("Node_{}", i),
                    });
                }
                println!("List view async add node complete.");
                ()
            });
        });
        self.list_view_2.register_free_area_released(|w, _| {
            println!("Free area released, {}", w.name());
        });

        self.list_view_1.start_loading();
        let arc = self.list_view_1.concurrent_store();
        self.build_list_task_i(
            async move {
                println!("Build list 1 in thread {:?}", thread::current().id());
                let mut list = arc.lock();
                for i in 0..5 {
                    list.add_node(&Node {
                        name: format!("Node_{}", i),
                    });
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
                    });
                }
            },
            |w: &mut ListViewHolder, _| {
                w.list_view_1.stop_loading();
                w.list_view_1.scroll_to(9);
            },
        );

        self.list_view_2.start_loading();
        let arc = self.list_view_2.concurrent_store();
        self.build_list_task_i_i(
            async move {
                println!("Build list 2 in thread {:?}", thread::current().id());
                let mut list = arc.lock();

                for i in 0..10 {
                    list.add_node(&Node {
                        name: format!("Node_{}", i),
                    });
                }

                drop(list);
            },
            |w: &mut ListViewHolder, _| {
                println!("list view 2 stop loading");
                w.list_view_2.stop_loading();
            },
        );

        self.list_view_3.start_loading();
        let arc = self.list_view_3.concurrent_store();
        self.build_list_task_i_i_i(
            async move {
                println!("Build list 3 in thread {:?}", thread::current().id());
                let mut list = arc.lock();

                for i in 0..1000 {
                    list.add_node(&Node {
                        name: format!("Node_{}", i),
                    });
                }

                drop(list);
            },
            |w: &mut ListViewHolder, _| {
                w.list_view_3.stop_loading();
            },
        )
    }
}

impl WidgetImpl for ListViewHolder {}

impl ListViewHolder {
    #[inline]
    pub fn new() -> Tr<Self> {
        Self::new_alloc()
    }
}
