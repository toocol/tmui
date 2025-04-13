use tlib::object::ObjectSubclass;
use tmui::prelude::*;

#[extends(Element)]
#[async_task(name = "AsyncElement", value = "Vec<i32>")]
pub struct AsyncTaskElement {}

impl ObjectSubclass for AsyncTaskElement {
    const NAME: &'static str = "AsyncTaskElement";
}

impl ObjectImpl for AsyncTaskElement {}

impl ElementImpl for AsyncTaskElement {
    fn on_renderer(&mut self, _cr: &DrawingContext) {}
}
