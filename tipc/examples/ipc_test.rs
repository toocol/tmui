use std::{env, thread, time::Duration};
use tipc::{ipc_event::IpcEvent, ipc_master::IpcMaster, ipc_slave::IpcSlave};
use tlib::utils::TimeStamp;

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
    let master = IpcMaster::new("_ipc_test", 100, 100);

    loop {
        match master.try_recv() {
            IpcEvent::RequestFocusEvent(content, timestamp) => {
                let time = TimeStamp::timestamp() - (timestamp as u64);
                println!(
                    "<2> Master receive native event, {}, time consumption: {}",
                    content, time
                );

                let start = TimeStamp::timestamp();
                if let Ok(resp) = master.send_shared_message(IpcEvent::SharedMessage(
                    "Shared message from master.".to_string(),
                    1,
                )) {
                    let end = TimeStamp::timestamp();
                    println!(
                        "<4> Master receive shared response: {}, time consumption: {}",
                        resp,
                        end - start
                    );
                }
            }
            _ => {}
        }
        thread::sleep(Duration::from_nanos(1));
    }
}

fn ipc_slave() {
    let slave = IpcSlave::new("_ipc_test");
    let id = slave.id();
    ctrlc::set_handler(move || {
        print!("Prepare terminate shared mem, id = {}", id);
        IpcSlave::terminate_at(id);
    }).unwrap();

    let mut last = TimeStamp::timestamp();

    loop {
        let now = TimeStamp::timestamp();
        if now - last >= 1000 {
            last = now;
            slave.send(IpcEvent::RequestFocusEvent(
                true,
                TimeStamp::timestamp() as i64,
            ));
            println!("<1> Slave send the native event");
        }

        if let Some(shared_msg) = slave.try_recv_shared_message() {
            println!("<3> Slave receive the shared msg: {}", shared_msg);
            IpcSlave::respose_shared_msg(slave.id(), Some("Slave response the msg"));
        }

        thread::sleep(Duration::from_nanos(1));
    }
}
