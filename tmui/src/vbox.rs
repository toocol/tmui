use crate::{application_window::ApplicationWindow, layout::LayoutManager, prelude::*};
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
        ApplicationWindow::initialize_dynamic_component(self, child.as_mut());
        child.set_parent(self);
        self.container.children.push(child);
        self.update();
    }
}

impl Layout for VBox {
    fn composition(&self) -> Composition {
        VBox::static_composition()
    }

    fn position_layout(
        &mut self,
        previous: &dyn WidgetImpl,
        parent: &dyn WidgetImpl,
        manage_by_container: bool,
    ) {
        VBox::container_position_layout(self, previous, parent, manage_by_container)
    }
}

impl ContainerLayout for VBox {
    fn static_composition() -> Composition {
        Composition::VerticalArrange
    }

    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        previous: &dyn WidgetImpl,
        parent: &dyn WidgetImpl,
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
    debug_assert!(parent_rect.height() >= children_total_height);

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

    debug_assert!(parent_rect.height() >= totoal_children_height);

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
