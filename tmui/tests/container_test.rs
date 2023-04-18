use tmui::prelude::{Container, StaticType, IsA};
use tmui::widget::{WidgetImpl, Widget};

fn main() {
    let container = Container::default();
    test_type(&container)
}

fn test_type<T: WidgetImpl + IsA<Widget>>(widget: &T) {
    let type_ = widget.object_type();
    println!("{:?}", type_);
    assert_eq!(Container::static_type(), type_);
}