use tlib::{connect, events::MouseEvent};
use tmui::{
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

use crate::popups::{rba_popup::RbaPopup, tba_popup::TbaPopup};

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct MyWidget {
    #[children]
    label_1: Tr<Label>,

    #[children]
    label_2: Tr<Label>,
}

impl ObjectSubclass for MyWidget {
    const NAME: &'static str = "MyWidget";
}

impl ObjectImpl for MyWidget {
    fn construct(&mut self) {
        self.parent_construct();
        self.set_strict_children_layout(true);
        self.set_homogeneous(false);

        self.label_1
            .set_text("Rect based animated popup. (click me)");
        self.label_2
            .set_text("Transparency based animated popup. (click me)");

        self.label_1.add_popup(RbaPopup::new().to_dyn_popup_tr());
        self.label_2.add_popup(TbaPopup::new().to_dyn_popup_tr());

        self.label_1.width_request(300);
        self.label_2.width_request(300);

        self.label_1.set_background(Color::CYAN);
        self.label_2.set_background(Color::GREEN);

        self.label_1.set_halign(Align::Start);
        self.label_2.set_halign(Align::End);

        // self.label_1.set_size_hint((Some((150, 0).into()), None));
        // self.label_2.set_size_hint((Some((150, 0).into()), None));

        connect!(
            self.label_1,
            mouse_pressed(),
            self,
            handle_label_1_popup(MouseEvent)
        );
        connect!(
            self.label_2,
            mouse_pressed(),
            self,
            handle_label_2_popup(MouseEvent)
        );
    }
}

impl WidgetImpl for MyWidget {}

impl MyWidget {
    fn handle_label_1_popup(&mut self, event: MouseEvent) {
        if self.label_1.get_popup_ref().unwrap().visible() {
            self.label_1.hide_popup();
        } else {
            let pos = self.label_1.map_to_global(&event.position().into());
            self.label_1.show_popup(pos);
        }
    }

    fn handle_label_2_popup(&mut self, event: MouseEvent) {
        if self.label_2.get_popup_ref().unwrap().visible() {
            self.label_2.hide_popup();
        } else {
            let pos = self.label_2.map_to_global(&event.position().into());
            self.label_2.show_popup(pos);
        }
    }
}
