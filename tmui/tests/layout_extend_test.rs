use tlib::object::ObjectSubclass;
use tmui::{prelude::*, label::Label};

#[extends(Widget, Layout(Stack))]
#[derive(Default, Childrenable)]
struct TestWidget {
    #[children]
    label: Label,
}

impl ObjectSubclass for TestWidget {
    const NAME: &'static str = "TestWidget";
}

impl ObjectImpl for TestWidget {
    fn construct(&mut self) {
        self.parent_construct();
        self.label.set_text("Hello World");
    }
}

impl WidgetImpl for TestWidget {}

#[test]
fn main() {
    let widget: TestWidget = Object::new(&[]);
    let children = widget.children();
    assert_eq!(1, children.len());
    let label_dyn = *children.first().unwrap();
    assert_eq!(widget.label.id(), label_dyn.id());
    let label = label_dyn.as_any().downcast_ref::<Label>().unwrap();
    assert_eq!("Hello World", label.text())
}