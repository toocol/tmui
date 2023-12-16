use tlib::run_after;
use tmui::{
   prelude::*,
   tlib::object::{ObjectImpl, ObjectSubclass},
   widget::WidgetImpl,
};

#[extends(Popup)]
#[async_task(name = "TestAsyncTask", value = "()")]
#[animatable(ty = "Linear", direction = "BottomToTop", duration = 500)]
#[run_after]
pub struct CustomPopup {}

impl ObjectSubclass for CustomPopup {
   const NAME: &'static str = "CustomPopup";
}

impl ObjectImpl for CustomPopup {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_hexpand(true);
        self.set_vexpand(true);
    }
}

impl WidgetImpl for CustomPopup {
    fn run_after(&mut self) {
        self.parent_run_after();

        println!("CustomPopup run after.")
    }
}

impl PopupImpl for CustomPopup {}