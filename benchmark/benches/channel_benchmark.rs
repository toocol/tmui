use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::mpsc;
use std::thread;

fn benchmark_channels(c: &mut Criterion) {
    c.bench_function("channel_single_2000", |b| {
        b.iter(|| {
            let (tx, rx) = mpsc::channel();

            let sender_thread = thread::spawn(move || {
                for i in 0..2000 {
                    tx.send(black_box(format!("Message {}", i))).unwrap();
                }
            });

            let receiver_thread = thread::spawn(move || {
                let mut cnt = 0;
                loop {
                    while let Ok(_) = rx.try_recv() {
                        cnt += 1;
                    }

                    if cnt == 2000 {
                        break;
                    }
                }
            });

            sender_thread.join().unwrap();
            receiver_thread.join().unwrap();
        });
    });

    c.bench_function("channel_vec_2000", |b| {
        b.iter(|| {
            let (tx, rx) = mpsc::channel();

            let sender_thread = thread::spawn(move || {
                let messages: Vec<String> = (0..2000).map(|i| format!("Message {}", i)).collect();
                tx.send(black_box(messages)).unwrap();
            });

            let receiver_thread = thread::spawn(move || {
                let _ = rx.recv().unwrap();
            });

            sender_thread.join().unwrap();
            receiver_thread.join().unwrap();
        });
    });
}

criterion_group!(benches, benchmark_channels);
criterion_main!(benches);
