use std::{collections::HashMap, thread::{ThreadId, self}};

use crate::Value;
use once_cell::sync::Lazy;
use tokio::{
    runtime::{Builder, Runtime},
    task::JoinHandle,
};

#[inline]
pub fn tokio_runtime() -> &'static Runtime {
    static TOKIO_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
        Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap()
    });
    &TOKIO_RUNTIME
}

#[inline]
pub fn async_tasks<'a>() -> &'static mut HashMap<ThreadId, Vec<AsyncTask<'a>>> {
    static mut ASYNC_TASK: Lazy<HashMap<ThreadId, Vec<AsyncTask>>> =
        Lazy::new(HashMap::new);
    unsafe { &mut ASYNC_TASK }
}

#[inline]
pub fn async_callbacks() {
    let thread_id = thread::current().id();

    let tasks_ref = async_tasks().get(&thread_id);
    if tasks_ref.is_some() && !tasks_ref.unwrap().is_empty() {
        let mut task_queue = async_tasks().remove(&thread_id).unwrap();

        let iter = task_queue.into_iter();
        task_queue = vec![];

        for task in iter {
            if let Some(t) = task.block_on_finished() {
                task_queue.push(t)
            }
        }

        async_tasks().insert(thread_id, task_queue);
    }
}

pub struct AsyncTask<'a> {
    join_handler: JoinHandle<Value>,
    then: Option<Box<dyn FnOnce(Value) + 'a>>,
}

impl<'a> AsyncTask<'a> {
    pub fn new(join_handler: JoinHandle<Value>) -> Self {
        Self {
            join_handler,
            then: None,
        }
    }

    pub fn block_on_finished(self) -> Option<Self> {
        if self.join_handler.is_finished() {
            let result = tokio_runtime().block_on(self.join_handler).unwrap();
            if let Some(then) = self.then {
                then(result);
            }
            None
        } else {
            Some(self)
        }
    }

    pub fn then<F>(mut self, then: F) -> Self
    where
        F: FnOnce(Value) + 'a,
    {
        self.then = Some(Box::new(then));
        self
    }
}
