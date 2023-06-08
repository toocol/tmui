use tlib::object::ObjectSubclass;
use tmui::prelude::*;

#[extends(Object)]
struct Foo {}

impl ObjectSubclass for Foo {
    const NAME: &'static str = "Foo";
}

impl ObjectImpl for Foo {}

#[test]
fn main() {

}