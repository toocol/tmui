use crate::{
    application_window::ApplicationWindow,
    container::{
        ContainerLayoutEnum, ContainerScaleCalculate, ScaleStrat, StaticContainerScaleCalculate,
        StaticSizeUnifiedAdjust, SCALE_ADAPTION,
    },
    layout::LayoutMgr,
    prelude::*,
};
use tlib::{namespace::Orientation, object::ObjectSubclass};

#[extends(Container)]
pub struct HBox {
    content_halign: Align,
    content_valign: Align,
    homogeneous: bool,
}

impl ObjectSubclass for HBox {
    const NAME: &'static str = "HBox";
}

impl ObjectImpl for HBox {
    fn type_register(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<HBox, ReflectContentAlignment>();
        type_registry.register::<HBox, ReflectSizeUnifiedAdjust>();
        type_registry.register::<HBox, ReflectSpacingCapable>();
    }
}

impl WidgetImpl for HBox {}

impl ContainerImpl for HBox {
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
        ContainerLayoutEnum::HBox
    }
}

impl ContainerImplExt for HBox {
    fn add_child<T>(&mut self, mut child: Tr<T>)
    where
        T: WidgetImpl,
    {
        child.set_parent(self);
        self.container.children.push(child.clone().into());
        ApplicationWindow::initialize_dynamic_component(child.as_dyn_mut(), self.is_in_tree());
        self.update();
    }

    #[inline]
    fn remove_children(&mut self, id: ObjectId) {
        if let Some(index) = self.container.children.iter().position(|w| w.id() == id) {
            let removed = self.container.children.remove(index);

            let window = ApplicationWindow::window();
            window._add_removed_widget(removed);
            window.layout_change(self);
        }
    }
}

impl Layout for HBox {
    #[inline]
    fn composition(&self) -> Composition {
        HBox::static_composition(self)
    }

    #[inline]
    fn position_layout(&mut self, parent: Option<&dyn WidgetImpl>) {
        HBox::container_position_layout(self, parent)
    }
}

impl ContainerLayout for HBox {
    #[inline]
    fn static_composition<T: WidgetImpl + ContainerImpl>(_: &T) -> Composition {
        Composition::HorizontalArrange
    }

    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        parent: Option<&dyn WidgetImpl>,
    ) {
        LayoutMgr::base_widget_position_layout(widget, parent);

        let content_align = cast!(widget as ContentAlignment).unwrap();
        let homogeneous = content_align.homogeneous();
        let content_halign = content_align.content_halign();
        let content_valign = content_align.content_valign();

        // If homogenous was true, HBox's children's alignments was ignored, should managed by HBox's `content_halign` and `content_valign`.
        // Otherwise the position should manage by contents' own alignment.
        if homogeneous {
            hbox_layout_homogeneous(widget, content_halign, content_valign)
        } else {
            hbox_layout_non_homogeneous(widget)
        }
    }
}

impl ContentAlignment for HBox {
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

impl HBox {
    pub fn new() -> Tr<Self> {
        Self::new_alloc()
    }
}

pub(crate) fn hbox_layout_homogeneous<T: WidgetImpl + ContainerImpl>(
    widget: &mut T,
    content_halign: Align,
    content_valign: Align,
) {
    let parent_rect = widget.contents_rect(None);
    let mut spacing_cnt = 0;
    let children_total_width: i32 =
        if content_halign == Align::Center || content_halign == Align::End {
            widget
                .children()
                .iter()
                .map(|c| {
                    spacing_cnt += 1;
                    c.image_rect().width()
                })
                .sum()
        } else {
            0
        };
    let spacing = cast!(widget as SpacingCapable)
        .map(|spacing| spacing.get_spacing() as i32)
        .unwrap_or(0);

    let mut offset = match content_halign {
        Align::Start => parent_rect.x(),
        Align::Center => (parent_rect.width() - children_total_width) / 2 + parent_rect.x(),
        Align::End => parent_rect.width() - children_total_width + parent_rect.x(),
    };

    for (idx, child) in widget.children_mut().iter_mut().enumerate() {
        let spacing = if idx == 0 { 0 } else { spacing };

        child.set_fixed_x(offset + child.margin_left() + spacing);
        offset += child.image_rect().width() + spacing;

        match content_valign {
            Align::Start => child.set_fixed_y(parent_rect.y() + child.margin_top()),
            Align::Center => {
                let offset =
                    (parent_rect.height() - child.rect().height()) / 2 + child.margin_top();
                child.set_fixed_y(parent_rect.y() + offset)
            }
            Align::End => {
                let offset = parent_rect.height() - child.rect().height() + child.margin_top();
                child.set_fixed_y(parent_rect.y() + offset)
            }
        }
    }
}

fn hbox_layout_non_homogeneous<T: WidgetImpl + ContainerImpl>(widget: &mut T) {
    let parent_rect = widget.contents_rect(None);
    let mut start_childs = vec![];
    let mut center_childs = vec![];
    let mut end_childs = vec![];
    let spacing = cast!(widget as SpacingCapable)
        .map(|spacing| spacing.get_spacing() as i32)
        .unwrap_or(0);
    let mut children = widget.children_mut();

    let mut center_childs_width = 0;
    let mut end_childs_width = 0;
    let mut idx = 0;

    for child in children.iter_mut() {
        match child.valign() {
            Align::Start => child.set_fixed_y(parent_rect.y() + child.margin_top()),
            Align::Center => {
                let offset =
                    (parent_rect.height() - child.rect().height()) / 2 + child.margin_top();
                child.set_fixed_y(parent_rect.y() + offset)
            }
            Align::End => {
                let offset = parent_rect.height() - child.rect().height() + child.margin_top();
                child.set_fixed_y(parent_rect.y() + offset)
            }
        }

        match child.halign() {
            Align::Start => start_childs.push(child),
            Align::Center => {
                center_childs_width += child.image_rect().width();
                center_childs.push(child);
            }
            Align::End => {
                end_childs_width += child.image_rect().width();
                end_childs.push(child);
            }
        }
    }

    let mut offset = parent_rect.x();
    for child in start_childs {
        let spacing = if idx == 0 { 0 } else { spacing };

        child.set_fixed_x(offset + child.margin_left() + spacing);
        offset += child.image_rect().width() + spacing;

        idx += 1;
    }

    let middle_point = parent_rect.width() / 2 + parent_rect.x();
    offset = middle_point - (center_childs_width / 2);
    for child in center_childs {
        let spacing = if idx == 0 { 0 } else { spacing };

        child.set_fixed_x(offset + child.margin_left() + spacing);
        offset += child.image_rect().width() + spacing;

        idx += 1;
    }

    offset = parent_rect.x() + parent_rect.width() - end_childs_width;
    for child in end_childs {
        let spacing = if idx == 0 { 0 } else { spacing };

        child.set_fixed_x(offset + child.margin_left() + spacing);
        offset += child.image_rect().width() + spacing;

        idx += 1;
    }
}

impl ContainerScaleCalculate for HBox {
    #[inline]
    fn container_hscale_calculate(&self) -> f32 {
        Self::static_container_hscale_calculate(self)
    }

    #[inline]
    fn container_vscale_calculate(&self) -> f32 {
        Self::static_container_vscale_calculate(self)
    }
}
impl StaticContainerScaleCalculate for HBox {
    #[inline]
    fn static_container_hscale_calculate(c: &dyn ContainerImpl) -> f32 {
        match c.scale_strat() {
            ScaleStrat::Sum => c
                .children()
                .iter()
                .filter(|c| !c.fixed_width())
                .map(|c| if c.visible() { c.hscale() } else { 0. })
                .sum(),
            ScaleStrat::Direct => 1.,
        }
    }

    #[inline]
    fn static_container_vscale_calculate(_: &dyn ContainerImpl) -> f32 {
        SCALE_ADAPTION
    }
}

impl SizeUnifiedAdjust for HBox {
    #[inline]
    fn size_unified_adjust(&mut self) {
        Self::static_size_unified_adjust(self)
    }
}
impl StaticSizeUnifiedAdjust for HBox {
    #[inline]
    fn static_size_unified_adjust(container: &mut dyn ContainerImpl) {
        let mut children_width = 0;
        let children_cnt = container.children().len();
        if children_cnt == 0 {
            return;
        }

        container.children().iter().for_each(|c| {
            children_width += c.size().width();
        });

        let width = container.size().width();
        if width >= children_width {
            return;
        }

        let exceed_width = children_width - width;
        let width_to_reduce = (exceed_width as f32 / children_cnt as f32).round() as i32;
        let gap = exceed_width - width_to_reduce * children_cnt as i32;

        let strict_children_layout = container.is_strict_children_layout();

        container
            .children_mut()
            .iter_mut()
            .enumerate()
            .for_each(|(i, c)| {
                let mut cw = (c.size().width() - width_to_reduce).max(0);

                // Handle to ensure that there are no gaps between all subcomponents:
                if i == children_cnt - 1 {
                    cw -= gap;
                }

                if strict_children_layout {
                    let size_hint = c.size_hint();
                    if let Some(min_width) = size_hint.min_width() {
                        if cw < min_width {
                            cw = min_width;
                        }
                    }
                }

                c.set_fixed_width(cw);
            });
    }
}

impl SpacingCapable for HBox {
    #[inline]
    fn orientation(&self) -> Orientation {
        Orientation::Horizontal
    }
}
