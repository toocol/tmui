use tmui::{
    button::Button,
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

use crate::{
    child_widget::ChildWidget, split_pane_layout::SplitPaneLayout, stack_widget::StackWidget,
};

#[extends(Widget, Layout(VBox))]
#[derive(Childrenable)]
pub struct RemoveWidget {
    #[children]
    top: Tr<HBox>,
    #[children]
    bottom: Tr<HBox>,
    #[children]
    stack: Tr<StackWidget>,
    #[children]
    split_pane: Tr<SplitPaneLayout>,

    to_remove: ObjectId,
    widget_id: ObjectId,
}

impl ObjectSubclass for RemoveWidget {
    const NAME: &'static str = "RemoveWidget";
}

impl ObjectImpl for RemoveWidget {
    fn construct(&mut self) {
        self.parent_construct();

        self.bottom.set_margin_top(5);
        self.bottom.set_background(Color::GREY_LIGHT);

        self.set_hexpand(true);
        self.set_vexpand(true);
        let mut button_0 = Button::new(Some("Remove 1th"));
        let mut button_1 = Button::new(Some("Remove Left"));
        let mut button_2 = Button::new(Some("Remove Right"));
        let mut button_3 = Button::new(Some("Remove Stack"));
        let mut button_4 = Button::new(Some("Remove SplitPane Left"));
        let mut button_5 = Button::new(Some("Remove SplitPane RightTop"));
        let mut button_6 = Button::new(Some("Remove SplitPane RightBottomLeft"));
        let mut button_7 = Button::new(Some("Remove SplitPane RightBottomRight"));
        button_0.width_request(100);
        button_1.width_request(100);
        button_1.width_request(100);
        button_2.width_request(100);
        button_3.width_request(100);
        button_4.width_request(200);
        button_5.width_request(200);
        button_6.width_request(200);
        button_7.width_request(200);
        connect!(
            button_0,
            mouse_pressed(),
            self,
            remove_0th_pressed(MouseEvent)
        );
        connect!(
            button_1,
            mouse_pressed(),
            self,
            remove_left_pressed(MouseEvent)
        );
        connect!(
            button_2,
            mouse_pressed(),
            self,
            remove_right_pressed(MouseEvent)
        );
        connect!(
            button_3,
            mouse_pressed(),
            self,
            remove_stack_widget(MouseEvent)
        );
        connect!(
            button_4,
            mouse_pressed(),
            self,
            remove_split_pane_left(MouseEvent)
        );
        connect!(
            button_5,
            mouse_pressed(),
            self,
            remove_split_pane_right_top(MouseEvent)
        );
        connect!(
            button_6,
            mouse_pressed(),
            self,
            remove_split_pane_right_bottom_left(MouseEvent)
        );
        connect!(
            button_7,
            mouse_pressed(),
            self,
            remove_split_pane_right_bottom_right(MouseEvent)
        );
        self.top.add_child(button_0);
        self.top.add_child(button_1);
        self.top.add_child(button_2);
        self.top.add_child(button_3);
        self.top.add_child(button_4);
        self.top.add_child(button_5);
        self.top.add_child(button_6);
        self.top.add_child(button_7);

        self.bottom.add_child(Label::new(Some("Label 1")));
        let label2 = Label::new(Some("Label 2"));
        self.to_remove = label2.id();
        self.bottom.add_child(label2);
        self.bottom.add_child(Label::new(Some("Label 3")));
        self.bottom.add_child(Label::new(Some("Label 4")));

        let child_widget = ChildWidget::new();
        self.widget_id = child_widget.id();
        self.bottom.add_child(child_widget);
    }
}

impl WidgetImpl for RemoveWidget {}

impl RemoveWidget {
    #[inline]
    pub fn new() -> Tr<Self> {
        Self::new_alloc()
    }

    pub fn remove_0th_pressed(&mut self, _: MouseEvent) {
        let id = self.bottom.children().first().map(|c| c.id());
        if let Some(id) = id {
            self.bottom.remove_children(id);
        }
    }

    pub fn remove_left_pressed(&mut self, _: MouseEvent) {
        self.bottom.remove_children(self.to_remove);
    }

    pub fn remove_right_pressed(&mut self, _: MouseEvent) {
        self.window()
            .find_id_mut(self.widget_id)
            .unwrap()
            .downcast_mut::<ChildWidget>()
            .unwrap()
            .remove_child();
    }

    pub fn remove_stack_widget(&mut self, _: MouseEvent) {
        self.stack.remove_index(1);
    }

    pub fn remove_split_pane_left(&mut self, _: MouseEvent) {
        self.split_pane.remove_left();
    }

    pub fn remove_split_pane_right_top(&mut self, _: MouseEvent) {
        self.split_pane.remove_right_top();
    }

    pub fn remove_split_pane_right_bottom_left(&mut self, _: MouseEvent) {
        self.split_pane.remove_right_bottom_left();
    }

    pub fn remove_split_pane_right_bottom_right(&mut self, _: MouseEvent) {
        self.split_pane.remove_right_bottom_right();
    }
}
