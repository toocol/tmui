use crate::{
    application_window::ApplicationWindow, container::ContainerScaleCalculate,
    layout::LayoutManager, prelude::*,
};
use tlib::object::ObjectSubclass;

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
    }
}

impl WidgetImpl for HBox {}

impl ContainerImpl for HBox {
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

impl ContainerImplExt for HBox {
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

impl Layout for HBox {
    fn composition(&self) -> Composition {
        HBox::static_composition()
    }

    fn position_layout(
        &mut self,
        previous: Option<&dyn WidgetImpl>,
        parent: Option<&dyn WidgetImpl>,
        manage_by_container: bool,
    ) {
        HBox::container_position_layout(self, previous, parent, manage_by_container)
    }
}

impl ContainerLayout for HBox {
    fn static_composition() -> Composition {
        Composition::HorizontalArrange
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
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}

fn hbox_layout_homogeneous<T: WidgetImpl + ContainerImpl>(
    widget: &mut T,
    content_halign: Align,
    content_valign: Align,
) {
    let parent_rect = widget.contents_rect(None);
    let children_total_width: i32 =
        if content_halign == Align::Center || content_halign == Align::End {
            widget
                .children()
                .iter()
                .map(|c| c.image_rect().width())
                .sum()
        } else {
            0
        };
    debug_assert!(parent_rect.width() >= children_total_width);

    let mut offset = match content_halign {
        Align::Start => parent_rect.x(),
        Align::Center => (parent_rect.width() - children_total_width) / 2 + parent_rect.x(),
        Align::End => parent_rect.width() - children_total_width + parent_rect.x(),
    };

    for child in widget.children_mut().iter_mut() {
        child.set_fixed_x(offset + child.margin_left());
        offset += child.image_rect().width();

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
    let mut totoal_children_width = 0;
    let mut children = widget.children_mut();

    let mut center_childs_width = 0;
    let mut end_childs_width = 0;

    for child in children.iter_mut() {
        totoal_children_width += child.image_rect().width();
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

    debug_assert!(parent_rect.height() >= totoal_children_width);

    let mut offset = parent_rect.x();
    for child in start_childs {
        child.set_fixed_x(offset);
        offset += child.image_rect().height();
    }

    let middle_point = parent_rect.width() / 2 + parent_rect.x();
    offset = middle_point - (center_childs_width / 2);
    for child in center_childs {
        child.set_fixed_x(offset);
        offset += child.image_rect().height();
    }

    offset = parent_rect.x() + parent_rect.width() - end_childs_width;
    for child in end_childs {
        child.set_fixed_x(offset);
        offset += child.image_rect().height();
    }
}

impl ContainerScaleCalculate for HBox {
    #[inline]
    fn container_hscale_calculate(&self) -> f32 {
        self.children().iter().map(|c| c.hscale()).sum()
    }

    #[inline]
    fn container_vscale_calculate(&self) -> f32 {
        f32::MAX
    }
}
