use tmui::{
    prelude::*, scroll_area::ScrollArea, scroll_bar::ScrollBarPosition, tlib::object::{ObjectImpl, ObjectSubclass}, widget::WidgetImpl
};

use super::stack::MyStack;

#[extends(Widget, Layout(SplitPane))]
pub struct MySplitPane {}

impl ObjectSubclass for MySplitPane {
    const NAME: &'static str = "MySplitPane";
}

impl ObjectImpl for MySplitPane {
    fn construct(&mut self) {
        self.parent_construct();
        self.set_hexpand(true);
        self.set_vexpand(true);

        let mut widget: Box<MyStack> = Object::new(&[]);
        widget.set_background(Color::CYAN);
        widget.set_vexpand(true);
        widget.set_hexpand(true);

        let mut scroll_area: Box<ScrollArea> = Object::new(&[]);
        scroll_area.set_scroll_bar_position(ScrollBarPosition::End);
        scroll_area.set_orientation(Orientation::Vertical);
        scroll_area.set_hexpand(true);
        scroll_area.set_vexpand(true);

        scroll_area.set_area(widget);
        self.add_child(scroll_area)
    }
}

impl WidgetImpl for MySplitPane {}
