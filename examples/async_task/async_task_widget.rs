use std::{thread, time::Duration};

use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

use crate::{async_task_element::AsyncTaskElement, async_task_object::AsyncTaskObject};

#[extends(Widget)]
#[async_task(name = "AsyncTask", value = "i32")]
#[async_task(name = "AsyncTask2", value = "f32")]
#[async_task(name = "AsyncTask3", value = "()")]
pub struct AsyncTaskWidget {
    async_object: Box<AsyncTaskObject>,
    async_element: Box<AsyncTaskElement>,
}

impl ObjectSubclass for AsyncTaskWidget {
    const NAME: &'static str = "AsyncTaskWidget";
}

impl ObjectImpl for AsyncTaskWidget {
    fn initialize(&mut self) {
        self.async_task(
            async {
                tokio::time::sleep(Duration::from_secs(1)).await;
                println!("{} => async do", thread::current().name().unwrap());
                12
            },
            |_, val| {
                println!(
                    "{} => then, should be last one",
                    thread::current().name().unwrap()
                );
                assert_eq!(val, 12)
            },
        );

        self.async_task2(
            async {
                println!("{} => async do 2", thread::current().name().unwrap());
                3.1
            },
            |_, val| {
                println!("{} => then 2", thread::current().name().unwrap());
                assert_eq!(val, 3.1)
            },
        );

        self.async_task3(async {}, |_, _val| {});

        self.async_object.async_object(
            async {
                println!("{} => async obejct", thread::current().name().unwrap());
                true
            },
            |_, val| {
                println!("{} => then obejct", thread::current().name().unwrap());
                assert!(val)
            },
        );

        self.async_element.async_element(
            async {
                println!("{} => async element", thread::current().name().unwrap());
                vec![1, 2, 3]
            },
            |_, val| {
                println!("{} => then element", thread::current().name().unwrap());
                assert_eq!(val, vec![1, 2, 3])
            },
        );

        async_do!(
            {
                println!("{} => async_do!", thread::current().name().unwrap());
                ()
            } =>
            || {
                println!("{} => then async_do!", thread::current().name().unwrap());
            }
        )
    }
}

impl WidgetImpl for AsyncTaskWidget {}

impl AsyncTaskWidget {
    #[inline]
    pub fn new() -> Tr<Self> {
        Self::new_alloc()
    }
}
