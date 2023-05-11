use std::{
    env,
    time::{Duration, Instant},
};
use tipc::{ipc_event::IpcEvent, ipc_master::IpcMaster, ipc_slave::IpcSlave};
use tlib::utils::TimeStamp;

const NAME: &'static str = "_ipc_test";

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let ty = args.next().unwrap();
    match ty.as_str() {
        "master" => ipc_master(),
        "slave" => ipc_slave(),
        _ => println!("Invalid startup type: {}", ty),
    }
}

fn ipc_master() {
    let master = IpcMaster::new(NAME, 100, 100);

    let mut cnt = 0u64;
    let mut ins = None;

    loop {
        if ins.is_none() {
            ins = Some(Instant::now());
        }
        tlib::timer::sleep(Duration::from_micros(10));
    }
}

fn ipc_slave() {
    let slave = IpcSlave::new(NAME);

    let mut cnt = 0u64;
    let ins = Instant::now();

    loop {
        if ins.elapsed() >= Duration::from_secs(1) {
            println!("IpcSlave send {} events a second.", cnt);
            return;
        }
        cnt += 1;
    }
}
