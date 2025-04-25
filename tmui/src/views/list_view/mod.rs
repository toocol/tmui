pub mod list_group;
pub mod list_item;
pub mod list_node;
pub mod list_separator;
pub mod list_store;
pub mod list_view_image;
pub mod list_view_object;
use super::node::MouseEffect;
use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{InnerEventProcess, WidgetHndAsable, WidgetImpl},
};
use list_group::ListGroup;
use list_node::ListNode;
use list_separator::GroupSeparator;
use list_store::{ConcurrentStore, ConcurrentStoreMutexGuard, ListStore};
use list_view_image::ListViewImage;
use list_view_object::ListViewObject;
use std::sync::Arc;
use tipc::parking_lot::Mutex;
use tlib::connect;

/// UI component displays data in a list manner.
///
/// Certain functions (such as node hover display, handle mouse enter/leave) need to invoke [`set_mouse_tracking(true)`](crate::widget::widget_ext::WidgetExt::set_mouse_tracking).
///
/// Usage:
/// ```
/// use tmui::{
///    prelude::*,
///    views::{
///        cell::{
///            cell_render::TextCellRender,
///            Cell,
///        },
///        list_view::{list_group::ListGroup, list_view_object::ListViewObject, ListView},
///        node::node_render::NodeRender,
///    },
/// };
///
/// struct Node {
///    name: String,
/// }
/// impl ListViewObject for Node {
///    #[inline]
///    fn cells(&self) -> Vec<Cell> {
///        vec![Cell::string()
///                .value(self.name.clone())
///                .cell_render(TextCellRender::builder().color(Color::BLACK).build())
///                .build()]
///    }
///
///    #[inline]
///    fn node_render(&self) -> NodeRender {
///        NodeRender::builder().build()
///    }
/// }
///
/// fn test_build_ui() {
///     let mut list_view = ListView::new();
///     list_view.add_node(&Node { name: "test".to_string() });
/// }
/// ```
#[extends(Widget, Layout(ScrollArea))]
#[popupable]
pub struct ListView {}

impl ObjectSubclass for ListView {
    const NAME: &'static str = "ListView";
}

impl ObjectImpl for ListView {
    fn construct(&mut self) {
        self.parent_construct();

        let mut img = ListViewImage::new(self.scroll_bar_mut());
        img.store.set_view(self.as_hnd());
        img.set_scroll_bar(self.scroll_bar_mut());

        connect!(self, background_changed(), img, set_background(Color));
        connect!(self, invalid_area_changed(), img, set_invalid_area(FRect));
        connect!(img, mouse_leave(), self, image_mouse_leave(MouseEvent));
        connect!(img, mouse_enter(), self, image_mouse_enter(MouseEvent));

        self.set_area(img);
    }

    #[allow(clippy::single_match)]
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

impl WidgetImpl for ListView {
    #[inline]
    fn enable_focus(&self) -> bool {
        true
    }
}

impl ListView {
    #[inline]
    pub fn new() -> Tr<Self> {
        Self::new_alloc()
    }

    /// @return the index of added node.
    #[inline]
    pub fn add_node(&mut self, obj: &dyn ListViewObject) -> usize {
        self.get_store_mut().add_node(obj)
    }

    /// @return the index of added node.
    #[inline]
    pub fn add_node_directly(&mut self, node: ListNode) -> usize {
        self.get_store_mut().add_node_directly(node)
    }

    /// @return
    /// - Some: the index of last node in items.
    /// - None: if group is empty.
    #[inline]
    pub fn add_group(&mut self, group: ListGroup) -> Option<usize> {
        self.get_store_mut().add_group(group)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.get_store_mut().clear()
    }

    #[inline]
    pub fn get_store(&self) -> &ListStore {
        &self.get_image().store
    }

    #[inline]
    pub fn get_store_mut(&mut self) -> &mut ListStore {
        &mut self.get_image_mut().store
    }

    #[inline]
    pub fn set_line_spacint(&mut self, line_spacing: i32) {
        self.get_image_mut().set_line_spacing(line_spacing)
    }

    /// This funcition should be called before [`concurrent_store()`](ListView::concurrent_store),
    /// otherwise it may cause a long wait.
    #[inline]
    pub fn set_group_separator(&mut self, group_separator: GroupSeparator) {
        self.get_store_mut().set_group_separator(group_separator)
    }

    #[inline]
    pub fn concurrent_store(&mut self) -> Arc<Mutex<ConcurrentStore>> {
        self.get_store_mut().concurrent_store()
    }

    #[inline]
    pub fn start_loading(&mut self) {
        self.get_image_mut().start_loading()
    }

    #[inline]
    pub fn stop_loading(&mut self) {
        self.get_image_mut().stop_loading()
    }

    #[inline]
    pub fn scroll_to(&mut self, idx: usize) {
        self.get_store_mut().scroll_to_index(idx);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.get_store().nodes_len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn get_line_spacing(&self) -> i32 {
        self.get_image().line_spacing
    }

    #[inline]
    pub fn set_line_spacing(&mut self, line_spacing: i32) {
        self.get_image_mut().line_spacing = line_spacing;
    }

    #[inline]
    pub fn get_line_height(&self) -> i32 {
        self.get_image().line_height
    }

    #[inline]
    pub fn set_line_height(&mut self, line_height: i32) {
        self.get_image_mut().line_height = line_height;
        self.get_image_mut().custom_line_height = true;
    }

    #[inline]
    pub fn mouse_effect(&self) -> MouseEffect {
        self.get_image().mouse_effect
    }

    #[inline]
    pub fn set_mouse_effect(&mut self, mouse_effect: MouseEffect) {
        self.get_image_mut().mouse_effect = mouse_effect
    }

    #[inline]
    pub fn disable_mouse_effect(&mut self, mouse_effect: MouseEffect) {
        self.get_image_mut().mouse_effect.remove(mouse_effect)
    }

    #[inline]
    pub fn enable_mouse_effect(&mut self, mouse_effect: MouseEffect) {
        self.get_image_mut().mouse_effect.insert(mouse_effect)
    }

    #[inline]
    pub fn register_node_enter<
        F: 'static + Fn(&mut ListNode, &mut ConcurrentStoreMutexGuard, &MouseEvent),
    >(
        &mut self,
        f: F,
    ) {
        self.get_image_mut().on_node_enter = Some(Box::new(f));
    }

    #[inline]
    pub fn register_node_leave<
        F: 'static + Fn(&mut ListNode, &mut ConcurrentStoreMutexGuard, &MouseEvent),
    >(
        &mut self,
        f: F,
    ) {
        self.get_image_mut().on_node_leave = Some(Box::new(f));
    }

    #[inline]
    pub fn register_node_pressed<
        F: 'static + Fn(&mut ListNode, &mut ConcurrentStoreMutexGuard, &MouseEvent),
    >(
        &mut self,
        f: F,
    ) {
        self.get_image_mut().on_node_pressed = Some(Box::new(f));
    }

    #[inline]
    pub fn register_node_released<
        F: 'static + Fn(&mut ListNode, &mut ConcurrentStoreMutexGuard, &MouseEvent),
    >(
        &mut self,
        f: F,
    ) {
        self.get_image_mut().on_node_released = Some(Box::new(f));
    }

    #[inline]
    pub fn register_free_area_pressed<F: 'static + Fn(&mut dyn WidgetImpl, &MouseEvent)>(
        &mut self,
        f: F,
    ) {
        self.get_image_mut().on_free_area_pressed = Some(Box::new(f));
    }

    #[inline]
    pub fn register_free_area_released<F: 'static + Fn(&mut dyn WidgetImpl, &MouseEvent)>(
        &mut self,
        f: F,
    ) {
        self.get_image_mut().on_free_area_released = Some(Box::new(f));
    }
}

impl ListView {
    #[inline]
    pub(crate) fn get_image(&self) -> &ListViewImage {
        self.get_area_cast::<ListViewImage>().unwrap()
    }

    #[inline]
    pub(crate) fn get_image_mut(&mut self) -> &mut ListViewImage {
        self.get_area_cast_mut::<ListViewImage>().unwrap()
    }

    #[inline]
    pub(crate) fn image_mouse_enter(&mut self, evt: MouseEvent) {
        self.inner_mouse_enter(&evt);
        self.on_mouse_enter(&evt);
    }

    #[inline]
    pub(crate) fn image_mouse_leave(&mut self, evt: MouseEvent) {
        self.inner_mouse_leave(&evt);
        self.on_mouse_leave(&evt);
    }
}
