use super::{
    list_item::{ItemType, ListItem, ListItemCast, RenderCtx},
    list_node::ListNode,
    list_separator::GroupSeparator,
    list_store::{ListStore, ListStoreSignals},
};
use crate::{
    font::FontCalculation,
    prelude::*,
    scroll_bar::ScrollBar,
    tlib::object::{ObjectImpl, ObjectSubclass},
    views::node::Status,
    widget::{IterExecutor, RegionClear, WidgetImpl},
};
use std::ptr::NonNull;
use tlib::{
    connect, disconnect, events::MouseEvent, iter_executor, nonnull_mut, ptr_mut, run_after,
};

type FnNodeAction = Box<dyn Fn(&mut ListNode, &MouseEvent)>;
type FnAreaAction = Box<dyn Fn(&mut dyn WidgetImpl, &MouseEvent)>;

#[extends(Widget)]
#[loadable]
#[run_after]
#[iter_executor]
pub(crate) struct ListViewImage {
    pub(crate) store: ListStore,
    scroll_bar: Option<NonNull<ScrollBar>>,

    indent_length: i32,
    #[derivative(Default(value = "1"))]
    line_height: i32,
    line_spacing: i32,

    pub(crate) on_node_enter: Option<FnNodeAction>,
    pub(crate) on_node_leave: Option<FnNodeAction>,
    pub(crate) on_node_pressed: Option<FnNodeAction>,
    pub(crate) on_node_released: Option<FnNodeAction>,
    pub(crate) on_free_area_pressed: Option<FnAreaAction>,
    pub(crate) on_free_area_released: Option<FnAreaAction>,
}

impl ObjectSubclass for ListViewImage {
    const NAME: &'static str = "ListViewImage";
}
impl ObjectImpl for ListViewImage {
    fn initialize(&mut self) {
        self.set_mouse_tracking(true);

        connect!(
            self.store,
            internal_scroll_value_changed(),
            self,
            internal_scroll_value_changed(i32)
        );
        connect!(
            self.store,
            items_len_changed(),
            self,
            on_items_changed(usize)
        );
        connect!(
            nonnull_mut!(self.scroll_bar),
            value_changed(),
            self,
            scroll_bar_value_changed(i32)
        );
        connect!(self, size_changed(), self, on_size_changed(Size));
    }
}

impl WidgetImpl for ListViewImage {
    #[inline]
    fn run_after(&mut self) {
        self.font_changed();

        self.on_items_changed(self.store.get_items_len());
    }

    #[inline]
    fn paint(&mut self, painter: &mut Painter) {
        self.draw_image(painter)
    }

    #[inline]
    fn font_changed(&mut self) {
        self.on_font_changed();

        self.calc_window_lines();
    }

    #[inline]
    fn on_mouse_move(&mut self, event: &MouseEvent) {
        self.handle_mouse_move(event)
    }

    #[inline]
    fn on_mouse_pressed(&mut self, event: &MouseEvent) {
        self.handle_mouse_pressed(event)
    }

    #[inline]
    fn on_mouse_released(&mut self, event: &MouseEvent) {
        self.handle_mouse_released(event)
    }

    #[inline]
    fn on_mouse_wheel(&mut self, event: &MouseEvent) {
        nonnull_mut!(self.scroll_bar).on_mouse_wheel(event)
    }
}

impl ListViewImage {
    #[inline]
    pub(crate) fn new(scroll_bar: &mut ScrollBar) -> Box<Self> {
        let mut img: Box<ListViewImage> = Object::new(&[]);
        img.scroll_bar = NonNull::new(scroll_bar);
        img
    }

    #[inline]
    pub(crate) fn set_scroll_bar(&mut self, scroll_bar: &mut ScrollBar) {
        self.scroll_bar = NonNull::new(scroll_bar)
    }

    #[inline]
    pub(crate) fn set_line_spacing(&mut self, line_spacing: i32) {
        self.line_spacing = line_spacing;
    }
}

impl IterExecutor for ListViewImage {
    #[inline]
    fn iter_execute(&mut self) {
        self.store.check_arc_count();
    }
}

impl ListViewImage {
    #[inline]
    fn draw_image(&mut self, painter: &mut Painter) {
        if self.store.occupied() {
            return
        }

        let mut rect = self.contents_rect_f(Some(Coordinate::Widget));
        self.clear(painter, rect);

        let background = self.opaque_background();
        rect.set_x(rect.x() + self.indent_length as f32);
        rect.set_width(rect.width() - self.indent_length as f32);

        let separator_height = self.store.separator_height() as f32;

        self.store.with_image(|image, y_offset| {
            let height = if let Some(first) = image.first() {
                match first.item_type() {
                    ItemType::Node => (self.line_height + self.line_spacing) as f32,
                    ItemType::Separator => separator_height,
                }
            } else {
                0.
            };
            let mut offset = rect.y() - (y_offset as f32 / 10. * height);

            for item in image {
                rect.set_y(offset);
                match item.item_type() {
                    ItemType::Node => {
                        offset += self.line_height as f32 + self.line_spacing as f32;
                        rect.set_height(self.line_height as f32);
                    }
                    ItemType::Separator => {
                        let separator_height =
                            item.downcast_ref::<GroupSeparator>()
                                .unwrap()
                                .separator_height() as f32;
                        offset += separator_height;
                        rect.set_height(separator_height);
                    }
                }

                item.render(painter, RenderCtx::new(rect, background))
            }
        });
    }

    #[inline]
    fn on_font_changed(&mut self) {
        let (_, h) = self.font().calc_font_dimension();

        self.line_height = h as i32;
    }

    #[inline]
    fn on_size_changed(&mut self, _: Size) {
        self.calc_window_lines();

        self.on_items_changed(self.store.get_items_len());
    }

    fn calc_window_lines(&mut self) {
        let size = self.size();
        let window_lines = size.height() / (self.line_height + self.line_spacing);
        self.store.set_window_lines(window_lines);

        let scroll_bar = nonnull_mut!(self.scroll_bar);
        scroll_bar.set_single_step(5);
        scroll_bar.set_page_step(window_lines * 10);
        scroll_bar.set_visible_area(window_lines * 10);
    }

    #[inline]
    fn scroll_bar_value_changed(&mut self, value: i32) {
        if self.store.scroll_to(value, false) {
            self.update();
        }
    }

    fn internal_scroll_value_changed(&mut self, value: i32) {
        let scroll_bar = nonnull_mut!(self.scroll_bar);
        disconnect!(scroll_bar, value_changed(), null, null);
        scroll_bar.set_value(value);
        connect!(
            scroll_bar,
            value_changed(),
            self,
            scroll_bar_value_changed(i32)
        );

        self.update();
    }

    #[inline]
    fn on_items_changed(&mut self, len: usize) {
        nonnull_mut!(self.scroll_bar)
            .set_range(0, (len as i32 - self.store.get_window_lines()) * 10);
    }

    fn handle_mouse_move(&mut self, event: &MouseEvent) {
        let scroll_bar = nonnull_mut!(self.scroll_bar);
        if scroll_bar.slider_pressed() {
            scroll_bar.on_mouse_move(event);
            return;
        }

        let (_, y) = event.position();
        let separator_height = self.store.separator_height() as f32;
        let rect = self.contents_rect_f(Some(Coordinate::Widget));

        let update = self
            .store
            .with_image_mut(|image, y_offset, entered_node, hovered_node, _| {
                let item = index_item(
                    image,
                    y_offset as f32,
                    rect,
                    separator_height,
                    self.line_height,
                    self.line_spacing,
                    y as f32,
                );

                if let Some(item) = item {
                    if item.item_type() == ItemType::Separator {
                        if hovered_node.is_some() {
                            let node = nonnull_mut!(hovered_node.take());
                            if !node.is_selected() {
                                node.set_status(Status::Default);
                            }
                            return true;
                        }
                        return false;
                    }

                    let mut update = false;
                    let node = item.downcast_mut::<ListNode>().unwrap();

                    // Handle node hover:
                    if !node.is_hovered() {
                        if hovered_node.is_some() {
                            let node = nonnull_mut!(hovered_node);
                            if !node.is_selected() {
                                node.set_status(Status::Default);
                            }
                            update = true;
                        }

                        if !node.is_selected() {
                            node.set_status(Status::Hovered);
                            *hovered_node = NonNull::new(node);
                            update = true;
                        }
                    }

                    // Handle node enter/leave:
                    if entered_node.is_none() {
                        *entered_node = NonNull::new(node);
                        if let Some(ref on_node_enter) = self.on_node_enter {
                            on_node_enter(node, event)
                        }
                    } else {
                        let previous_node = nonnull_mut!(entered_node);

                        if previous_node.id() != node.id() {
                            *entered_node = NonNull::new(node);

                            if let Some(ref on_node_leave) = self.on_node_leave {
                                on_node_leave(previous_node, event)
                            }
                            if let Some(ref on_node_enter) = self.on_node_enter {
                                on_node_enter(node, event);
                            }
                        }
                    }

                    return update;
                } else {
                    let mut old_hover = hovered_node.take();
                    if old_hover.is_some() {
                        let node = nonnull_mut!(old_hover);
                        if node.is_hovered() {
                            node.set_status(Status::Default);
                            return true;
                        }
                    }
                }

                false
            });

        if update {
            self.update();
        }
    }

    fn handle_mouse_pressed(&mut self, event: &MouseEvent) {
        let (_, y) = event.position();
        let separator_height = self.store.separator_height() as f32;
        let rect = self.contents_rect_f(Some(Coordinate::Widget));
        let parent = ptr_mut!(self.get_raw_parent_mut().unwrap());

        let update = self
            .store
            .with_image_mut(|image, y_offset, _, _, selected_node| {
                let item = index_item(
                    image,
                    y_offset as f32,
                    rect,
                    separator_height,
                    self.line_height,
                    self.line_spacing,
                    y as f32,
                );

                if let Some(item) = item {
                    if item.item_type() == ItemType::Separator {
                        return false;
                    }
                    let node = item.downcast_mut::<ListNode>().unwrap();

                    if selected_node.is_some() {
                        let node = nonnull_mut!(selected_node);
                        node.set_status(Status::Default);
                    }

                    node.set_status(Status::Selected);
                    *selected_node = NonNull::new(node);

                    if let Some(ref on_node_pressed) = self.on_node_pressed {
                        on_node_pressed(node, event);
                    }
                    true
                } else {
                    let mut update = false;
                    let mut old_select = selected_node.take();
                    if old_select.is_some() {
                        let node = nonnull_mut!(old_select);
                        node.set_status(Status::Default);
                        update = true;
                    }

                    if let Some(ref on_frea_area_pressed) = self.on_free_area_pressed {
                        on_frea_area_pressed(parent, event);
                    }

                    update
                }
            });

        if update {
            self.update();
        }
    }

    fn handle_mouse_released(&mut self, event: &MouseEvent) {
        let parent = ptr_mut!(self.get_raw_parent_mut().unwrap());
        let mut selected_node = self.store.get_selected_node();
        if selected_node.is_none() {
            if let Some(ref on_free_area_released) = self.on_free_area_released {
                on_free_area_released(parent, event);
            }
            return;
        }

        if let Some(ref on_node_released) = self.on_node_released {
            let node = nonnull_mut!(selected_node);
            on_node_released(node, event)
        }
    }
}

fn index_item(
    image: &mut [Box<dyn ListItem>],
    y_offset: f32,
    rect: FRect,
    separator_height: f32,
    line_height: i32,
    line_spacing: i32,
    y: f32,
) -> Option<&mut Box<dyn ListItem>> {
    if image.is_empty() {
        return None;
    }

    let mut height = if let Some(first) = image.first() {
        match first.item_type() {
            ItemType::Node => (line_height + line_spacing) as f32,
            ItemType::Separator => separator_height,
        }
    } else {
        0.
    };
    let mut offset = rect.y() - (y_offset / 10. * height);

    for item in image {
        height = match item.item_type() {
            ItemType::Node => (line_height + line_spacing) as f32,
            ItemType::Separator => separator_height,
        };

        if (offset..offset + height).contains(&y) {
            return Some(item);
        }

        offset += height;
    }

    None
}
