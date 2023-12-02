use super::{tree_node::TreeNode, tree_store::TreeStore};
use crate::{
    prelude::*,
    scroll_bar::ScrollBar,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};
use std::ptr::NonNull;
use tlib::{nonnull_ref, run_after, skia_safe::textlayout::{TypefaceFontProvider, FontCollection, ParagraphStyle, TextStyle, ParagraphBuilder}};

const REPCHAR: &'static str = concat!(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    "abcdefgjijklmnopqrstuvwxyz",
    "0123456789./+@"
);

#[extends(Widget)]
#[run_after]
pub(crate) struct TreeViewImage {
    store: TreeStore,
    image: Vec<Option<NonNull<TreeNode>>>,
    scroll_bar: Option<NonNull<ScrollBar>>,

    row_height: i32,
}

impl ObjectSubclass for TreeViewImage {
    const NAME: &'static str = "TreeViewImage";
}

impl ObjectImpl for TreeViewImage {}

impl WidgetImpl for TreeViewImage {
    fn run_after(&mut self) {
        self.parent_run_after();

        self.store.initialize_buffer();

        self.image = self.store.get_image();
    }

    fn paint(&mut self, mut painter: Painter) {
        let rect = self.contents_rect(Some(Coordinate::Widget));

        for (idx, node) in self.image.iter().enumerate() {
            let y_offset = idx as i32 * self.row_height;
            let geometry = Rect::new(rect.x(), rect.y() + y_offset, rect.width(), self.row_height);
            nonnull_ref!(node).render_node(&mut painter, geometry)
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

        self.row_height = paragraph.height() as i32;
    }
}

impl TreeViewImage {
    #[inline]
    pub fn new(scroll_bar: &mut ScrollBar) -> Box<Self> {
        let mut image: Box<TreeViewImage> = Object::new(&[]);
        image.scroll_bar = NonNull::new(scroll_bar);
        image
    }
}
