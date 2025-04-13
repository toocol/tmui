use tlib::object::ObjectSubclass;
use tmui::prelude::*;

#[extends(Object)]
#[async_task(name = "AsyncObject", value = "bool")]
pub struct AsyncTaskObject {}

impl ObjectSubclass for AsyncTaskObject {
    const NAME: &'static str = "AsyncTaskObject";
}

impl ObjectImpl for AsyncTaskObject {}
