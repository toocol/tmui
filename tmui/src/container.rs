use crate::{
    graphics::element::HierachyZ,
    prelude::*,
    widget::{ScaleCalculate, WidgetImpl, WindowAcquire},
};
use tlib::{object::{ObjectImpl, ObjectSubclass}, skia_safe::region::RegionOp};

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
            "visible" => {
                let visible = value.get::<bool>();
                for child in self.children.iter_mut() {
                    if visible {
                        child.show()
                    } else {
                        child.hide()
                    }
                }
            }
            "z_index" => {
                if !ApplicationWindow::window_of(self.window_id()).initialized() {
                    return;
                }
                let offset = value.get::<u32>() - self.z_index();
                for child in self.children.iter_mut() {
                    child.set_z_index(child.z_index() + offset);
                }
            }
            "rerender_styles" => {
                let rerender = value.get::<bool>();
                if rerender {
                    for child in self.children.iter_mut() {
                        child.set_rerender_styles(true)
                    }
                }
            }
            "minimized" => {
                let minimized = value.get::<bool>();
                if minimized {
                    for child in self.children.iter_mut() {
                        child.set_minimized(true)
                    }
                }
            }
            "propagate_update" => {
                let propagate_update = value.get::<bool>();
                if propagate_update {
                    for child in self.children.iter_mut() {
                        child.propagate_update()
                    }
                }
            }
            _ => {}
        }
    }
}

impl WidgetImpl for Container {}

pub trait ContainerAcquire: ContainerImpl + ContainerImplExt + Default {}

#[reflect_trait]
pub trait ContainerImpl: WidgetImpl + ContainerPointEffective + ContainerScaleCalculate {
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
            if child.visible() && child.rect().contains(point) {
                return false;
            }
        }

        true
    }
}

pub trait ChildrenRegionAcquirer {
    fn children_region(&self) -> tlib::skia_safe::Region;
}
impl<T: ContainerImpl> ChildrenRegionAcquirer for T {
    fn children_region(&self) -> tlib::skia_safe::Region {
        let mut region = tlib::skia_safe::Region::new();
        for c in self.children() {
            let rect: tlib::skia_safe::IRect = c.rect().into();
            region.op_rect(rect, RegionOp::Union);
        }
        region
    }
}

impl ScaleCalculate for dyn ContainerImpl {
    #[inline]
    fn hscale_calculate(&self) -> f32 {
        self.container_hscale_calculate()
    }

    #[inline]
    fn vscale_calculate(&self) -> f32 {
        self.container_vscale_calculate()
    }
}

pub trait ContainerScaleCalculate {
    fn container_hscale_calculate(&self) -> f32;

    fn container_vscale_calculate(&self) -> f32;
}
pub trait StaticContainerScaleCalculate {
    fn static_container_hscale_calculate(c: &dyn ContainerImpl) -> f32;

    fn static_container_vscale_calculate(c: &dyn ContainerImpl) -> f32;
}

pub const SCALE_ADAPTION: f32 = f32::MAX;
pub const SCALE_DISMISS: f32 = f32::MIN;
pub trait ScaleMeasure {
    fn is_adaption(&self) -> bool;

    fn is_dismiss(&self) -> bool;
}
impl ScaleMeasure for f32 {
    #[inline]
    fn is_adaption(&self) -> bool {
        *self == SCALE_ADAPTION
    }

    #[inline]
    fn is_dismiss(&self) -> bool {
        *self == SCALE_DISMISS
    }
}

impl WindowAcquire for dyn ContainerImpl {
    #[inline]
    fn window(&self) -> &'static mut ApplicationWindow {
        ApplicationWindow::window_of(self.window_id())
    }
}

#[reflect_trait]
pub trait SizeUnifiedAdjust {
    fn size_unified_adjust(&mut self);
}