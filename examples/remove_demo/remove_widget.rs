use tmui::{
    button::Button,
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

use crate::{child_widget::ChildWidget, stack_widget::StackWidget};

#[extends(Widget, Layout(VBox))]
#[derive(Childrenable)]
pub struct RemoveWidget {
    #[children]
    top: Box<HBox>,
    #[children]
    bottom: Box<HBox>,
    #[children]
    stack: Box<StackWidget>,

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

        self.set_hexpand(true);
        self.set_vexpand(true);
        let mut button_1 = Button::new(Some("Remove Left"));
        let mut button_2 = Button::new(Some("Remove Right"));
        let mut button_3 = Button::new(Some("Remove Stack"));
        button_1.width_request(100);
        button_2.width_request(100);
        button_3.width_request(100);
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
        self.top.add_child(button_1);
        self.top.add_child(button_2);
        self.top.add_child(button_3);

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
    pub fn new() -> Box<Self> {
        Object::new(&[])
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
}
