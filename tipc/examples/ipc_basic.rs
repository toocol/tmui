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
        while master.has_event() {
            if ins.is_none() {
                ins = Some(Instant::now());
            }
            match master.try_recv() {
                IpcEvent::RequestFocusEvent(_, _) => {
                    cnt += 1;
                }
                _ => {}
            }
        }
        if ins.is_some() {
            if ins.as_ref().unwrap().elapsed() >= Duration::from_secs(1) {
                println!("IpcMaster receive {} events/per second.", cnt);
                return;
            }
        }
        tlib::timer::sleep(Duration::from_micros(10));
    }
}

fn ipc_slave() {
    let slave = IpcSlave::new(NAME);
    let id = slave.id();
    ctrlc::set_handler(move || {
        print!("Prepare terminate shared mem, id = {}", id);
        IpcSlave::terminate_at(id);
    })
    .unwrap();

    let mut cnt = 0u64;
    let ins = Instant::now();

    loop {
        if ins.elapsed() >= Duration::from_secs(1) {
            println!("IpcSlave send {} events a second.", cnt);
            return;
        }
        cnt += 1;
        slave.send(IpcEvent::RequestFocusEvent(
            true,
            TimeStamp::timestamp() as i64,
        ));
    }
}
