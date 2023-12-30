use super::tree_store::TreeStore;
use crate::{
    prelude::*,
    scroll_bar::ScrollBar,
    tlib::object::{ObjectImpl, ObjectSubclass},
    tree_view::tree_store::TreeStoreSignals,
    widget::WidgetImpl,
};
use std::ptr::NonNull;
use tlib::{
    connect,
    events::MouseEvent,
    nonnull_mut, nonnull_ref, run_after,
    skia_safe::textlayout::{
        FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle, TypefaceFontProvider,
    },
};

const REPCHAR: &'static str = concat!(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    "abcdefgjijklmnopqrstuvwxyz",
    "0123456789./+@"
);

#[extends(Widget)]
#[run_after]
pub(crate) struct TreeViewImage {
    store: Box<TreeStore>,
    scroll_bar: Option<NonNull<ScrollBar>>,

    #[derivative(Default(value = "20"))]
    indent_length: i32,
    #[derivative(Default(value = "1"))]
    line_height: i32,
    line_spacing: i32,
}

impl ObjectSubclass for TreeViewImage {
    const NAME: &'static str = "TreeViewImage";
}

impl ObjectImpl for TreeViewImage {
    fn construct(&mut self) {
        self.parent_construct();

        self.store.prepare_store();
    }
}

impl WidgetImpl for TreeViewImage {
    fn run_after(&mut self) {
        self.parent_run_after();

        self.set_mouse_tracking(true);

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
            nonnull_mut!(self.scroll_bar),
            value_changed(),
            self,
            scroll_bar_value_changed(i32)
        );
        connect!(self, size_changed(), self, when_size_changed(Size));
    }

    fn paint(&mut self, mut painter: &mut Painter) {
        for redraw_rect in self.redraw_region().iter() {
            painter.fill_rect(*redraw_rect, self.background());
        }

        let rect = self.contents_rect(Some(Coordinate::Widget));

        for (idx, node) in self.store.get_image().iter().enumerate() {
            let i = idx as i32;
            let y_offset = i * self.line_height + i * self.line_spacing;

            let geometry = Rect::new(
                rect.x(),
                rect.y() + y_offset,
                rect.width(),
                self.line_height,
            );

            nonnull_ref!(node).render_node(
                &mut painter,
                geometry,
                self.background(),
                self.indent_length,
            )
        }
    }

    fn font_changed(&mut self) {
        let font = self.font().to_skia_font();
        let typeface = font.typeface().unwrap();

        let mut typeface_provider = TypefaceFontProvider::new();
        let family = typeface.family_name();
        typeface_provider.register_typeface(typeface, Some(family.clone()));

        let mut font_collection = FontCollection::new();
        font_collection.set_asset_font_manager(Some(typeface_provider.clone().into()));

        // define text style
        let mut style = ParagraphStyle::new();
        let mut text_style = TextStyle::new();
        text_style.set_font_size(font.size());
        text_style.set_font_families(&vec![family]);
        text_style.set_letter_spacing(0.);
        style.set_text_style(&text_style);

        // layout the paragraph
        let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
        paragraph_builder.add_text(REPCHAR);
        let mut paragraph = paragraph_builder.build();
        paragraph.layout(f32::MAX);

        self.line_height = paragraph.height() as i32;
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
    }

    fn on_mouse_pressed(&mut self, event: &MouseEvent) {
        let (_, y) = event.position();
        let idx = self.index_node(y);

        self.store.click_node(idx);
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
    pub fn set_line_spacing(&mut self, line_spacing: i32) {
        self.line_spacing = line_spacing
    }
}

impl TreeViewImage {
    #[inline]
    pub(crate) fn when_size_changed(&mut self, _size: Size) {
        self.calculate_window_lines()
    }

    #[inline]
    pub(crate) fn calculate_window_lines(&mut self) {
        let size = self.size();
        let window_lines = size.height() / (self.line_height + self.line_spacing);
        self.store.set_window_lines(window_lines);

        let scroll_bar = nonnull_mut!(self.scroll_bar);
        scroll_bar.set_single_step(1);
        scroll_bar.set_page_step(window_lines);
    }

    #[inline]
    pub(crate) fn index_node(&self, y: i32) -> usize {
        (y / (self.line_height + self.line_spacing)) as usize
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
        let height = size.height() - y as i32;

        let rect: Rect = (x, y, width, height).into();

        if rect.is_valid() {
            self.update_rect(rect)
        }
    }

    pub(crate) fn when_nodes_buffer_changed(&mut self, buffer_len: usize) {
        let scroll_bar = nonnull_mut!(self.scroll_bar);

        scroll_bar.set_range(0, buffer_len as i32 - self.store.get_window_lines());
    }

    pub(crate) fn scroll_bar_value_changed(&mut self, value: i32) {
        if self.store.scroll_to(value) {
            self.update()
        }
    }
}
