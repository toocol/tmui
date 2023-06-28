use crate::{
    application_window::ApplicationWindow,
    layout::LayoutManager,
    prelude::*,
    tlib::{
        object::{ObjectImpl, ObjectSubclass},
        values::{FromBytes, FromValue, ToBytes, ToValue},
    },
    widget::WidgetImpl,
};
use std::{
    collections::HashMap,
    mem::size_of,
    ptr::NonNull,
};
use tlib::{implements_enum_value, namespace::AsNumeric, nonnull_mut, split_pane_impl};

#[extends(Container)]
pub struct SplitPane {
    /// The HashMap to hold all the ownership of SplitInfo.
    split_infos: HashMap<u16, Box<SplitInfo>>,
    /// The vector to hold all the raw pointer of SplitInfo, ensure execution order.
    split_infos_vec: Vec<Option<NonNull<SplitInfo>>>,
}

impl ObjectSubclass for SplitPane {
    const NAME: &'static str = "SplitPane";
}

impl ObjectImpl for SplitPane {
    fn type_register(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<SplitPane, ReflectSplitInfosGetter>()
    }
}

impl WidgetImpl for SplitPane {}

impl ContainerImpl for SplitPane {
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

impl ContainerImplExt for SplitPane {
    fn add_child<T>(&mut self, mut child: Box<T>)
    where
        T: WidgetImpl,
    {
        if self.container.children.len() != 0 {
            panic!("Only first widget can use function `add_child()` to add, please use `split_left()`,`split_top()`,`split_right()` or `split_down()`")
        }
        ApplicationWindow::initialize_dynamic_component(self, child.as_mut());
        child.set_parent(self);
        let widget_ptr: Option<NonNull<dyn WidgetImpl>> = NonNull::new(child.as_mut());
        let mut split_info = Box::new(SplitInfo::new(
            child.id(),
            widget_ptr.clone(),
            None,
            SplitType::SplitNone,
        ));
        self.split_infos_vec.push(NonNull::new(split_info.as_mut()));
        self.split_infos.insert(child.id(), split_info);
        self.container.children.push(child);
        self.update();
    }
}

impl Layout for SplitPane {
    fn composition(&self) -> Composition {
        SplitPane::static_composition()
    }

    fn position_layout(
        &mut self,
        previous: &dyn WidgetImpl,
        parent: &dyn WidgetImpl,
        manage_by_container: bool,
    ) {
        SplitPane::container_position_layout(self, previous, parent, manage_by_container)
    }
}

impl ContainerLayout for SplitPane {
    fn static_composition() -> Composition {
        Composition::FixedContainer
    }

    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        previous: &dyn WidgetImpl,
        parent: &dyn WidgetImpl,
        manage_by_container: bool,
    ) {
        LayoutManager::base_widget_position_layout(widget, previous, parent, manage_by_container);

        let parent_rect = widget.contents_rect(None);
        let split_infos_getter = cast_mut!(widget as SplitInfosGetter).unwrap();
        for split_info in split_infos_getter.split_infos_vec() {
            nonnull_mut!(split_info).calculate_layout(parent_rect)
        }
    }
}

#[macro_export]
macro_rules! split_widget {
    ( $st:ident ) => {
        unsafe { $st.widget.as_mut().unwrap().as_mut() }
    };
}
#[macro_export]
macro_rules! split_from {
    ( $st:ident ) => {
        unsafe { $st.split_from.as_mut().unwrap().as_mut() }
    };
}

#[reflect_trait]
pub trait SplitInfosGetter {
    /// Get the split infos map.
    fn split_infos(&mut self) -> &mut HashMap<u16, Box<SplitInfo>>;

    /// Get the plit infos deque.
    fn split_infos_vec(&mut self) -> &mut Vec<Option<NonNull<SplitInfo>>>;
}

pub trait SplitPaneExt {
    /// Split new widget left.
    ///
    /// @param: id the id of target widget where user click on. <br>
    /// @param widget the new widget that split off. <br>
    ///
    /// @reutrn success or not.
    fn split_left<T: WidgetImpl>(&mut self, id: u16, widget: Box<T>);

    /// Split new widget up.
    ///
    /// @param: id the id of target widget where user click on. <br>
    /// @param widget the new widget that split off. <br>
    ///
    /// @reutrn success or not.
    fn split_up<T: WidgetImpl>(&mut self, id: u16, widget: Box<T>);

    /// Split new widget right.
    ///
    /// @param: id the id of target widget where user click on. <br>
    /// @param widget the new widget that split off. <br>
    ///
    /// @reutrn success or not.
    fn split_right<T: WidgetImpl>(&mut self, id: u16, widget: Box<T>);

    /// Split new widget down.
    ///
    /// @param: id the id of target widget where user click on. <br>
    /// @param widget the new widget that split off. <br>
    ///
    /// @reutrn success or not.
    fn split_down<T: WidgetImpl>(&mut self, id: u16, widget: Box<T>);

    /// Close the split pane, the widgets were splited from this pane will be closed automatically.
    ///
    /// @param: id the id of target widget where user click on. <br>
    fn close_pane(&mut self, id: u16);

    /// Common function of split().
    fn split<T: WidgetImpl>(&mut self, id: u16, widget: Box<T>, ty: SplitType);
}

split_pane_impl!(SplitPane);

pub struct SplitInfo {
    /// The id of self widget.
    pub id: u16,
    /// The self widget ptr which the `SplitInfo` binded.
    pub widget: Option<NonNull<dyn WidgetImpl>>,
    /// The ptr of `SplitInfo` which this widget splited from.
    pub split_from: Option<NonNull<SplitInfo>>,
    /// The vector of `SplitInfo` ptr which was splited from this widget.
    pub split_to: Vec<Option<NonNull<SplitInfo>>>,
    /// The split type.
    pub ty: SplitType,
}

impl SplitInfo {
    #[inline]
    pub fn new(
        id: u16,
        widget: Option<NonNull<dyn WidgetImpl>>,
        split_from: Option<NonNull<SplitInfo>>,
        ty: SplitType,
    ) -> Self {
        Self {
            id,
            widget,
            split_from,
            split_to: vec![],
            ty,
        }
    }

    pub(crate) fn calculate_layout(&mut self, parent_rect: Rect) {
        let widget = split_widget!(self);

        match self.ty {
            SplitType::SplitNone => {
                widget.width_request(parent_rect.width());
                widget.height_request(parent_rect.height());
                widget.set_fixed_x(parent_rect.x());
                widget.set_fixed_y(parent_rect.y());
            }

            SplitType::SplitLeft => {
                let split_from = split_from!(self);
                let split_from_widget = split_widget!(split_from);
                let from_rect = split_from_widget.rect();
                let new_width = from_rect.width() / 2;

                widget.width_request(new_width);
                widget.height_request(from_rect.height());
                widget.set_fixed_x(from_rect.x());
                widget.set_fixed_y(from_rect.y());

                split_from_widget.width_request(new_width);
                split_from_widget.height_request(from_rect.height());
                split_from_widget.set_fixed_x(from_rect.x() + new_width);
                split_from_widget.set_fixed_y(from_rect.y());
            }

            SplitType::SplitUp => {
                let split_from = split_from!(self);
                let split_from_widget = split_widget!(split_from);
                let from_rect = split_from_widget.rect();
                let new_height = from_rect.height() / 2;

                widget.width_request(from_rect.width());
                widget.height_request(new_height);
                widget.set_fixed_x(from_rect.x());
                widget.set_fixed_y(from_rect.y());

                split_from_widget.width_request(from_rect.width());
                split_from_widget.height_request(new_height);
                split_from_widget.set_fixed_x(from_rect.x());
                split_from_widget.set_fixed_y(from_rect.y() + new_height);
            }

            SplitType::SplitRight => {
                let split_from = split_from!(self);
                let split_from_widget = split_widget!(split_from);
                let from_rect = split_from_widget.rect();
                let new_width = from_rect.width() / 2;

                split_from_widget.width_request(new_width);
                split_from_widget.height_request(from_rect.height());
                split_from_widget.set_fixed_x(from_rect.x());
                split_from_widget.set_fixed_y(from_rect.y());

                widget.width_request(new_width);
                widget.height_request(from_rect.height());
                widget.set_fixed_x(from_rect.x() + new_width);
                widget.set_fixed_y(from_rect.y());
            }

            SplitType::SplitDown => {
                let split_from = split_from!(self);
                let split_from_widget = split_widget!(split_from);
                let from_rect = split_from_widget.rect();
                let new_height = from_rect.height() / 2;

                split_from_widget.width_request(from_rect.width());
                split_from_widget.height_request(new_height);
                split_from_widget.set_fixed_x(from_rect.x());
                split_from_widget.set_fixed_y(from_rect.y());

                widget.width_request(from_rect.width());
                widget.height_request(new_height);
                widget.set_fixed_x(from_rect.x());
                widget.set_fixed_y(from_rect.y() + new_height);
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////
/// Enums
///////////////////////////////////////////////////////////////////////////////////////////
#[repr(u8)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum SplitType {
    #[default]
    SplitNone = 0,
    SplitLeft,
    SplitUp,
    SplitRight,
    SplitDown,
}
impl AsNumeric<u8> for SplitType {
    fn as_numeric(&self) -> u8 {
        match self {
            Self::SplitNone => 0,
            Self::SplitLeft => 1,
            Self::SplitUp => 2,
            Self::SplitRight => 3,
            Self::SplitDown => 4,
        }
    }
}
impl From<u8> for SplitType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::SplitNone,
            1 => Self::SplitLeft,
            2 => Self::SplitUp,
            3 => Self::SplitRight,
            4 => Self::SplitDown,
            _ => unimplemented!(),
        }
    }
}
implements_enum_value!(SplitType, u8);
