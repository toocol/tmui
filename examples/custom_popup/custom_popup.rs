use tlib::run_after;
use tmui::{
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Popup)]
#[async_task(name = "TestAsyncTask", value = "()")]
#[animatable(ty = "Linear", direction = "BottomToTop", duration = 500)]
#[derive(Childable)]
#[run_after]
pub struct CustomPopup {
    #[child]
    label: Box<Label>,
}

impl ObjectSubclass for CustomPopup {
    const NAME: &'static str = "CustomPopup";
}

impl ObjectImpl for CustomPopup {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_hexpand(true);
        self.set_vexpand(true);

        self.label.set_text("This is a popup.");

        let label_size = self.label.size();
        self.width_request(label_size.width());
        self.height_request(label_size.height());
    }
}

impl WidgetImpl for CustomPopup {
    fn run_after(&mut self) {
        self.parent_run_after();

        println!("CustomPopup run after.")
    }
}

impl PopupImpl for CustomPopup {}

impl CustomPopup {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
