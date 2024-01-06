pub mod cell;
pub mod node_render;
pub mod tree_node;
pub mod tree_store;
pub mod tree_view_image;
pub mod tree_view_object;

use self::{tree_node::TreeNode, tree_store::TreeStore, tree_view_image::TreeViewImage};
use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};
use tlib::{connect, signals};

/// Tree components display data in a hierarchical manner.
///
/// Certain functions (such as node hover display, handle mouse enter/leave) need to invoke [`set_mouse_tracking(true)`](crate::widget::widget_ext::WidgetExt::set_mouse_tracking).
///
/// Basic usage:
/// ```
/// use tmui::tree_view::{
///     tree_view_object::TreeViewObject, cell::{Cell, cell_render::TextCellRender},
///     node_render::NodeRender, TreeView,
/// };
/// use tmui::tlib::figure::color::Color;
///
/// pub struct Content {
///     val: String,
/// }
/// impl TreeViewObject for Content {
///    #[inline]
///    fn cells(&self) -> Vec<Cell> {
///        vec![Cell::string()
///            .value(self.val.clone())
///            .cell_render(TextCellRender::builder().color(Color::BLACK).build())
///            .build()]
///    }
///
///    #[inline]
///    fn extensible(&self) -> bool {
///        false
///    }
///
///    #[inline]
///    fn node_render(&self) -> NodeRender {
///        NodeRender::default()
///   }
/// }
/// 
/// fn test_build_ui() {
///     let mut tree_view = TreeView::new();
///     let _node_added = tree_view
///                         .get_store_mut()
///                         .root_mut()
///                         .add_node(&Content { val: "test".to_string() });
/// }
/// ```
#[extends(Widget, Layout(ScrollArea), internal = true)]
#[popupable]
pub struct TreeView {}

impl ObjectSubclass for TreeView {
    const NAME: &'static str = "TreeView";
}

impl ObjectImpl for TreeView {
    fn construct(&mut self) {
        self.parent_construct();

        let mut image = TreeViewImage::new(self.get_scroll_bar_mut());

        connect!(self, background_changed(), image, set_background(Color));

        self.set_area(image);
    }

    fn on_property_set(&mut self, name: &str, value: &Value) {
        self.parent_on_property_set(name, value);

        match name {
            "mouse_tracking" => {
                let is_tracking = value.get::<bool>();
                self.get_image_mut().set_mouse_tracking(is_tracking);
            }
            _ => {}
        };
    }
}

impl WidgetImpl for TreeView {}

impl TreeView {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }

    #[inline]
    pub fn start_loading(&mut self) {
        self.get_image_mut().start_loading();
    }

    #[inline]
    pub fn stop_loading(&mut self) {
        self.get_image_mut().stop_loading();
    }

    #[inline]
    pub fn get_store(&self) -> &TreeStore {
        self.get_image().get_store()
    }

    #[inline]
    pub fn get_store_mut(&mut self) -> &mut TreeStore {
        self.get_image_mut().get_store_mut()
    }

    #[inline]
    pub fn set_indent_length(&mut self, indent_length: i32) {
        self.get_image_mut().set_indent_length(indent_length)
    }

    #[inline]
    pub fn set_line_spacing(&mut self, line_spacing: i32) {
        self.get_image_mut().set_line_spacing(line_spacing)
    }

    /// Function clousure will be executed when mouse pressed the node.
    #[inline]
    pub fn register_node_pressed<T: 'static + Fn(&mut TreeNode, &MouseEvent)>(&mut self, f: T) {
        self.get_image_mut().register_node_pressed(f)
    }

    /// Function clousure will be executed when mouse released on the node.
    #[inline]
    pub fn register_node_released<T: 'static + Fn(&mut TreeNode, &MouseEvent)>(&mut self, f: T) {
        self.get_image_mut().register_node_released(f)
    }

    /// Function clousure will be executed when mouse enter the node.
    ///
    /// [`set_mouse_tracking(true)`](crate::widget::widget_ext::WidgetExt::set_mouse_tracking) to enable this.
    #[inline]
    pub fn register_node_enter<T: 'static + Fn(&mut TreeNode, &MouseEvent)>(&mut self, f: T) {
        self.get_image_mut().register_node_enter(f)
    }

    /// Function clousure will be executed when mouse leave the node.
    ///
    /// [`set_mouse_tracking(true)`](crate::widget::widget_ext::WidgetExt::set_mouse_tracking) to enable this.
    #[inline]
    pub fn register_node_leave<T: 'static + Fn(&mut TreeNode, &MouseEvent)>(&mut self, f: T) {
        self.get_image_mut().register_node_leave(f)
    }
}

impl TreeView {
    #[inline]
    pub(crate) fn get_image(&self) -> &TreeViewImage {
        self.get_area_cast::<TreeViewImage>().unwrap()
    }

    #[inline]
    pub(crate) fn get_image_mut(&mut self) -> &mut TreeViewImage {
        self.get_area_cast_mut::<TreeViewImage>().unwrap()
    }
}

pub trait TreeViewSignals: ActionExt {
    signals!(
        TreeViewSignals:

        selection_changed();

        row_activated();
    );
}
impl TreeViewSignals for TreeView {}
