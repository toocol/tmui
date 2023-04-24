use tlib::object::ObjectSubclass;
use tmui::prelude::*;

#[extends(Object)]
struct Foo {}

impl ObjectSubclass for Foo {
    const NAME: &'static str = "Foo";

    type Type = Foo;

    type ParentType = Object;
}

impl ObjectImpl for Foo {}

#[test]
fn main() {

}