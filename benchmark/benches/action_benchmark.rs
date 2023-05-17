use std::sync::atomic::Ordering;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tlib::{
    actions::{ActionHub, ACTIVATE},
    connect, disconnect, emit,
    object::{ObjectImpl, ObjectSubclass},
    prelude::*,
    signal, signals,
};

#[extends(Object)]
#[derive(Default)]
struct Widget {}

impl Widget {
    pub fn slot_test_string(&self, str: String) {
        assert_eq!(str, "action benchmark string param")
    }
}

impl ObjectSubclass for Widget {
    const NAME: &'static str = "Widget";
}

impl ObjectImpl for Widget {}

impl Widget {
    signals! {
        /// Sginal to action benchmark tuple test.
        action_benchmark_tuple();
        /// Signal to action benchmark string test.
        action_benchmark_string();
    }
}

fn test_action_tuple(widget: &Widget) {
    emit!(
        widget.action_benchmark_tuple(),
        "Bench mark param 1",
        "Bench mark param 2",
        i32::MAX,
        i32::MIN,
        f64::MAX,
        "Bench mark param 6",
        f64::MIN
    )
}

fn test_action_string(widget: &Widget) {
    emit!(
        widget.action_benchmark_string(),
        "action benchmark string param"
    )
}

fn test_emit_action(widget: &[Widget], idx: usize) {
    emit!(
        widget[idx].action_benchmark_string(),
        "action benchmark string param"
    )
}

fn test_disconnect(widget: &[Widget], idx: usize) {
    disconnect!(null, null, widget[idx], null);
}

fn criterion_values(c: &mut Criterion) {
    let mut action_hub = ActionHub::new();
    action_hub.initialize();
    ACTIVATE.store(true, Ordering::SeqCst);

    let widget: Widget = Object::new(&[]);
    widget.connect(
        widget.action_benchmark_tuple(),
        widget.id(),
        Box::new(|param| {
            let (p1, p2, p3, p4, p5, p6, p7) =
                param
                    .as_ref()
                    .unwrap()
                    .get::<(String, String, i32, i32, f64, String, f64)>();
            assert_eq!(p1, "Bench mark param 1");
            assert_eq!(p2, "Bench mark param 2");
            assert_eq!(p3, i32::MAX);
            assert_eq!(p4, i32::MIN);
            assert_eq!(p5, f64::MAX);
            assert_eq!(p6, "Bench mark param 6");
            assert_eq!(p7, f64::MIN);
        }),
    );
    widget.connect(
        widget.action_benchmark_string(),
        widget.id(),
        Box::new(|param| {
            let param = param.as_ref().unwrap().get::<String>();
            assert_eq!(param, "action benchmark string param");
        }),
    );

    let mut widgets = vec![];
    for _ in 0..10000usize {
        let mut widget: Widget = Object::new(&[]);
        for _ in 0..100usize {
            connect!(
                widget,
                action_benchmark_string(),
                widget,
                slot_test_string(String)
            );
        }
        widgets.push(widget);
    }

    c.bench_function("actions-tuple-test", |b| {
        b.iter(|| test_action_tuple(&widget))
    });
    c.bench_function("actions-string-test", |b| {
        b.iter(|| test_action_string(&widget))
    });
    c.bench_function("actions-emit-action", |b| {
        b.iter(|| test_emit_action(&widgets, black_box(9999)))
    });
    c.bench_function("actions-disconnect-test", |b| {
        b.iter(|| test_disconnect(&widgets, black_box(9999)))
    });
}

criterion_group!(benches, criterion_values);
criterion_main!(benches);
