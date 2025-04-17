use std::ptr::NonNull;
use tlib::{connect, events::MouseEvent};
use tmui::{
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget, Layout(SplitPane))]
pub struct SplitPaneLayout {
    left: ObjectId,
    right_top: ObjectId,
    right_bottom_left: ObjectId,
    right_bottom_right: ObjectId,
    cnt: i32,
    label: Option<NonNull<Label>>,
}

impl ObjectSubclass for SplitPaneLayout {
    const NAME: &'static str = "SplitPaneLayout";
}

impl ObjectImpl for SplitPaneLayout {
    fn construct(&mut self) {
        self.parent_construct();

        let mut label = Label::new(Some("Click to split."));
        label.set_size(20);
        label.set_background(Color::RED);
        label.set_content_halign(Align::Center);
        label.set_content_valign(Align::Center);
        label.set_mouse_tracking(true);
        self.left = label.id();

        connect!(label, mouse_pressed(), self, split_off(MouseEvent));
        self.add_child(label);

        let mut children_mut = self.children_mut();
        let label = children_mut[0].downcast_mut::<Label>().unwrap();
        self.label = NonNull::new(label);

        self.width_request(400);
        self.height_request(300);
        self.set_render_difference(true);
    }
}

impl WidgetImpl for SplitPaneLayout {}

impl SplitPaneLayout {
    pub fn remove_left(&mut self) {
        self.remove_children(self.left);
    }

    pub fn remove_right_top(&mut self) {
        self.remove_children(self.right_top);
    }

    pub fn remove_right_bottom_left(&mut self) {
        self.remove_children(self.right_bottom_left);
    }

    pub fn remove_right_bottom_right(&mut self) {
        self.remove_children(self.right_bottom_right);
    }

    fn split_off(&mut self, _event: tlib::events::MouseEvent) {
        if self.cnt != 0 {
            return;
        }
        self.cnt += 1;

        unsafe {
            self.label
                .as_mut()
                .unwrap()
                .as_mut()
                .set_text("Split Left.")
        };

        let mut label_2 = Label::new(Some("Split right Top"));
        label_2.set_size(20);
        label_2.set_background(Color::GREEN);
        label_2.set_content_halign(Align::Center);
        label_2.set_content_valign(Align::Center);
        self.right_top = label_2.id();

        let mut label_3 = Label::new(Some("Split right Bottom-Left"));
        label_3.set_size(20);
        label_3.set_background(Color::YELLOW);
        label_3.set_content_halign(Align::Center);
        label_3.set_content_valign(Align::Center);
        self.right_bottom_left = label_3.id();

        let mut label_4 = Label::new(Some("Split right Bottom-Right"));
        label_4.set_size(20);
        label_4.set_background(Color::CYAN);
        label_4.set_content_halign(Align::Center);
        label_4.set_content_valign(Align::Center);
        self.right_bottom_right = label_4.id();

        self.split_right(self.left, label_2);
        self.split_down(self.right_top, label_3);
        self.split_right(self.right_bottom_left, label_4);
    }
}
