use criterion::{criterion_group, criterion_main, Criterion};
use tlib::{emit, prelude::*, actions::{ActionHub, initialize_action_hub}};

struct Widget;
impl ActionHubExt for Widget {}

pub fn test_action_tuple() {
    emit!(
        "action_benchmark_tuple",
        (
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

pub fn test_action_string() {
    emit!("action_benchmark_string", "action benchmark string param")
}

pub fn criterion_values(c: &mut Criterion) {
    let mut action_hub = ActionHub::new();
    initialize_action_hub(&mut action_hub);

    let widget = Widget {};
    widget.connect_action("action_benchmark_tuple", |param| {
        let (p1, p2, p3, p4, p5, p6, p7) =
            param
                .unwrap()
                .get::<(String, String, i32, i32, f64, String, f64)>();
        assert_eq!(p1, "Bench mark param 1");
        assert_eq!(p2, "Bench mark param 2");
        assert_eq!(p3, i32::MAX);
        assert_eq!(p4, i32::MIN);
        assert_eq!(p5, f64::MAX);
        assert_eq!(p6, "Bench mark param 6");
        assert_eq!(p7, f64::MIN);
    });
    widget.connect_action("action_benchmark_string", |param| {
        let param = param.unwrap().get::<String>();
        assert_eq!(param, "action benchmark string param");
    });

    c.bench_function("test-actions-tuple", |b| b.iter(|| test_action_tuple()));
    c.bench_function("test-actions-string", |b| b.iter(|| test_action_string()));
}

criterion_group!(benches, criterion_values);
criterion_main!(benches);
