use crate::{prelude::*, widget::WidgetImpl};
use tlib::object::{ObjectImpl, ObjectSubclass};

#[extends(Widget)]
pub struct Container {
    pub children: Vec<Box<dyn WidgetImpl>>,
}

impl ObjectSubclass for Container {
    const NAME: &'static str = "Container";
}

impl ObjectImpl for Container {
    fn on_property_set(&mut self, name: &str, value: &Value) {
        self.parent_on_property_set(name, value);

        match name {
            "invalidate" => {
                for child in self.children.iter_mut() {
                    child.update()
                }
            }
            _ => {}
        }
    }
}

impl WidgetImpl for Container {}

pub trait ContainerAcquire: ContainerImpl + ContainerImplExt + Default {}

#[reflect_trait]
pub trait ContainerImpl: WidgetImpl + ContainerPointEffective {
    /// Go to[`Function defination`](ContainerImpl::children) (Defined in [`ContainerImpl`])
    /// Get all the children ref in `Container`
    fn children(&self) -> Vec<&dyn WidgetImpl>;

    /// Go to[`Function defination`](ContainerImpl::children_mut) (Defined in [`ContainerImpl`])
    /// Get all the mut children ref in `Container`
    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl>;
}

pub trait ContainerImplExt: ContainerImpl {
    /// Go to[`Function defination`](ContainerImplExt::add_child) (Defined in [`ContainerImplExt`])
    fn add_child<T>(&mut self, child: Box<T>)
    where
        T: WidgetImpl;
}

pub trait ContainerPointEffective {
    fn container_point_effective(&self, point: &Point) -> bool;
}
impl<T: ContainerImpl> ContainerPointEffective for T {
    fn container_point_effective(&self, point: &Point) -> bool {
        let self_rect = self.rect();

        if !self_rect.contains(point) {
            return false;
        }

        for child in self.children() {
            if child.rect().contains(point) {
                return false;
            }
        }

        true
    }
}
