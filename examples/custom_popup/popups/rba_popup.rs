use std::time::Duration;

use tlib::run_after;
use tmui::{
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

/// Rect based animation popup.
#[extends(Popup)]
#[async_task(name = "TestAsyncTask", value = "&'static str")]
#[animatable(
    ty = "EaseOut",
    direction = "BottomToTop",
    duration = 1000,
    mode = "Flex",
    effect = "Slide"
)]
#[derive(Childable)]
#[run_after]
pub struct RbaPopup {
    #[child]
    label: Box<Label>,
}

impl ObjectSubclass for RbaPopup {
    const NAME: &'static str = "RbaPopup";
}

impl ObjectImpl for RbaPopup {
    fn construct(&mut self) {
        self.parent_construct();

        self.width_request(100);
        self.height_request(40);
        self.label.set_halign(Align::Center);
        self.label.set_valign(Align::Center);

        self.test_async_task(
            async {
                tokio::time::sleep(Duration::from_secs(1)).await;
                "This is a popup."
            },
            |p: &mut RbaPopup, text| {
                p.label.set_text(text);
            },
        );
    }
}

impl WidgetImpl for RbaPopup {
    fn run_after(&mut self) {
        println!("CustomPopup run after.")
    }
}

impl PopupImpl for RbaPopup {}

impl RbaPopup {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
