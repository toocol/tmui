use crate::{
    application_window::ApplicationWindow,
    container::{ContainerScaleCalculate, StaticContainerScaleCalculate, SCALE_ADAPTION, StaticSizeUnifiedAdjust},
    layout::LayoutManager,
    prelude::*, graphics::painter::Painter,
};
use derivative::Derivative;
use log::debug;
use tlib::object::ObjectSubclass;

#[extends(Container)]
pub struct VBox {
    content_halign: Align,
    content_valign: Align,
    #[derivative(Default(value = "true"))]
    homogeneous: bool,
}

impl ObjectSubclass for VBox {
    const NAME: &'static str = "VBox";
}

impl ObjectImpl for VBox {
    fn type_register(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<VBox, ReflectContentAlignment>();
        type_registry.register::<VBox, ReflectSizeUnifiedAdjust>();
    }
}

impl WidgetImpl for VBox {}

impl ContainerImpl for VBox {
    fn children(&self) -> Vec<&dyn WidgetImpl> {
        self.container.children.iter().map(|c| c.as_ref()).collect()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
        self.container
            .children
            .iter_mut()
            .map(|c| c.as_mut())
            .collect()
    }
}

impl ContainerImplExt for VBox {
    fn add_child<T>(&mut self, mut child: Box<T>)
    where
        T: WidgetImpl,
    {
        child.set_parent(self);
        ApplicationWindow::initialize_dynamic_component(child.as_mut());
        self.container.children.push(child);
        self.update();
    }
}

impl Layout for VBox {
    fn composition(&self) -> Composition {
        VBox::static_composition(self)
    }

    fn position_layout(
        &mut self,
        previous: Option<&dyn WidgetImpl>,
        parent: Option<&dyn WidgetImpl>,
        manage_by_container: bool,
    ) {
        VBox::container_position_layout(self, previous, parent, manage_by_container)
    }
}

impl ContainerLayout for VBox {
    fn static_composition<T: WidgetImpl + ContainerImpl>(_: &T) -> Composition {
        Composition::VerticalArrange
    }

    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        previous: Option<&dyn WidgetImpl>,
        parent: Option<&dyn WidgetImpl>,
        manage_by_container: bool,
    ) {
        LayoutManager::base_widget_position_layout(widget, previous, parent, manage_by_container);

        let content_align = cast!(widget as ContentAlignment).unwrap();
        let homogeneous = content_align.homogeneous();
        let content_halign = content_align.content_halign();
        let content_valign = content_align.content_valign();

        // If homogenous was true, VBox's children's alignments was ignored, should managed by VBox's `content_halign` and `content_valign`.
        // Otherwise the position should manage by contents' own alignment.
        if homogeneous {
            vbox_layout_homogeneous(widget, content_halign, content_valign)
        } else {
            vbox_layout_non_homogeneous(widget)
        }
    }
}

impl ContentAlignment for VBox {
    #[inline]
    fn homogeneous(&self) -> bool {
        self.homogeneous
    }

    #[inline]
    fn set_homogeneous(&mut self, homogeneous: bool) {
        self.homogeneous = homogeneous
    }

    #[inline]
    fn content_halign(&self) -> Align {
        self.content_halign
    }

    #[inline]
    fn content_valign(&self) -> Align {
        self.content_valign
    }

    #[inline]
    fn set_content_halign(&mut self, halign: Align) {
        self.content_halign = halign
    }

    #[inline]
    fn set_content_valign(&mut self, valign: Align) {
        self.content_valign = valign
    }
}

impl VBox {
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}

fn vbox_layout_homogeneous<T: WidgetImpl + ContainerImpl>(
    widget: &mut T,
    content_halign: Align,
    content_valign: Align,
) {
    let parent_rect = widget.contents_rect(None);
    let is_strict_children_layout = widget.is_strict_children_layout();
    let children_total_height: i32 =
        if content_valign == Align::Center || content_valign == Align::End {
            widget
                .children()
                .iter()
                .map(|c| c.image_rect().height())
                .sum()
        } else {
            0
        };

    if !is_strict_children_layout {
        debug_assert!(parent_rect.height() >= children_total_height);
    }

    let mut offset = match content_valign {
        Align::Start => parent_rect.y(),
        Align::Center => (parent_rect.height() - children_total_height) / 2 + parent_rect.y(),
        Align::End => parent_rect.height() - children_total_height + parent_rect.y(),
    };

    for child in widget.children_mut().iter_mut() {
        debug!("Layout widget position: {}", child.type_name());
        match content_halign {
            Align::Start => child.set_fixed_x(parent_rect.x() + child.margin_left()),
            Align::Center => {
                let offset = (parent_rect.width() - child.rect().width()) / 2 + child.margin_left();
                child.set_fixed_x(parent_rect.x() + offset);
            }
            Align::End => {
                let offset = parent_rect.width() - child.rect().width() + child.margin_left();
                child.set_fixed_x(parent_rect.x() + offset)
            }
        }

        child.set_fixed_y(offset + child.margin_top());
        offset += child.image_rect().height();
    }
}

fn vbox_layout_non_homogeneous<T: WidgetImpl + ContainerImpl>(widget: &mut T) {
    let parent_rect = widget.contents_rect(None);
    let is_strict_children_layout = widget.is_strict_children_layout();
    let mut start_childs = vec![];
    let mut center_childs = vec![];
    let mut end_childs = vec![];
    let mut totoal_children_height = 0;
    let mut children = widget.children_mut();

    let mut center_childs_height = 0;
    let mut end_childs_height = 0;

    for child in children.iter_mut() {
        totoal_children_height += child.image_rect().height();
        match child.halign() {
            Align::Start => child.set_fixed_x(parent_rect.x() + child.margin_left()),
            Align::Center => {
                let offset = (parent_rect.width() - child.rect().width()) / 2 + child.margin_left();
                child.set_fixed_x(parent_rect.x() + offset);
            }
            Align::End => {
                let offset = parent_rect.width() - child.rect().width() + child.margin_left();
                child.set_fixed_x(parent_rect.x() + offset)
            }
        }

        match child.valign() {
            Align::Start => start_childs.push(child),
            Align::Center => {
                center_childs_height += child.image_rect().height();
                center_childs.push(child)
            }
            Align::End => {
                end_childs_height += child.image_rect().height();
                end_childs.push(child)
            }
        }
    }

    if !is_strict_children_layout {
        debug_assert!(parent_rect.height() >= totoal_children_height);
    }

    let mut offset = parent_rect.y();
    for child in start_childs {
        child.set_fixed_y(offset);
        offset += child.image_rect().height();
    }

    let middle_point = parent_rect.height() / 2 + parent_rect.y();
    offset = middle_point - (center_childs_height / 2);
    for child in center_childs {
        child.set_fixed_y(offset);
        offset += child.image_rect().height();
    }

    offset = parent_rect.y() + parent_rect.height() - end_childs_height;
    for child in end_childs {
        child.set_fixed_y(offset);
        offset += child.image_rect().height();
    }
}

impl ContainerScaleCalculate for VBox {
    #[inline]
    fn container_hscale_calculate(&self) -> f32 {
        Self::static_container_hscale_calculate(self)
    }

    #[inline]
    fn container_vscale_calculate(&self) -> f32 {
        Self::static_container_vscale_calculate(self)
    }
}
impl StaticContainerScaleCalculate for VBox {
    #[inline]
    fn static_container_hscale_calculate(_: &dyn ContainerImpl) -> f32 {
        SCALE_ADAPTION
    }

    #[inline]
    fn static_container_vscale_calculate(c: &dyn ContainerImpl) -> f32 {
        c.children().iter().map(|c| c.vscale()).sum()
    }
}

impl ChildContainerDiffRender for VBox {
    fn container_diff_render(&mut self, _painter: &mut Painter, _background: Color) {
    }
}

impl SizeUnifiedAdjust for VBox {
    #[inline]
    fn size_unified_adjust(&mut self) {
        Self::static_size_unified_adjust(self)
    }
}
impl StaticSizeUnifiedAdjust for VBox {
    #[inline]
    fn static_size_unified_adjust(container: &mut dyn ContainerImpl) {
        let mut children_height = 0;
        let children_cnt = container.children().len();
        if children_cnt == 0 {
            return
        }

        container.children().iter().for_each(|c| {
            children_height += c.size().height();
        });

        let height = container.size().height();
        debug_assert!(height < children_height);

        let exceed_height = children_height - height;
        let height_to_reduce = (exceed_height as f32 / children_cnt as f32).round() as i32;
        let gap = exceed_height - height_to_reduce * children_cnt as i32;

        let strict_children_layout = container.is_strict_children_layout();

        container.children_mut().iter_mut().enumerate().for_each(|(i, c)| {
            let mut ch = (c.size().height() - height_to_reduce).max(0);

            // Handle to ensure that there are no gaps between all subcomponents:
            if i == children_cnt - 1 {
                ch -= gap;
            }
            
            if strict_children_layout {
                let (minimum_hint, _) = c.size_hint();
                if let Some(minimum_hint) = minimum_hint {
                    if ch < minimum_hint.height() {
                        ch = minimum_hint.height();
                    }
                }
            }

            c.set_fixed_height(ch);
        });
    }
}