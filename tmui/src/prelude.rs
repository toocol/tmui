pub use std::cell::Ref;
pub use tlib::prelude::*;

pub type RawWindowHandle5 = raw_window_handle::RawWindowHandle;
pub type RawWindowHandle6 = tlib::winit::raw_window_handle::RawWindowHandle;

pub use tlib::{nonnull_mut, nonnull_ref, cast, cast_mut, cast_boxed, connect, disconnect};
pub use crate::{split_widget, split_from};

pub use crate::animation::{
    frame_animator::ReflectFrameAnimator,
    inner::AnimationsHolder,
    snapshot::{ReflectSnapshot, Snapshot},
    state_holder::{RectHolder, ReflectRectHolder, ReflectTransparencyHolder, TransparencyHolder},
    Animatable, Animation, AnimationModel, ReflectAnimatable, {self},
};
pub use crate::application_window::{window_id, ApplicationWindow};
pub use crate::container::{
    ChildrenRegionAcquirer, Container, ContainerAcquire, ContainerExt, ContainerImpl,
    ContainerImplExt, ContainerPointEffective, ContainerPropsAcquire, ContainerScaleCalculate,
    ReflectContainerImpl, ReflectSizeUnifiedAdjust, ReflectSpacingCapable, SizeUnifiedAdjust,
    SpacingCapable, StaticContainerScaleCalculate, StaticSizeUnifiedAdjust, ContainerLayoutEnum
};
pub use crate::font::Font;
pub use crate::graphics::board::Board;
pub use crate::graphics::border::Border;
pub use crate::graphics::drawing_context::DrawingContext;
pub use crate::graphics::element::{
    Element, ElementAcquire, ElementExt, ElementImpl, ElementPropsAcquire, ElementSignals,
    ReflectElementImpl,
};
pub use crate::graphics::painter::Painter;
pub use crate::graphics::render_difference::{
    ChildContainerDiffRender, ReflectChildContainerDiffRender,
};
pub use crate::hbox::HBox;
pub use crate::label::LabelSignal;
pub use crate::layout::{
    Composition, ContainerLayout, ContentAlignment, Layout, ReflectContentAlignment,
};
pub use crate::loading::{Loadable, LoadingModel, ReflectLoadable};
pub use crate::overlay::{Overlaid, ReflectOverlaid};
pub use crate::pane::{Pane, PaneExt, ReflectPaneExt};
pub use crate::popup::{Popup, PopupExt, PopupImpl, Popupable, ReflectPopupImpl, ReflectPopupable};
pub use crate::primitive::close_handler::{
    CloseHandler, CloseHandlerMgr, CloseHandlerRequire, ReflectCloseHandler,
};
pub use crate::primitive::global_watch::{
    GlobalWatch, GlobalWatchEvent, GlobalWatchImpl, ReflectGlobalWatch,
};
pub use crate::scroll_area::{ReflectScrollAreaExt, ScrollAreaExt, ScrollAreaGenericExt, LayoutMode, ScrollAreaSlots, ScrollArea};
pub use crate::scroll_bar::{ScrollBarSignal, ScrollBar, ScrollBarPosition};
pub use crate::shared_widget::{
    ReflectSharedWidgetImpl, SharedWidget, SharedWidgetExt, SharedWidgetImpl,
};
pub use crate::shortcut::Shortcut;
pub use crate::split_pane::{
    ReflectSplitInfosGetter, SplitInfo, SplitInfosGetter, SplitPane, SplitPaneExt, SplitType,
};
pub use crate::stack::{ReflectStackImpl, Stack, StackImpl};
pub use crate::vbox::VBox;
pub use crate::widget::{
    callbacks::Callbacks,
    widget_ext::WidgetExt,
    win_widget::{
        CrossWinMsgHandler, CrossWinMsgHandlerInner, CrossWinMsgHandlerRequire, CrossWinMsgSender,
        ReflectCrossWinMsgHandlerInner, ReflectWinWidget, WinWidget, ReflectCrossWinWidget, CrossWinWidget
    },
    ChildOp, ChildRegionAcquire, EventBubble, InnerCustomizeEventProcess, IsolatedVisibility,
    PointEffective, ReflectInnerCustomizeEventProcess, ReflectIsolatedVisibility,
    ReflectIterExecutor, ReflectWidgetImpl, Transparency, Widget, WidgetAcquire, WidgetHnd,
    WidgetImpl, WidgetPropsAcquire, WidgetSignals, WindowAcquire, WidgetGenericExt, InnerEventProcess
};
pub use tlib::tokio;
pub use tlib::{
    figure::{Color, FPoint, FRect, FRegion, Point, Rect, Region, Size, SizeHint},
    {self},
};
