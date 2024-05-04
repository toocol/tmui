#![allow(dead_code)]
use std::time::Duration;
use tlib::{
    actions::ActionExt, connect, global::SemanticExt, namespace::MouseButton, timer::Timer,
    tokio::task::JoinHandle,
};
use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    tree_view::{
        cell::{cell_render::TextCellRender, Cell},
        node_render::NodeRender,
        tree_node::TreeNode,
        tree_view_object::TreeViewObject,
        TreeView,
    },
    widget::{WidgetImpl, WidgetImplExt},
};

use crate::ctx_menu::CtxMenu;

const DATA_SIZE: i32 = 500000;

#[extends(Widget)]
#[derive(Childable)]
#[async_task(name = "BuildTreeTask", value = "Box<TreeNode>")]
pub struct TreeViewHolder {
    #[child]
    tree_view: Box<TreeView>,
    // task: Option<Box<AsyncTask>>,
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
            let view = node.get_view().downcast_mut::<TreeView>().unwrap();
            view.show_popup(view.map_to_global(&evt.position().into()));
        });
        self.tree_view.register_node_enter(|node, _| {
            println!("Node enter, id = {}", node.id());
        });
        self.tree_view.register_node_leave(|node, _| {
            println!("Node leave, id = {}", node.id());
        });

        self.set_hexpand(true);
        self.set_vexpand(true);
        self.set_hscale(0.3);
        self.set_halign(Align::Center);

        let store = self.tree_view.get_store_mut();

        let id = store.id();
        let level = store.root_mut().level();

        self.build_tree_task(
            async move {
                let mut group1 = TreeNode::create(id, level + 1, &Group { name: "group-1" });

                for i in 0..10 {
                    let content = format!("content_{}", i);
                    group1.add_node(&Content { val: content });
                }

                if let Some(group2) = group1.add_node(&Group { name: "group-2" }) {
                    for i in 0..DATA_SIZE {
                        let content = format!("sub_content_{}", i);
                        group2.add_node(&Content { val: content });
                    }
                }

                for i in 10..15 {
                    let content = format!("content_{}", i);
                    group1.add_node(&Content { val: content });
                }

                if let Some(group3) = group1.add_node(&Group { name: "group-3" }) {
                    for i in 0..DATA_SIZE {
                        let content = format!("sub_content_{}", i);
                        group3.add_node(&Content { val: content });
                    }
                }

                group1
            },
            Some(|w: &mut TreeViewHolder, node| {
                let root_mut = w.tree_view.get_store_mut().root_mut();

                root_mut.add_node_directly(node);
                root_mut.notify_update();

                w.tree_view.stop_loading();
            }),
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
            .border_top(2.)
            .border_right(2.)
            .border_bottom(2.)
            .border_left(2.)
            .build()
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
