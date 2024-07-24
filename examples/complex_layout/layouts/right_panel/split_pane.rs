use std::time::Duration;

use tlib::{connect, timer::Timer};
use tmui::{
    application,
    prelude::*,
    scroll_area::ScrollArea,
    scroll_bar::ScrollBarPosition,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

use super::stack::MyStack;

#[extends(Widget, Layout(SplitPane))]
pub struct MySplitPane {
    timer: Timer,
    idx: usize,
}

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
        self.add_child(scroll_area);

        self.timer.start(Duration::from_millis(
            application::cursor_blinking_time() as u64
        ));
        connect!(self.timer, timeout(), self, try_update());
    }
}

impl WidgetImpl for MySplitPane {}

impl MySplitPane {
    fn try_update(&mut self) {
        if self.idx % 2 == 0 {
            self.update_rect(CoordRect::new(Rect::new(0, 0, 30, 30), Coordinate::Widget));
        } else {
            self.update();
        }

        if self.idx == usize::MAX {
            self.idx = 0;
        } else {
            self.idx += 1;
        }
    }
}
