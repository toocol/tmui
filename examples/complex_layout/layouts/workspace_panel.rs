use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget, Layout(Stack))]
#[derive(Childrenable)]
pub struct WorkspacePanel {
    #[children]
    widget: Box<Widget>
}

impl ObjectSubclass for WorkspacePanel {
    const NAME: &'static str = "WorkspacePanel";
}

impl ObjectImpl for WorkspacePanel {
    fn initialize(&mut self) {
        self.set_hexpand(true);
        self.set_vexpand(true);

        self.widget.set_hexpand(true);
        self.widget.set_vexpand(true);
        self.widget.set_background(Color::grey_with(108));
    }
}

impl WidgetImpl for WorkspacePanel {}
