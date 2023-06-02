use crate::{
    prelude::*,
    tlib::{
        object::{ObjectImpl, ObjectSubclass},
        values::{FromBytes, FromValue, ToBytes, ToValue},
    },
    widget::WidgetImpl,
};
use std::{mem::size_of, collections::HashMap};
use tlib::{implements_enum_value, namespace::AsNumeric};

#[extends(Container)]
#[derive(Default)]
pub struct SplitPane {
    split_state: SplitState,
    locations: HashMap<ViewLocation, u16>,
}

impl ObjectSubclass for SplitPane {
    const NAME: &'static str = "SplitPane";
}

impl ObjectImpl for SplitPane {}

impl WidgetImpl for SplitPane {}

impl ContainerImpl for SplitPane {
    fn children(&self) -> Vec<&dyn WidgetImpl> {
        self.children.iter().map(|c| c.as_ref()).collect()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
        self.children.iter_mut().map(|c| c.as_mut()).collect()
    }
}

impl ContainerImplExt for SplitPane {
    fn add_child<T>(&mut self, child: T)
    where
        T: WidgetImpl + IsA<Widget>,
    {
        self.children.push(Box::new(child))
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
        todo!()
    }

    fn container_position_layout<T: WidgetImpl + ContainerImpl>(
        widget: &mut T,
        previous: &dyn WidgetImpl,
        parent: &dyn WidgetImpl,
        manage_by_container: bool,
    ) {
        todo!()
    }
}

pub trait SplitPaneExt {
    fn change_state(&mut self, state: SplitState);
}

///////////////////////////////////////////////////////////////////////////////////////////
/// Enums
///////////////////////////////////////////////////////////////////////////////////////////
#[repr(u8)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum SplitState {
    #[default]
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 4,
    Four = 8,
}
impl AsNumeric<u8> for SplitState {
    fn as_numeric(&self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 4,
            Self::Four => 8,
        }
    }
}
impl From<u8> for SplitState {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            4 => Self::Three,
            8 => Self::Four,
            _ => unimplemented!(),
        }
    }
}
implements_enum_value!(SplitState, u8);

#[repr(u8)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ViewLocation {
    #[default]
    OneCenter = 0,

    // Split screen for two
    TwoLeft,
    TwoRight,

    // Split screen for three
    ThreeLeft,
    ThreeRightTop,
    ThreeRightBottom,

    // Split screen for four
    FourLeftTop,
    FourLeftBottom,
    FourRightTop,
    FourRightBottom,
}
impl AsNumeric<u8> for ViewLocation {
    fn as_numeric(&self) -> u8 {
        match self {
            Self::OneCenter => 0,
            Self::TwoLeft => 1,
            Self::TwoRight => 2,
            Self::ThreeLeft => 3,
            Self::ThreeRightTop => 4,
            Self::ThreeRightBottom => 5,
            Self::FourLeftTop => 6,
            Self::FourLeftBottom => 7,
            Self::FourRightTop => 8,
            Self::FourRightBottom => 9,
        }
    }
}
impl From<u8> for ViewLocation {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::OneCenter,
            1 => Self::TwoLeft,
            2 => Self::TwoRight,
            3 => Self::ThreeLeft,
            4 => Self::ThreeRightTop,
            5 => Self::ThreeRightBottom,
            6 => Self::FourLeftTop,
            7 => Self::FourLeftBottom,
            8 => Self::FourRightTop,
            9 => Self::FourRightBottom,
            _ => unimplemented!(),
        }
    }
}
implements_enum_value!(ViewLocation, u8);
