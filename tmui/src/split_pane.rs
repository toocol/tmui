use crate::{
    prelude::*,
    tlib::{
        object::{ObjectImpl, ObjectSubclass},
        values::{FromBytes, FromValue, ToBytes, ToValue},
    },
    widget::WidgetImpl,
};
use std::{collections::HashMap, mem::size_of};
use tlib::{implements_enum_value, namespace::AsNumeric};

#[extends(Container)]
#[derive(Default)]
pub struct SplitPane {
    split_state: SplitState,
    locations: HashMap<u16, ViewLocation>,
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
    /// Change the split state of SplitPane.
    fn change_state(&mut self, state: SplitState);

    /// Split new widget left.
    ///
    /// @param: id the id of target widget where user click on. <br>
    /// @param widget the new widget that split off. <br>
    ///
    /// @reutrn success or not.
    fn split_left<T: WidgetImpl>(&mut self, id: u16, widget: T) -> bool;

    /// Split new widget up.
    ///
    /// @param: id the id of target widget where user click on. <br>
    /// @param widget the new widget that split off. <br>
    ///
    /// @reutrn success or not.
    fn split_up<T: WidgetImpl>(&mut self, id: u16, widget: T);

    /// Split new widget right.
    ///
    /// @param: id the id of target widget where user click on. <br>
    /// @param widget the new widget that split off. <br>
    ///
    /// @reutrn success or not.
    fn split_right<T: WidgetImpl>(&mut self, id: u16, widget: T);

    /// Split new widget down.
    ///
    /// @param: id the id of target widget where user click on. <br>
    /// @param widget the new widget that split off. <br>
    ///
    /// @reutrn success or not.
    fn split_down<T: WidgetImpl>(&mut self, id: u16, widget: T);
}

impl SplitPaneExt for SplitPane {
    fn change_state(&mut self, state: SplitState) {
        todo!()
    }

    fn split_left<T: WidgetImpl>(&mut self, id: u16, widget: T) -> bool {
        let view_location = *self.locations.get(&id).unwrap();
        if !view_location.check_split_left(&mut self.locations) {
            return false;
        }
        true
    }

    fn split_up<T: WidgetImpl>(&mut self, id: u16, widget: T) {
        todo!()
    }

    fn split_right<T: WidgetImpl>(&mut self, id: u16, widget: T) {
        todo!()
    }

    fn split_down<T: WidgetImpl>(&mut self, id: u16, widget: T) {
        todo!()
    }
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

    ThreeRight,
    ThreeLeftTop,
    ThreeLeftBottom,

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
            Self::ThreeRight => 6,
            Self::ThreeLeftTop => 7,
            Self::ThreeLeftBottom => 8,
            Self::FourLeftTop => 9,
            Self::FourLeftBottom => 10,
            Self::FourRightTop => 11,
            Self::FourRightBottom => 12,
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
            6 => Self::ThreeRight,
            7 => Self::ThreeLeftTop,
            8 => Self::ThreeLeftBottom,
            9 => Self::FourLeftTop,
            10 => Self::FourLeftBottom,
            11 => Self::FourRightTop,
            12 => Self::FourRightBottom,
            _ => unimplemented!(),
        }
    }
}
implements_enum_value!(ViewLocation, u8);
impl ViewLocation {
    /// Check the widget can split left or not.
    #[inline]
    pub fn check_split_left(self, view_locations: &mut HashMap<u16, ViewLocation>) -> bool {
        match self {
            Self::OneCenter => true,
            Self::TwoRight => true,
            Self::ThreeRightTop => true,
            Self::ThreeRightBottom => true,
            _ => false,
        }
    }

    /// Check the widget can split up or not.
    #[inline]
    pub fn check_split_up(self) -> bool {
        match self {
            Self::OneCenter => true,
            Self::TwoLeft => true,
            Self::TwoRight => true,
            Self::ThreeLeft => true,
            Self::ThreeRight => true,
            _ => false,
        }
    }

    /// Check the widget can split right or not.
    #[inline]
    pub fn check_split_right(self) -> bool {
        match self {
            Self::OneCenter => true,
            Self::TwoLeft => true,
            Self::ThreeLeftTop => true,
            Self::ThreeLeftBottom => true,
            _ => false,
        }
    }

    /// Check the widget can split down or not.
    #[inline]
    pub fn check_split_down(self) -> bool {
        match self {
            Self::OneCenter => true,
            Self::TwoLeft => true,
            Self::TwoRight => true,
            Self::ThreeLeft => true,
            Self::ThreeRight => true,
            _ => false,
        }
    }
}
