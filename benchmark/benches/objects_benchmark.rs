use std::collections::HashMap;

use criterion::{criterion_group, criterion_main, Criterion};
use tlib::{
    object::{ObjectImpl, ObjectSubclass},
    prelude::*,
};

#[extends_object]
pub struct Widget {}
impl ObjectSubclass for Widget {
    const NAME: &'static str = "Widget";

    type Type = Widget;

    type ParentType = Object;
}
impl ObjectImpl for Widget {}

pub fn object_downcast_test<T: IsA<Object>>(widget: &T) {
    let _ = widget.downcast_ref::<Widget>();
}

pub fn object_property_test(widget: &Widget, tuple: &(i32, i32, Vec<String>)) {
    widget.set_property("property-1", tuple.to_value());
    let val = widget
        .get_property("property-1")
        .unwrap()
        .get::<(i32, i32, Vec<String>)>();
    assert_eq!(&val, tuple);
}

pub fn hashmap_property_test(
    map: &mut HashMap<String, (i32, i32, Vec<String>)>,
    tuple: &(i32, i32, Vec<String>),
) {
    map.insert("property-1".to_string(), tuple.clone());
    let val = map.get("property-1").unwrap();
    assert_eq!(val, tuple);
}

pub fn criterion_values(c: &mut Criterion) {
    let widget: Widget = Object::new(&[]);
    let tuple = (
        i32::MAX,
        i32::MIN,
        vec![
            "value1".to_string(),
            "value2".to_string(),
            "value3".to_string(),
            "value4".to_string(),
        ],
    );
    let mut map = HashMap::new();

    c.bench_function("object-downcast-test", |b| {
        b.iter(|| object_downcast_test(&widget))
    });
    c.bench_function("object-property-test", |b| {
        b.iter(|| object_property_test(&widget, &tuple))
    });
    c.bench_function("object-hashmap-property-test", |b| {
        b.iter(|| hashmap_property_test(&mut map, &tuple))
    });
}

criterion_group!(benches, criterion_values);
criterion_main!(benches);
