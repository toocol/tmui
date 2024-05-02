use crate::{
    graphics::element::HierachyZ,
    prelude::*,
    widget::{ScaleCalculate, WidgetImpl},
};
use tlib::{
    namespace::Orientation,
    object::{ObjectImpl, ObjectSubclass},
    skia_safe::region::RegionOp,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ContainerLayoutEnum {
    Stack,
    VBox,
    HBox,
    SplitPane,
    ScrollArea,
    Pane,
    Overlay,
}

#[extends(Widget)]
pub struct Container {
    pub children: Vec<Box<dyn WidgetImpl>>,

    /// The container will restrictly respect childrens' `size_hint` or not.
    ///
    /// `false`: <br>
    /// - Container layout will attempt to respect the `size_hint` of each subcomponent.
    ///   But when space is insufficient, it will compress these components,
    ///   which may result in them being smaller than the size specified by their `size_hint`.
    ///
    /// `true`: <br>
    /// - Container layout will strictly respect the `size_hint` of each subcomponent,
    ///   the parts beyond the size range will be hidden.
    strict_children_layout: bool,
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
            "propagate_update_rect" => {
                let rect = value.get::<CoordRect>();
                for child in self.children.iter_mut() {
                    child.propagate_update_rect(rect)
                }
            }
            "propagate_update_styles_rect" => {
                let rect = value.get::<CoordRect>();
                for child in self.children.iter_mut() {
                    child.propagate_update_styles_rect(rect)
                }
            }
            "animation_progressing" => {
                let is = value.get::<bool>();
                for child in self.children.iter_mut() {
                    child.propagate_animation_progressing(is)
                }
            }
            "propagate_transparency" => {
                let transparency = value.get::<Transparency>();
                for child in self.children.iter_mut() {
                    child.propagate_set_transparency(transparency)
                }
            }
            "overlaid_rect" => {
                let overlaid = value.get::<Rect>();
                for child in self.children.iter_mut() {
                    child.set_overlaid_rect(overlaid)
                }
            }
            _ => {}
        }
    }
}

impl WidgetImpl for Container {}

pub trait ContainerAcquire: ContainerImpl + ContainerImplExt + Default {}

pub trait ContainerExt {
    fn is_strict_children_layout(&self) -> bool;

    fn set_strict_children_layout(&mut self, strict_children_layout: bool);
}
impl ContainerExt for Container {
    #[inline]
    fn is_strict_children_layout(&self) -> bool {
        self.strict_children_layout
    }

    #[inline]
    fn set_strict_children_layout(&mut self, strict_children_layout: bool) {
        self.strict_children_layout = strict_children_layout
    }
}

#[reflect_trait]
pub trait ContainerImpl:
    WidgetImpl + ContainerPointEffective + ContainerScaleCalculate + ContainerExt
{
    /// Go to[`Function defination`](ContainerImpl::children) (Defined in [`ContainerImpl`])
    /// Get all the children ref in `Container`.
    fn children(&self) -> Vec<&dyn WidgetImpl>;

    /// Go to[`Function defination`](ContainerImpl::children_mut) (Defined in [`ContainerImpl`])
    /// Get all the mut children ref in `Container`.
    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl>;

    /// Get the layout of container.
    fn container_layout(&self) -> ContainerLayoutEnum;
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
            if !c.visible() && !c.is_animation_progressing() {
                continue;
            }

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

#[reflect_trait]
pub trait SizeUnifiedAdjust {
    fn size_unified_adjust(&mut self);
}
pub trait StaticSizeUnifiedAdjust {
    fn static_size_unified_adjust(container: &mut dyn ContainerImpl);
}

pub struct SpacingSize {
    spacing: i32,
    orientation: Orientation,
}
impl SpacingSize {
    #[inline]
    pub fn spacing(&self) -> i32 {
        self.spacing
    }

    #[inline]
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    #[inline]
    pub fn remove_spacing_from(&self, size: &mut Size) {
        match self.orientation {
            Orientation::Horizontal => size.set_width(size.width() - self.spacing),
            Orientation::Vertical => size.set_height(size.height() - self.spacing),
        }
    }
}

const SPACING_PROPERTY_NAME: &str = "_container_spacing";
#[reflect_trait]
pub trait SpacingCapable: ContainerImpl {
    fn orientation(&self) -> Orientation;

    #[inline]
    fn get_spacing(&self) -> u16 {
        self.get_property(SPACING_PROPERTY_NAME)
            .map(|v| v.get::<u16>())
            .unwrap_or(0)
    }

    #[inline]
    fn set_spacing(&mut self, spacing: u16) {
        self.set_property(SPACING_PROPERTY_NAME, spacing.to_value())
    }

    #[inline]
    fn spacing_size(&self) -> SpacingSize {
        let spacing = (self.get_spacing() as i32) * (self.children().len() as i32 - 1);

        SpacingSize {
            spacing,
            orientation: self.orientation(),
        }
    }
}
