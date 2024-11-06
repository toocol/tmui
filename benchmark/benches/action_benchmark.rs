use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tlib::{
    actions::ActionHub,
    connect, disconnect, emit,
    object::{ObjectImpl, ObjectSubclass},
    prelude::*,
    signal, signals,
};

#[extends(Object)]
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
        Widget:

        /// Sginal to action benchmark tuple test.
        action_benchmark_tuple(&str, &str, i32, i32, f64, &str, f64);
        /// Signal to action benchmark string test.
        action_benchmark_string(&str);
    }
}

fn test_action_tuple(widget: &Widget) {
    emit!(
        widget,
        action_benchmark_tuple(
            "Bench mark param 1",
            "Bench mark param 2",
            i32::MAX,
            i32::MIN,
            f64::MAX,
            "Bench mark param 6",
            f64::MIN
        )
    )
}

fn test_action_string(widget: &Widget) {
    emit!(
        widget,
        action_benchmark_string("action benchmark string param")
    )
}

fn test_emit_action(widget: &[Box<Widget>], idx: usize) {
    emit!(
        widget[idx],
        action_benchmark_string("action benchmark string param")
    )
}

fn test_disconnect(widget: &[Box<Widget>], idx: usize) {
    disconnect!(null, null, widget[idx], null);
}

fn criterion_values(c: &mut Criterion) {
    ActionHub::initialize();

    let widget: Box<Widget> = Object::new(&[]);
    widget.connect(
        widget.action_benchmark_tuple(),
        widget.id(),
        Box::new(|_param| {}),
    );
    widget.connect(
        widget.action_benchmark_string(),
        widget.id(),
        Box::new(|_param| {}),
    );

    let mut widgets = vec![];
    for _ in 0..10000usize {
        let mut widget: Box<Widget> = Object::new(&[]);
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
