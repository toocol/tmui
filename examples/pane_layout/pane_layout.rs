use tmui::{
    label::Label,
    prelude::*,
    tlib::{
        namespace::Orientation,
        object::{ObjectImpl, ObjectSubclass},
    },
    widget::WidgetImpl,
};

#[extends(Widget, Layout(Pane))]
#[derive(Childrenable)]
pub struct PaneLayout {
    #[children]
    left: Tr<Label>,

    #[children]
    right: Tr<Label>,
}

impl ObjectSubclass for PaneLayout {
    const NAME: &'static str = "PaneLayoutWidget";
}

impl ObjectImpl for PaneLayout {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_orientation(Orientation::Vertical);
        self.set_hexpand(true);
        self.set_vexpand(true);

        self.left.set_text("Left child.");
        self.left.set_background(Color::GREY_MEDIUM);
        self.left.set_hexpand(true);
        self.left.set_vexpand(true);
        self.left
            .set_size_hint(SizeHint::new().with_min_width(200).with_min_height(200));

        self.right.set_text("Right child.");
        self.right.set_background(Color::MAGENTA);
        self.right.set_hexpand(true);
        self.right.set_vexpand(true);
    }
}

impl WidgetImpl for PaneLayout {}

impl PaneLayout {
    #[inline]
    #[allow(dead_code)]
    pub fn new() -> Tr<Self> {
        Self::new_alloc()
    }
}
