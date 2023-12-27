use std::{
    env,
    time::{Duration, Instant},
};
use tipc::{ipc_event::IpcEvent, IpcBuilder, IpcNode};

const NAME: &'static str = "_ipc_test_02";
const TEXT_SIZE: usize = 1024;
const REQUEST_TEXT: &'static str = "Request from master.";
const RESPONSE_TEXT: &'static str = "Reponse from master.";

#[derive(Debug, Clone, Copy)]
enum UserEvent {
    Test(u64),
}

#[derive(Debug, Clone, Copy)]
enum Request {
    Request([u8; TEXT_SIZE]),
    Response([u8; TEXT_SIZE], bool),
}

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
    let master = IpcBuilder::<UserEvent, Request>::with_customize()
        .name(NAME)
        .master();

    let mut cnt = 0u64;
    let mut ins = None;

    loop {
        while master.has_event() {
            if ins.is_none() {
                ins = Some(Instant::now());
            }
            let evt = master.try_recv().unwrap();
            if let IpcEvent::UserEvent(UserEvent::Test(a), ..) = evt {
                assert_eq!(a, cnt);
            }
            cnt += 1;
        }

        if ins.is_some() {
            if ins.as_ref().unwrap().elapsed() >= Duration::from_secs(1) {
                println!("IpcMaster receive {} events/per second.", cnt);
                break;
            }
        }
        tlib::timer::sleep(Duration::from_micros(10));
    }

    let rec = Instant::now();
    let bytes = REQUEST_TEXT.as_bytes();
    let mut data = [0u8; TEXT_SIZE];
    data[0..bytes.len()].copy_from_slice(bytes);
    let resp = master
        .send_request(Request::Request(data))
        .unwrap()
        .unwrap();
    if let Request::Response(a, b) = resp {
        let str = String::from_utf8(a.to_vec())
            .unwrap()
            .trim_end_matches('\0')
            .to_string();
        assert_eq!(str, RESPONSE_TEXT);
        assert!(b);
    }
    println!(
        "Request time: {}ms",
        rec.elapsed().as_micros() as f64 / 1000.
    );
}

fn ipc_slave() {
    let slave = IpcBuilder::<UserEvent, Request>::with_customize()
        .name(NAME)
        .slave();

    let mut cnt = 0u64;
    let ins = Instant::now();

    loop {
        if ins.elapsed() >= Duration::from_secs(1) {
            println!("IpcSlave send {} events a second.", cnt);
            break;
        }
        if let Ok(_) = slave.try_send(IpcEvent::UserEvent(
            UserEvent::Test(cnt),
            Instant::now(),
        )) {
            cnt += 1;
        }
    }

    loop {
        if let Some(Request::Request(a)) = slave.try_recv_request() {
            let str = String::from_utf8(a.to_vec())
                .unwrap()
                .trim_end_matches('\0')
                .to_string();
            assert_eq!(str, REQUEST_TEXT);

            let bytes = RESPONSE_TEXT.as_bytes();
            let mut data = [0u8; TEXT_SIZE];
            data[0..bytes.len()].copy_from_slice(bytes);
            slave.respose_request(Some(Request::Response(data, true)));
            return
        }
    }
}
