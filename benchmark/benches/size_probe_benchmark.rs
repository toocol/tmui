use criterion::{criterion_group, criterion_main, Criterion};
use tlib::{object::ObjectSubclass, prelude::*};
use tmui::{application_window::ApplicationWindow, platform::PlatformType, prelude::*};

pub fn criterion_values(c: &mut Criterion) {
    c.bench_function("size-probe-bench", |b| {
        b.iter(|| {
            let mut window = ApplicationWindow::new(PlatformType::default(), 1280, 800);
            window.window_layout_change();

            let (mut w1, mut w2, mut w3, mut w4, mut w5) = (
                Stack::new(),
                Stack::new(),
                Stack::new(),
                TestWidget::new(),
                TestWidget::new(),
            );
            for _ in 0..100 {
                w3.add_child(w1);
                w3.add_child(w2);
                w4.child(w3);
                w1 = Stack::new();
                w2 = Stack::new();
                w3 = Stack::new();
                w5 = w4;
                w4 = TestWidget::new();
            }

            window.child(w5)
        })
    });
}

criterion_group!(benches, criterion_values);
criterion_main!(benches);

#[extends(Widget)]
pub struct TestWidget {}

impl ObjectSubclass for TestWidget {
    const NAME: &'static str = "TestWidget";
}

impl ObjectImpl for TestWidget {}

impl WidgetImpl for TestWidget {}

impl TestWidget {
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
