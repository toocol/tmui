#![allow(dead_code)]
use std::{cmp::Ordering, time::Duration};
use tlib::{
    actions::ActionExt, compare::Compare, connect, global::SemanticExt, namespace::MouseButton,
    timer::Timer, tokio::task::JoinHandle,
};
use tmui::{
    container::ScaleStrat,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    views::{
        cell::{cell_render::TextCellRender, Cell},
        node::node_render::NodeRender,
        tree_view::{tree_node::TreeNode, tree_view_object::TreeViewObject, TreeView},
    },
    widget::{ChildOp, WidgetImpl},
};

use crate::ctx_menu::CtxMenu;

const DATA_SIZE: u32 = 300000;

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
#[async_task(name = "BuildTreeTask", value = "Box<TreeNode>")]
#[async_task(name = "BuildSortTreeTask", value = "Box<TreeNode>")]
pub struct TreeViewHolder {
    #[children]
    tree_view: Box<TreeView>,

    #[children]
    tree_view_2: Box<TreeView>,

    #[children]
    tree_view_3: Box<TreeView>,
}

impl ObjectSubclass for TreeViewHolder {
    const NAME: &'static str = "TreeViewHolder";
}

impl ObjectImpl for TreeViewHolder {
    fn construct(&mut self) {
        self.parent_construct();

        self.tree_view
            .set_layout_mode(tmui::scroll_area::LayoutMode::Overlay);
        self.tree_view.add_popup(CtxMenu::new());
        self.tree_view.start_loading();
        self.tree_view.set_hexpand(true);
        self.tree_view.set_vexpand(true);
        self.tree_view.set_mouse_tracking(true);
        self.tree_view.set_line_spacing(10);
        self.tree_view.set_hscale(0.3);

        self.tree_view
            .scroll_bar_mut()
            .set_background(Color::TRANSPARENT);
        self.tree_view
            .scroll_bar_mut()
            .set_color(Color::GREY_LIGHT.with_a(155));
        self.tree_view
            .scroll_bar_mut()
            .set_active_color(Some(Color::GREY_MEDIUM.with_a(155)));
        self.tree_view.scroll_bar_mut().set_slider_radius(5.);
        self.tree_view.scroll_bar_mut().set_auto_hide(true);

        self.tree_view.register_node_pressed(|node, evt| {
            println!(
                "Node has pressed, id = {}, mouse position = {:?}, value = {:?}",
                node.id(),
                evt.position(),
                node.get_value::<String>(0)
            );

            if evt.mouse_button() == MouseButton::RightButton {
                if node.is_extensible() {
                    node.add_node(&Content {
                        val: "new_content".to_string(),
                        idx: 1,
                    });
                } else {
                    node.remove();
                }
            }
        });
        self.tree_view.register_node_released(|node, evt| {
            println!(
                "Node released, id = {}, position = {:?}",
                node.id(),
                evt.position()
            );
        });
        self.tree_view.register_node_enter(|node, _| {
            println!("Node enter, id = {}", node.id());
        });
        self.tree_view.register_node_leave(|node, _| {
            println!("Node leave, id = {}", node.id());
        });
        self.tree_view.register_free_area_released(|node, evt| {
            if evt.mouse_button() != MouseButton::RightButton {
                return;
            }
            let view = node.get_view();
            view.show_popup(view.map_to_global(&evt.position().into()));
        });

        self.set_hexpand(true);
        self.set_vexpand(true);
        self.set_halign(Align::Center);
        self.set_scale_strat(ScaleStrat::Direct);

        let store = self.tree_view.get_store_mut();

        let id = store.id();
        let level = store.root_mut().level();

        self.build_tree_task(
            async move {
                let mut group1 = TreeNode::create(id, level + 1, &Group { name: "group-1" });

                for i in 0..10 {
                    let content = format!("content_{}", i);
                    group1.add_node(&Content {
                        val: content,
                        idx: i,
                    });
                }

                if let Some(group2) = group1.add_node(&Group { name: "group-2" }) {
                    for i in 0..DATA_SIZE {
                        let content = format!("sub_content_{}", i);
                        group2.add_node(&Content {
                            val: content,
                            idx: i,
                        });
                    }
                }

                for i in 10..15 {
                    let content = format!("content_{}", i);
                    group1.add_node(&Content {
                        val: content,
                        idx: i,
                    });
                }

                if let Some(group3) = group1.add_node(&Group { name: "group-3" }) {
                    for i in 0..DATA_SIZE {
                        let content = format!("sub_content_{}", i);
                        group3.add_node(&Content {
                            val: content,
                            idx: i,
                        });
                    }
                }

                group1
            },
            |w: &mut TreeViewHolder, node| {
                let root_mut = w.tree_view.get_store_mut().root_mut();

                root_mut.add_node_directly(node);

                w.tree_view.stop_loading();
            },
        );

        // let mut root_mut = NonNull::new(store.root_mut());
        // let join = tokio::spawn(async move {
        //     let mut group1 = TreeNode::create(id, level + 1, &Group { name: "group-1" });

        //     for i in 0..10 {
        //         let content = format!("content_{}", i);
        //         group1.add_node(&Content { val: content });
        //     }

        //     if let Some(group2) = group1.add_node(&Group { name: "group-2" }) {
        //         for i in 0..DATA_SIZE {
        //             let content = format!("sub_content_{}", i);
        //             group2.add_node(&Content { val: content });
        //         }
        //     }

        //     for i in 10..15 {
        //         let content = format!("content_{}", i);
        //         group1.add_node(&Content { val: content });
        //     }

        //     if let Some(group3) = group1.add_node(&Group { name: "group-3" }) {
        //         for i in 0..DATA_SIZE {
        //             let content = format!("sub_content_{}", i);
        //             group3.add_node(&Content { val: content });
        //         }
        //     }

        //     group1
        // });
        // self.task = Some(AsyncTask::new(join).then(move |node| {
        //     let root_mut = nonnull_mut!(root_mut);

        //     root_mut.add_node_directly(node);
        //     root_mut.notify_update();

        //     // let group1 = root_mut.add_node(&Group { name: "group-1" }).unwrap();

        //     // for i in 0..10 {
        //     //     let content = format!("content_{}", i);
        //     //     group1.add_node(&Content { val: content });
        //     // }

        //     // if let Some(group2) = group1.add_node(&Group { name: "group-2" }) {
        //     //     for i in 0..300 {
        //     //         let content = format!("sub_content_{}", i);
        //     //         group2.add_node(&Content { val: content });
        //     //     }
        //     // }

        //     // for i in 10..15 {
        //     //     let content = format!("content_{}", i);
        //     //     group1.add_node(&Content { val: content });
        //     // }

        //     // if let Some(group3) = group1.add_node(&Group { name: "group-3" }) {
        //     //     for i in 0..300 {
        //     //         let content = format!("sub_content_{}", i);
        //     //         group3.add_node(&Content { val: content });
        //     //     }
        //     // }
        //     // root_mut.notify_update();
        // }));

        self.tree_view_2.set_hexpand(true);
        self.tree_view_2.set_vexpand(true);
        self.tree_view_2.set_hscale(0.3);

        self.tree_view_2
            .get_store_mut()
            .root_mut()
            .add_node(&Group { name: "roots" });
        self.tree_view_2.register_node_released(|node, evt| {
            if !node.is_extensible() {
                return;
            }
            if evt.mouse_button() == MouseButton::RightButton {
                node.add_node(&Group { name: "group" });
            } else if evt.mouse_button() == MouseButton::MiddleButton {
                node.add_node(&SimpContent { name: "content" });
            }
        });
        self.tree_view_2
            .set_sort_proxy(Compare::<TreeNode>::new(|a, b| {
                if a.is_extensible() && !b.is_extensible() {
                    Ordering::Less
                } else if !a.is_extensible() && b.is_extensible() {
                    Ordering::Greater
                } else {
                    a.id().cmp(&b.id())
                }
            }));

        self.tree_view_3.set_hexpand(true);
        self.tree_view_3.set_vexpand(true);
        self.tree_view_3.set_hscale(0.3);
        self.tree_view_3
            .set_sort_proxy(Compare::<TreeNode>::new(|a, b| {
                if a.is_extensible() && !b.is_extensible() {
                    Ordering::Less
                } else if !a.is_extensible() && b.is_extensible() {
                    Ordering::Greater
                } else {
                    a.id().cmp(&b.id())
                }
            }));

        let store = self.tree_view_3.get_store_mut();
        let store_id = store.id();
        let level = store.root_mut().level() + 1;
        self.build_sort_tree_task(
            async move {
                let mut root = TreeNode::create(store_id, level, &Group { name: "root" });

                root.add_node(&SimpContent { name: "cont-1" });
                root.add_node(&SimpContent { name: "cont-2" });
                root.add_node(&SimpContent { name: "cont-3" });

                if let Some(group1) = root.add_node(&Group { name: "group1" }) {
                    group1.add_node(&SimpContent { name: "cont-gp1-1" });
                    group1.add_node(&SimpContent { name: "cont-gp1-2" });
                    if let Some(group11) = group1.add_node(&Group { name: "group1-1" }) {
                        group11.add_node(&SimpContent {
                            name: "cont-gp1-1-1",
                        });
                        group11.add_node(&SimpContent {
                            name: "cont-gp1-1-2",
                        });
                        group11.shuffle_expand();
                    }
                    group1.shuffle_expand();
                }

                root.add_node(&SimpContent { name: "cont-4" });
                root.add_node(&SimpContent { name: "cont-5" });

                if let Some(group2) = root.add_node(&Group { name: "group2" }) {
                    group2.add_node(&SimpContent { name: "cont-gp2-1" });
                    group2.add_node(&SimpContent { name: "cont-gp2-2" });
                    group2.add_node(&Group { name: "group2-1" });
                    group2.add_node(&Group { name: "group2-2" });
                    group2.add_node(&SimpContent { name: "cont-gp2-3" });
                    group2.add_node(&SimpContent { name: "cont-gp2-4" });
                    group2.add_node(&Group { name: "group2-3" });
                    group2.add_node(&SimpContent { name: "cont-gp2-5" });
                }

                root.sort(true);
                root
            },
            |w: &mut TreeViewHolder, node| {
                w.tree_view_3.root_mut().add_node_directly(node);
            },
        )
    }
}

impl WidgetImpl for TreeViewHolder {}

impl TreeViewHolder {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}

#[extends(Object)]
pub struct AsyncTask {
    join_handler: Option<JoinHandle<Box<TreeNode>>>,
    timer: Box<Timer>,
    then: Option<Box<dyn FnOnce(Box<TreeNode>)>>,
}
impl AsyncTask {
    #[inline]
    pub fn new(join: JoinHandle<Box<TreeNode>>) -> Box<Self> {
        let mut task = Self {
            object: Default::default(),
            join_handler: Some(join),
            timer: Timer::new(),
            then: None,
        }
        .boxed();

        connect!(task.timer, timeout(), task, check());
        task.timer.start(Duration::from_millis(1));

        task
    }

    #[inline]
    pub fn then<F: FnOnce(Box<TreeNode>) + 'static>(mut self: Box<Self>, then: F) -> Box<Self> {
        self.then = Some(Box::new(then));
        self
    }

    fn check(&mut self) {
        let join_handler = self.join_handler.as_mut().unwrap();
        if join_handler.is_finished() {
            self.timer.disconnect_all();
            self.timer.stop();

            let result = tokio_runtime().block_on(join_handler).unwrap();
            if let Some(then) = self.then.take() {
                then(result);
            }
        }
    }
}
impl ObjectSubclass for AsyncTask {
    const NAME: &'static str = "AsyncTask";
}
impl ObjectImpl for AsyncTask {}

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
        NodeRender::builder()
            // .border_top(2.)
            // .border_right(2.)
            // .border_bottom(2.)
            .border_left(2.)
            .build()
    }
}

pub struct SimpContent {
    name: &'static str,
}
impl TreeViewObject for SimpContent {
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
        NodeRender::default()
    }
}

pub struct Content {
    val: String,
    idx: u32,
}
impl TreeViewObject for Content {
    #[inline]
    fn cells(&self) -> Vec<Cell> {
        vec![
            Cell::string()
                .value(self.val.clone())
                .cell_render(TextCellRender::builder().color(Color::BLACK).build())
                .build(),
            Cell::string()
                .value(self.val.clone())
                .cell_render(TextCellRender::builder().color(Color::BLACK).build())
                .build(),
            Cell::u32()
                .value(self.idx)
                .cell_render(TextCellRender::builder().color(Color::BLACK).build())
                .build(),
        ]
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
