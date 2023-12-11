use std::{thread, time::Duration};

use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
#[async_task(name = "AsyncTask", value = "i32")]
#[async_task(name = "AsyncTask2", value = "f32")]
pub struct AsyncTaskWidget {}

impl ObjectSubclass for AsyncTaskWidget {
    const NAME: &'static str = "AsyncTaskWidget";
}

impl ObjectImpl for AsyncTaskWidget {
    fn construct(&mut self) {
        self.parent_construct();

        self.async_task(async { 
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("{} => async do", thread::current().name().unwrap());
            12
         }, Some(|_, val| {
            println!("{} => then", thread::current().name().unwrap());
            assert_eq!(val, 12)
        }));

        self.async_task2(async {
            println!("{} => async do 2", thread::current().name().unwrap());
            3.1
        }, Some(|_, val| {
            println!("{} => then 2", thread::current().name().unwrap());
            assert_eq!(val, 3.1)
        }));
    }
}

impl WidgetImpl for AsyncTaskWidget {}

impl AsyncTaskWidget {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
