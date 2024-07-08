use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::{callbacks::CallbacksRegister, WidgetImpl},
};

use crate::ctx_menu::CtxMenu;

#[extends(Widget, Layout(Stack))]
#[derive(Childrenable)]
#[popupable]
pub struct WorkspacePanel {}

impl ObjectSubclass for WorkspacePanel {
    const NAME: &'static str = "WorkspacePanel";
}

impl ObjectImpl for WorkspacePanel {
    fn construct(&mut self) {
        self.parent_construct();

        self.add_popup(CtxMenu::new());
    }

    fn initialize(&mut self) {
        self.set_hexpand(true);
        self.set_vexpand(true);

        self.set_background(Color::grey_with(108));
        self.register_mouse_released(|w, evt| {
            w.downcast_mut::<WorkspacePanel>()
                .unwrap()
                .show_popup(evt.position().into())
        });
    }
}

impl WidgetImpl for WorkspacePanel {}
