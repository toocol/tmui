use std::{env, thread, time::{Duration, Instant}};
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

    loop {
        match master.try_recv() {
            IpcEvent::RequestFocusEvent(content, timestamp) => {
                let time = TimeStamp::timestamp() - (timestamp as u64);
                cnt += 1;
                println!(
                    "<2> Master receive native event, {}, time consumption: {}ms, cnt: {}",
                    content, time, cnt
                );

                let instant = Instant::now();
                if let Ok(resp) = master.send_shared_message(IpcEvent::SharedMessage(
                    "Shared message from master.".to_string(),
                    1,
                )) {
                    let duration = instant.elapsed();
                    println!(
                        "<4> Master receive shared response: {}, time consumption: {}ms",
                        resp,
                        duration.as_micros() as f32 / 1000.
                    );
                }
            }
            _ => {}
        }
        thread::sleep(Duration::from_nanos(1));
    }
}

fn ipc_slave() {
    let slave = IpcSlave::new(NAME);
    let id = slave.id();
    ctrlc::set_handler(move || {
        print!("Prepare terminate shared mem, id = {}", id);
        IpcSlave::terminate_at(id);
    }).unwrap();

    let mut last = TimeStamp::timestamp();
    let mut cnt = 0u64;

    loop {
        let now = TimeStamp::timestamp();
        if now - last >= 1000 {
            last = now;
            cnt += 1;
            slave.send(IpcEvent::RequestFocusEvent(
                // "Native event from slave!".to_string(),
                true,
                TimeStamp::timestamp() as i64,
            ));
            println!("<1> Slave send the native event, cnt: {}", cnt);
        }

        if let Some(shared_msg) = slave.try_recv_shared_message() {
            println!("<3> Slave receive the shared msg: {}", shared_msg);
            IpcSlave::respose_shared_msg(slave.id(), Some("Slave response the msg"));
        }

        thread::sleep(Duration::from_nanos(1));
    }
}
