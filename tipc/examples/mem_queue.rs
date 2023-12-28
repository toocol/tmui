use std::{
    env,
    time::{Duration, Instant},
};
use tipc::mem::{mem_queue::{MemQueue, MemQueueBuilder}, BuildType};
use tlib::utils::TimeStamp;

const SIZE: usize = 10000;
const TEXT_SIZE: usize = 4096;

const OS_ID: &'static str = "/mem_queue_04";
const TEXT: &'static str = "Text from slave.";

#[derive(Debug, Clone, Copy)]
enum Event {
    Foo(i32, [u8; TEXT_SIZE], u128),
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
    let queue: MemQueue<SIZE, Event> = MemQueueBuilder::new()
        .os_id(OS_ID)
        .build_type(BuildType::Create)
        .build()
        .unwrap();

    let mut ins = None;
    let mut cnt = 0;

    loop {
        while queue.has_event() {
            if ins.is_none() {
                ins = Some(Instant::now());
            }
            if let Some(evt) = queue.try_read() {
                match evt {
                    Event::Foo(a, b, _) => {
                        assert_eq!(a, cnt);
                        let text = String::from_utf8_lossy(&b)
                            .trim_end_matches('\0')
                            .to_string();
                        assert_eq!(text, TEXT);
                    }
                    Event::Bar(a, _) => {
                        assert!(a);
                    }
                }
                cnt += 1;
            }
        }
        if ins.is_some() {
            if ins.as_ref().unwrap().elapsed() >= Duration::from_secs(1) {
                println!("MemQueue receive {} events/per second.", cnt);
                return;
            }
        }
        std::thread::yield_now();
    }
}

fn slave() {
    let queue: MemQueue<SIZE, Event> = MemQueueBuilder::new()
        .os_id(OS_ID)
        .build_type(BuildType::Open)
        .build()
        .unwrap();

    let ins = Instant::now();
    let mut flag = false;
    let mut cnt = 0;

    loop {
        if ins.elapsed() >= Duration::from_secs(1) {
            println!("MemQueue send {} events a second.", cnt);
            return;
        }
        if flag {
            let bytes = TEXT.as_bytes();
            let mut data = [b'\0'; TEXT_SIZE];
            data[0..bytes.len()].copy_from_slice(bytes);
            if let Ok(_) = queue.try_write(Event::Foo(cnt, data, TimeStamp::timestamp_micros())) {
                cnt += 1;
            }
        } else {
            if let Ok(_) = queue.try_write(Event::Bar(true, TimeStamp::timestamp_micros())) {
                cnt += 1;
            }
        }

        flag = !flag;
    }
}
