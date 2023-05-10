use std::{
    mem::size_of,
    sync::atomic::{AtomicUsize, Ordering},
    time::{Duration, Instant},
};

use raw_sync::{events::*, Timeout};
use shared_memory::*;
use tlib::{timer::{self}, utils::TimeStamp};

const OS_ID: &'static str = "__mem_mapping_os_id_1";
const STRUCT_ID: &'static str = "__mem_struct_os_id_10";

enum Msg {
    Event(i32, i32, u128),
    _Msg(u64, i64, bool),
}

pub struct Struct<const SIZE: usize> {
    read: AtomicUsize,
    write: AtomicUsize,
    msgs: [Msg; SIZE],
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const SIZE: usize = 11;
    // test_event()?;
    test_queue::<SIZE>()?;
    Ok(())
}

fn test_event() -> Result<(), Box<dyn std::error::Error>> {
    // Attempt to create a mapping or open if it already exists
    println!("Getting the shared memory mapping");
    let shmem = match ShmemConf::new().size(4096).os_id(OS_ID).create() {
        Ok(m) => m,
        Err(ShmemError::MappingIdExists) => ShmemConf::new().os_id(OS_ID).open()?,
        Err(ShmemError::LinkExists) => ShmemConf::new().os_id(OS_ID).open()?,
        Err(e) => return Err(Box::new(e)),
    };

    if shmem.is_owner() {
        //Create an event in the shared memory
        println!("Creating event in shared memory");
        let (evt, used_bytes) = unsafe { Event::new(shmem.as_ptr(), true)? };
        println!("\tUsed {used_bytes} bytes");

        println!("Launch another instance of this example to signal the event !");
        evt.wait(Timeout::Infinite)?;
        println!("\tGot signal !");
    } else {
        // Open existing event
        println!("Openning event from shared memory");
        let (evt, used_bytes) = unsafe { Event::from_existing(shmem.as_ptr())? };
        println!("\tEvent uses {used_bytes} bytes");

        println!("Signaling event !");
        evt.set(EventState::Signaled)?;
        println!("\tSignaled !");
    }

    println!("Done !");
    Ok(())
}

fn test_queue<const SIZE: usize>() -> Result<(), Box<dyn std::error::Error>> {
    println!("Test queue !");
    let shm = match ShmemConf::new()
        .size(size_of::<Struct<SIZE>>())
        .os_id(STRUCT_ID)
        .create()
    {
        Ok(m) => m,
        Err(ShmemError::MappingIdExists) => ShmemConf::new().os_id(STRUCT_ID).open()?,
        Err(ShmemError::LinkExists) => ShmemConf::new().os_id(STRUCT_ID).open()?,
        Err(e) => return Err(Box::new(e)),
    };

    let ptr = shm.as_ptr() as *mut Struct<SIZE>;

    if shm.is_owner() {
        println!("Is owner !");
        let mut flag = false;
        let mut instant = Instant::now();
        loop {
            println!("{}ms", instant.elapsed().as_micros() as f64 / 1000.);
            instant = Instant::now();
            let sr = unsafe { ptr.as_mut().unwrap() };
            let mut cnt = 0;
            while sr.read.load(Ordering::SeqCst) != sr.write.load(Ordering::SeqCst) {
                let msg = &sr.msgs[sr.read.fetch_add(1, Ordering::SeqCst)];
                if let Msg::Event(a, b, t) = msg {
                    assert_eq!(*a, 12);
                    assert_eq!(*b, 24);
                    println!(
                        "<{}> Time consumption: {}ms",
                        cnt,
                        (TimeStamp::timestamp_micros() - *t) as f32 / 1000.
                    );
                }
                cnt += 1;
                flag = true;
            }
            if flag {
                break;
            }
            timer::sleep(Duration::from_millis(16));
        }
    } else {
        for i in 0..10 {
            let sr = unsafe { ptr.as_mut().unwrap() };
            sr.msgs[sr.write.fetch_add(1, Ordering::SeqCst)] =
                Msg::Event(12, 24, TimeStamp::timestamp_micros());
            println!("<{}> Send msg !", i);
        }
    }

    Ok(())
}
