use tlib::object::ObjectSubclass;
use tmui::{label::Label, platform::PlatformType, prelude::*};

#[extends(Widget, Layout(Stack))]
#[derive(Childrenable)]
struct TestWidget {
    #[children]
    label: Box<Label>,
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
    ActionHub::initialize();
    let _window = ApplicationWindow::new(PlatformType::default(), 0, 0);

    let widget: Box<TestWidget> = Object::new(&[]);
    let children = widget.children();
    assert_eq!(1, children.len());
    let label_dyn = *children.first().unwrap();
    assert_eq!(widget.label.id(), label_dyn.id());
    let label = label_dyn.downcast_ref::<Label>().unwrap();
    assert_eq!("Hello World", label.text())
}