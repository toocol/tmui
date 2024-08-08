use super::{tree_node::TreeNode, tree_store::TreeStore};
use crate::{
    font::FontCalculation,
    prelude::*,
    scroll_bar::ScrollBar,
    tlib::object::{ObjectImpl, ObjectSubclass},
    views::tree_view::tree_store::TreeStoreSignals,
    widget::{RegionClear, WidgetImpl},
};
use std::ptr::NonNull;
use tlib::{connect, disconnect, events::MouseEvent, nonnull_mut, nonnull_ref, run_after};

type FnNodePressed = Box<dyn Fn(&mut TreeNode, &MouseEvent)>;
type FnNodeReleased = Box<dyn Fn(&mut TreeNode, &MouseEvent)>;
type FnNodeEnter = Box<dyn Fn(&mut TreeNode, &MouseEvent)>;
type FnNodeLeave = Box<dyn Fn(&mut TreeNode, &MouseEvent)>;
type FnFreeAreaPressed = Box<dyn Fn(&mut TreeNode, &MouseEvent)>;
type FnFreeAreaReleased = Box<dyn Fn(&mut TreeNode, &MouseEvent)>;

#[extends(Widget)]
#[run_after]
#[loadable]
pub(crate) struct TreeViewImage {
    store: Box<TreeStore>,
    scroll_bar: Option<NonNull<ScrollBar>>,

    #[derivative(Default(value = "20"))]
    indent_length: i32,
    #[derivative(Default(value = "1"))]
    line_height: i32,
    line_spacing: i32,

    on_node_pressed: Option<FnNodePressed>,
    on_node_released: Option<FnNodeReleased>,
    on_node_enter: Option<FnNodeEnter>,
    on_node_leave: Option<FnNodeLeave>,
    on_free_area_pressed: Option<FnFreeAreaPressed>,
    on_free_area_released: Option<FnFreeAreaReleased>,
}

impl ObjectSubclass for TreeViewImage {
    const NAME: &'static str = "TreeViewImage";
}

impl ObjectImpl for TreeViewImage {
    fn construct(&mut self) {
        self.parent_construct();
        self.set_mouse_tracking(true);

        self.store.prepare_store();
    }
}

impl WidgetImpl for TreeViewImage {
    #[inline]
    fn enable_focus(&self) -> bool {
        true
    }

    fn run_after(&mut self) {
        self.font_changed();

        self.calculate_window_lines();

        connect!(self.store, notify_update(), self, update());
        connect!(
            self.store,
            notify_update_rect(),
            self,
            notify_update_rect(usize)
        );
        connect!(
            self.store,
            buffer_len_changed(),
            self,
            when_nodes_buffer_changed(usize)
        );
        connect!(
            self.store,
            internal_scroll_value_changed(),
            self,
            internal_scroll_value_changed(i32)
        );
        connect!(
            nonnull_mut!(self.scroll_bar),
            value_changed(),
            self,
            scroll_bar_value_changed(i32)
        );
        connect!(self, size_changed(), self, when_size_changed(Size));

        self.when_nodes_buffer_changed(self.store.get_nodes_buffer_len());
    }

    fn paint(&mut self, painter: &mut Painter) {
        let rect = self.contents_rect(Some(Coordinate::Widget));

        if self.redraw_region().is_empty() {
            self.clear(painter, rect);
        } else {
            for redraw_rect in self.redraw_region().iter() {
                self.clear(painter, redraw_rect.rect());
            }
        }

        let (image, y_offset) = self.store.get_image();
        let mut offset = rect.y()
            - (y_offset as f32 / 10. * (self.line_height + self.line_spacing) as f32) as i32;

        for node in image.iter() {
            let geometry = Rect::new(rect.x(), offset, rect.width(), self.line_height);
            offset += self.line_height + self.line_spacing;

            nonnull_ref!(node).render_node(
                painter,
                geometry,
                self.opaque_background(),
                self.indent_length,
            );

            painter.draw_rect(nonnull_ref!(node).rect(Coordinate::Widget).unwrap());
        }
    }

    fn font_changed(&mut self) {
        let (_, h) = self.font().calc_font_dimension();

        self.line_height = h as i32;
    }

    fn on_mouse_move(&mut self, event: &MouseEvent) {
        let scroll_bar = nonnull_mut!(self.scroll_bar);
        if scroll_bar.slider_pressed() {
            scroll_bar.on_mouse_move(event);
            return;
        }

        let (_, y) = event.position();
        let idx = self.index_node(y);

        self.store.hover_node(idx);

        // Handle the mouse enter/leave event:
        let mut entered_node = self.store.get_entered_node();
        let mut node_ptr = self.store.get_image_node_ptr(idx);
        if node_ptr.is_some() && (self.on_node_enter.is_some() || self.on_node_leave.is_some()) {
            let node = nonnull_mut!(node_ptr);

            if entered_node.is_none() {
                self.store.set_entered_node(node);

                if let Some(ref on_node_enter) = self.on_node_enter {
                    on_node_enter(node, event);
                }
            } else {
                let previous_node = nonnull_mut!(entered_node);

                if previous_node.id() != node.id() {
                    self.store.set_entered_node(node);

                    if let Some(ref on_node_leave) = self.on_node_leave {
                        on_node_leave(previous_node, event)
                    }
                    if let Some(ref on_node_enter) = self.on_node_enter {
                        on_node_enter(node, event);
                    }
                }
            }
        }
    }

    fn on_mouse_pressed(&mut self, event: &MouseEvent) {
        let (_, y) = event.position();
        let idx = self.index_node(y);

        self.store.click_node(idx, event.mouse_button());

        if let Some(node) = self.store.get_image_node(idx) {
            if let Some(ref on_node_pressed) = self.on_node_pressed {
                on_node_pressed(node, event);
            }
        } else if let Some(ref on_free_area_pressed) = self.on_free_area_pressed {
            on_free_area_pressed(self.store.root_mut(), event)
        }
    }

    fn on_mouse_released(&mut self, event: &MouseEvent) {
        let (_, y) = event.position();
        let idx = self.index_node(y);

        if let Some(node) = self.store.get_image_node(idx) {
            if let Some(ref on_node_released) = self.on_node_released {
                on_node_released(node, event);
            }
        } else if let Some(ref on_free_area_released) = self.on_free_area_released {
            on_free_area_released(self.store.root_mut(), event)
        }
    }

    fn on_mouse_wheel(&mut self, event: &MouseEvent) {
        nonnull_mut!(self.scroll_bar).on_mouse_wheel(event)
    }
}

impl TreeViewImage {
    #[inline]
    pub fn new(scroll_bar: &mut ScrollBar) -> Box<Self> {
        let mut image: Box<TreeViewImage> = Object::new(&[]);
        image.scroll_bar = NonNull::new(scroll_bar);
        image
    }

    #[inline]
    pub fn get_store(&self) -> &TreeStore {
        &self.store
    }

    #[inline]
    pub fn get_store_mut(&mut self) -> &mut TreeStore {
        &mut self.store
    }

    #[inline]
    pub fn set_indent_length(&mut self, indent_length: i32) {
        self.indent_length = indent_length
    }

    #[inline]
    pub fn indent_length(&self) -> i32 {
        self.indent_length
    }

    #[inline]
    pub fn set_line_spacing(&mut self, line_spacing: i32) {
        self.line_spacing = line_spacing
    }

    #[inline]
    pub fn line_spacing(&self) -> i32 {
        self.line_spacing
    }

    #[inline]
    pub fn line_height(&self) -> i32 {
        self.line_height
    }
}

impl TreeViewImage {
    #[inline]
    pub(crate) fn when_size_changed(&mut self, _size: Size) {
        self.calculate_window_lines();

        self.when_nodes_buffer_changed(self.store.buffer_len());
    }

    #[inline]
    pub(crate) fn calculate_window_lines(&mut self) {
        let size = self.size();
        let window_lines = size.height() / (self.line_height + self.line_spacing);
        self.store.set_window_lines(window_lines);

        let scroll_bar = nonnull_mut!(self.scroll_bar);
        scroll_bar.set_single_step(4);
        scroll_bar.set_page_step(window_lines * 10);
        scroll_bar.set_visible_area(window_lines * 10);
    }

    #[inline]
    pub(crate) fn index_node(&self, y: i32) -> usize {
        let y_offset = (self.store.y_offset() as f32 / 10.
            * (self.line_height + self.line_spacing) as f32) as i32;
        ((y + y_offset) / (self.line_height + self.line_spacing)) as usize
    }

    #[inline]
    pub(crate) fn notify_update_rect(&mut self, start_idx: usize) {
        let size = self.size();
        let x = 0;
        let y = start_idx as i32 * (self.line_height + self.line_spacing);

        if y >= size.height() {
            return;
        }

        let width = size.width();
        let height = size.height() - y;

        let rect: Rect = (x, y, width, height).into();

        if rect.is_valid() {
            self.update_rect(CoordRect::new(rect, Coordinate::Widget))
        }
    }

    #[inline]
    pub(crate) fn when_nodes_buffer_changed(&mut self, buffer_len: usize) {
        let scroll_bar = nonnull_mut!(self.scroll_bar);

        scroll_bar.set_range(0, (buffer_len as i32 - self.store.get_window_lines()) * 10);
    }

    #[inline]
    pub(crate) fn scroll_bar_value_changed(&mut self, value: i32) {
        if self.store.scroll_to(value, false) {
            self.update()
        }
    }

    #[inline]
    pub(crate) fn internal_scroll_value_changed(&mut self, value: i32) {
        let scroll_bar = nonnull_mut!(self.scroll_bar);
        disconnect!(scroll_bar, value_changed(), null, null);
        scroll_bar.set_value(value);
        connect!(
            scroll_bar,
            value_changed(),
            self,
            scroll_bar_value_changed(i32)
        );

        self.update_rect(CoordRect::new(self.rect(), Coordinate::World));
    }

    #[inline]
    pub(crate) fn register_node_pressed<T: 'static + Fn(&mut TreeNode, &MouseEvent)>(
        &mut self,
        f: T,
    ) {
        self.on_node_pressed = Some(Box::new(f))
    }

    #[inline]
    pub(crate) fn register_node_released<T: 'static + Fn(&mut TreeNode, &MouseEvent)>(
        &mut self,
        f: T,
    ) {
        self.on_node_released = Some(Box::new(f))
    }

    #[inline]
    pub(crate) fn register_node_enter<T: 'static + Fn(&mut TreeNode, &MouseEvent)>(
        &mut self,
        f: T,
    ) {
        self.on_node_enter = Some(Box::new(f))
    }

    #[inline]
    pub(crate) fn register_node_leave<T: 'static + Fn(&mut TreeNode, &MouseEvent)>(
        &mut self,
        f: T,
    ) {
        self.on_node_leave = Some(Box::new(f))
    }

    #[inline]
    pub(crate) fn register_free_area_pressed<T: 'static + Fn(&mut TreeNode, &MouseEvent)>(
        &mut self,
        f: T,
    ) {
        self.on_free_area_pressed = Some(Box::new(f))
    }

    #[inline]
    pub(crate) fn register_free_area_released<T: 'static + Fn(&mut TreeNode, &MouseEvent)>(
        &mut self,
        f: T,
    ) {
        self.on_free_area_released = Some(Box::new(f))
    }
}
