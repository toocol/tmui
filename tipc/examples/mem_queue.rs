use std::{
    env,
    time::{Duration, Instant},
};
use tipc::mem::mem_queue::{BuildType, MemQueue, MemQueueBuilder};
use tlib::utils::TimeStamp;

const SIZE: usize = 10000;
const OS_ID: &'static str = "/mem_queue_04";

#[derive(Debug, Clone, Copy)]
enum TestEvent {
    Foo(i32, i32, u128),
    Bar(bool, u128),
}

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let ty = args.next().unwrap();

    match ty.as_str() {
        "master" => master(),
        "slave" => slave(),
        _ => {}
    }
}

fn master() {
    let queue: MemQueue<SIZE, TestEvent> = MemQueueBuilder::new()
        .os_id(OS_ID)
        .build_type(BuildType::Create)
        .build();

    let mut ins = None;
    let mut cnt = 0;

    loop {
        while queue.has_event() {
            if ins.is_none() {
                ins = Some(Instant::now());
            }
            if let Some(evt) = queue.try_read() {
                cnt += 1;
                match evt {
                    TestEvent::Foo(a, b, _) => {
                        assert_eq!(a, 1024);
                        assert_eq!(b, 48);
                    }
                    TestEvent::Bar(a, _) => {
                        assert!(a);
                    }
                }
            }
        }
        if ins.is_some() {
            if ins.as_ref().unwrap().elapsed() >= Duration::from_secs(1) {
                println!("MemQueue receive {} events/per second.", cnt);
                return;
            }
        }
        tlib::timer::sleep(Duration::from_micros(10));
    }
}

fn slave() {
    let mut queue: MemQueue<SIZE, TestEvent> = MemQueueBuilder::new()
        .os_id(OS_ID)
        .build_type(BuildType::Open)
        .build();

    let ins = Instant::now();
    let mut flag = false;
    let mut cnt = 0;

    loop {
        if ins.elapsed() >= Duration::from_secs(1) {
            println!("MemQueue send {} events a second.", cnt);
            return;
        }
        if flag {
            if let Ok(_) = queue.try_write(TestEvent::Foo(1024, 48, TimeStamp::timestamp_micros()))
            {
                cnt += 1;
            }
        } else {
            if let Ok(_) = queue.try_write(TestEvent::Bar(true, TimeStamp::timestamp_micros())) {
                cnt += 1;
            }
        }

        flag = !flag;
    }
}
