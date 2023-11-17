use std::{
    env,
    sync::atomic::{AtomicBool, Ordering},
    thread::{self},
    time::Instant,
};
use tipc::mem::{mem_rw_lock::MemRwLock, BuildType};
use tlib::global::SemanticExt;

const OS_ID: &'static str = "mem_rw_lock44";

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
    println!("Master started.");

    let lock = MemRwLock::builder()
        .os_id(OS_ID)
        .build_type(BuildType::Create)
        .build()
        .unwrap()
        .arc();

    let exit = AtomicBool::new(false).arc();

    let exitc = exit.clone();
    let lockc = lock.clone();
    let join = thread::spawn(move || {
        let now = Instant::now();
        loop {
            let _guard = lockc.read();
            println!("read");

            if now.elapsed().as_secs() > 10 {
                exitc.store(true, Ordering::SeqCst);
                break;
            }
        }
    });

    loop {
        let _guard = lock.write();
        println!("write");

        if exit.load(Ordering::SeqCst) {
            break;
        }
    }

    join.join().unwrap()
}

fn slave() {
    println!("Slave started.");

    let lock = MemRwLock::builder()
        .os_id(OS_ID)
        .build_type(BuildType::Open)
        .build()
        .unwrap()
        .arc();

    let exit = AtomicBool::new(false).arc();

    let exitc = exit.clone();
    let lockc = lock.clone();
    let join = thread::spawn(move || {
        let now = Instant::now();
        loop {
            let _guard = lockc.read();
            println!("read");

            if now.elapsed().as_secs() > 5 {
                exitc.store(true, Ordering::SeqCst);
                break;
            }
        }
    });

    loop {
        let _guard = lock.write();
        println!("write");

        if exit.load(Ordering::SeqCst) {
            break;
        }
    }

    join.join().unwrap()
}
