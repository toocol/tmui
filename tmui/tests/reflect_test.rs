use tlib::object::{ObjectImpl, ObjectSubclass};
use tmui::prelude::*;

#[reflect_trait]
pub trait DoA {
    fn do_a(&self);
}

#[reflect_trait]
pub trait DoB {
    fn do_b(&self);
}

#[extends_object]
#[derive(Default)]
pub struct Foo {}
impl ObjectSubclass for Foo {
    const NAME: &'static str = "Foo";

    type Type = Foo;

    type ParentType = Object;
}
impl ObjectImpl for Foo {}
impl DoA for Foo {
    fn do_a(&self) {
        println!("Foo do a");
    }
}
impl DoB for Foo {
    fn do_b(&self) {
        println!("Foo do b");
    }
}

#[extends_object]
#[derive(Default)]
pub struct Bar {}
impl ObjectSubclass for Bar {
    const NAME: &'static str = "Bar";

    type Type = Bar;

    type ParentType = Object;
}
impl ObjectImpl for Bar {}
impl DoB for Bar {
    fn do_b(&self) {
        println!("Bar do b");
    }
}

#[test]
fn main() {
    let mut registry = TypeRegistry::new();
    registry.initialize();
    registry.register::<Foo, ReflectDoA>();
    registry.register::<Foo, ReflectDoB>();
    registry.register::<Bar, ReflectDoB>();

    let mut cnt = 0;
    let list: Vec<Box<dyn Reflect>> = vec![Box::new(Foo::default()), Box::new(Bar::default())];
    for item in list.iter() {
        let item_ref = item.as_ref();
        if let Some(reflect) = TypeRegistry::get_type_data::<ReflectDoA>(item_ref) {
            (reflect.get_func)(item_ref).do_a();
            cnt += 1;
        }
        if let Some(reflect) = TypeRegistry::get_type_data::<ReflectDoB>(item_ref) {
            (reflect.get_func)(item_ref).do_b();
            cnt += 1;
        }
    }
    assert_eq!(3, cnt)
}
