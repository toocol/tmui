use tlib::{events::MouseEvent, connect};
use tmui::{
   prelude::*,
   tlib::object::{ObjectImpl, ObjectSubclass},
   widget::WidgetImpl, label::Label,
};

use crate::popups::{rba_popup::RbaPopup, tba_popup::TbaPopup};

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct MyWidget {
    #[children]
    label_1: Box<Label>,

    #[children]
    label_2: Box<Label>,
}

impl ObjectSubclass for MyWidget {
   const NAME: &'static str = "MyWidget";
}

impl ObjectImpl for MyWidget {
    fn construct(&mut self) {
        self.parent_construct();

        self.label_1.set_text("Rect based animated popup. (click me)");
        self.label_2.set_text("Transparency based animated popup. (click me)");

        self.label_1.add_popup(RbaPopup::new());
        self.label_2.add_popup(TbaPopup::new());

        self.label_1.width_request(300);
        self.label_2.width_request(300);

        self.label_1.set_background(Color::CYAN);
        self.label_2.set_background(Color::GREEN);

        self.label_1.set_halign(Align::Start);
        self.label_2.set_halign(Align::End);

        connect!(self.label_1, mouse_pressed(), self, handle_label_1_popup(MouseEvent));
        connect!(self.label_2, mouse_pressed(), self, handle_label_2_popup(MouseEvent));
    }
}

impl WidgetImpl for MyWidget {}

impl MyWidget {
    fn handle_label_1_popup(&mut self, event: MouseEvent ) {
        if self.label_1.get_popup_ref().unwrap().visible() {
            self.label_1.hide_popup();
        } else {
            self.label_1.show_popup(self.label_1.map_to_global(&event.position().into()));
        }
    }

    fn handle_label_2_popup(&mut self, event: MouseEvent ) {
        if self.label_2.get_popup_ref().unwrap().visible() {
            self.label_2.hide_popup();
        } else {
            self.label_2.show_popup(self.label_2.map_to_global(&event.position().into()));
        }
    }
}