use std::{cell::RefCell, rc::Rc, thread, time::Duration};
use tmui::prelude::*;
use tokio::time::sleep;

#[test]
fn main() {
    let runtime = tokio_runtime();
    let _guard = runtime.enter();

    let rec = Rc::new(RefCell::new(0));
    let rec_clone = rec.clone();

    async_do!(move {
        println!("[{:?}] Start executing async block 1", thread::current().id());
        sleep(Duration::from_secs(2)).await;
        println!("[{:?}] Async block 1 awake", thread::current().id());
        64
    } => |num| {
        println!("[{:?}] After the async block 1 runs, return to the main thread to execute the then() function", thread::current().id());
        let num = num.get::<i32>();
        *rec_clone.borrow_mut() = num;
    });


    async_do!({
        println!("[{:?}] Start executing async block 2", thread::current().id());
        sleep(Duration::from_secs(1)).await;
        println!("[{:?}] Async block 2 awake", thread::current().id())
    } => {
        println!("[{:?}] After the async block 2 runs, return to the main thread to execute the then() function", thread::current().id());
    });

    let thread_id = thread::current().id();
    let mut flag = 0;

    loop {
        if flag == 2 {
            assert_eq!(*rec.borrow(), 64);
            break;
        }

        if async_tasks().get(&thread_id).unwrap().len() > 0 {
            let mut task_queue = async_tasks().remove(&thread_id).unwrap();

            let iter = task_queue.into_iter();
            task_queue = vec![];

            for task in iter {
                if let Some(t) = task.block_on_finished() {
                    task_queue.push(t)
                } else {
                    flag += 1;
                }
            }

            async_tasks().insert(thread_id, task_queue);
        }
    }
}
