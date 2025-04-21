use tlib::object::{ObjectImpl, ObjectSubclass};
use tmui::{
    container::{
        ContainerImpl, ContainerImplExt, ContainerLayoutEnum, ContainerScaleCalculate,
        SCALE_ADAPTION,
    },
    label::Label,
    prelude::*,
    widget::{Widget, WidgetImpl},
};

#[extends(Container)]
#[derive(Childrenable)]
pub struct TestContainer {
    #[derivative(Default(value = "99"))]
    _num: i32,
    #[children]
    _label: Tr<Label>,
}

impl ObjectSubclass for TestContainer {
    const NAME: &'static str = "TestContainer";
}

impl ObjectImpl for TestContainer {}

impl WidgetImpl for TestContainer {}

impl ContainerImpl for TestContainer {
    fn children(&self) -> Vec<&dyn WidgetImpl> {
        self.container.children.iter().map(|c| c.bind()).collect()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
        self.container
            .children
            .iter_mut()
            .map(|c| c.bind_mut())
            .collect()
    }

    fn container_layout(&self) -> ContainerLayoutEnum {
        ContainerLayoutEnum::Stack
    }
}

impl ContainerImplExt for TestContainer {
    fn add_child<T>(&mut self, child: Tr<T>)
    where
        T: WidgetImpl,
    {
        self.container.children.push(child.into())
    }

    fn remove_children(&mut self, _: ObjectId) {}
}

impl Layout for TestContainer {
    fn composition(&self) -> tmui::layout::Composition {
        tmui::layout::Composition::HorizontalArrange
    }

    fn position_layout(&mut self, _: Option<&dyn WidgetImpl>) {}
}

impl ContainerScaleCalculate for TestContainer {
    fn container_hscale_calculate(&self) -> f32 {
        SCALE_ADAPTION
    }

    fn container_vscale_calculate(&self) -> f32 {
        SCALE_ADAPTION
    }
}

#[test]
fn main() {
    ActionHub::initialize();
    let r = TypeRegistry::instance();
    let mut c = TestContainer::new_alloc();
    c.inner_type_register(r);
    cast_test(c.bind());
    cast_mut_test(c.bind_mut());
}

fn cast_test(widget: &dyn WidgetImpl) {
    let mut c = false;
    if let Some(_) = cast!(widget as ContainerImpl) {
        c = true;
    }
    assert!(c);
}

fn cast_mut_test(widget: &mut dyn WidgetImpl) {
    let mut c = false;
    if let Some(_) = cast_mut!(widget as ContainerImpl) {
        c = true;
    }
    assert!(c);
}
