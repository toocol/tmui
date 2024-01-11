use crate::{graphics::painter::Painter, primitive::frame::Frame, widget::WidgetImpl};
use std::{cell::RefCell, ptr::NonNull};
use tlib::{
    figure::{Color, FRect, Rect},
    nonnull_mut,
    prelude::*, skia_safe::{ClipOp, PaintStyle},
};

pub const LOADING_SIZE: f32 = 14.;
pub const LOADING_COLOR: Color = Color::GREY.with_a(160);
thread_local! {
    static INSTANCE: RefCell<LoadingManager> = RefCell::new(LoadingManager::new());
}

#[derive(Debug, Default, PartialEq, Eq)]
enum State {
    #[default]
    Stopped,
    Playing,
}

#[derive(Derivative)]
#[derivative(Default)]
pub struct LoadingModel {
    state: State,
    #[derivative(Default(value = "Box::new(DefaultLoading(0.))"))]
    loading: Box<dyn Loading>,
}

impl LoadingModel {
    #[inline]
    pub fn new(loading: Box<dyn Loading>) -> Self {
        Self {
            state: State::Stopped,
            loading,
        }
    }
}

#[reflect_trait]
pub trait Loadable: WidgetImpl {
    fn loading_model(&self) -> &LoadingModel;

    fn loading_model_mut(&mut self) -> &mut LoadingModel;

    #[inline]
    fn set_loading(&mut self, loading: Box<dyn Loading>) {
        self.loading_model_mut().loading = loading;
    }

    #[inline]
    fn start_loading(&mut self) {
        self.loading_model_mut().state = State::Playing;
        self.update();
    }

    #[inline]
    fn stop_loading(&mut self) {
        self.loading_model_mut().state = State::Stopped;
        self.update();
    }

    #[inline]
    fn is_loading(&self) -> bool {
        self.loading_model().state == State::Playing
    }

    fn loading_snapshot(&mut self, frame: Frame) {
        let model = self.loading_model_mut();
        if model.state == State::Stopped {
            return;
        }

        model.loading.loading_snapshot(frame);

        self.update()
    }

    fn render_loading(&self, painter: &mut Painter) {
        let mut rect = self.rect();
        rect.set_point(&self.map_to_widget(&rect.top_left()));

        self.loading_model().loading.render_loading(painter, rect, self.background())
    }
}

pub trait Loading {
    fn loading_snapshot(&mut self, frame: Frame);

    fn render_loading(&self, painter: &mut Painter, rect: Rect, background: Color);
}

pub struct DefaultLoading(f32);
impl Loading for DefaultLoading {
    #[inline]
    fn loading_snapshot(&mut self, _frame: Frame) {
        self.0 += 0.01;
        if self.0 > 1. {
            self.0 = 0.
        }
    }

    fn render_loading(&self, painter: &mut Painter, rect: Rect, background: Color) {
        painter.save();
        painter.clip_rect(rect, ClipOp::Intersect);
        painter.fill_rect(rect, background);

        let rect: FRect = rect.into();

        let (x, y) = (
            rect.x() + rect.width() / 2. - LOADING_SIZE / 2.,
            rect.y() + rect.height() / 2. - LOADING_SIZE / 2.,
        );

        painter.set_line_width(4.);
        painter.set_antialiasing(true);
        painter.set_color(LOADING_COLOR);

        let angle = 360. * self.0;
        painter.set_style(PaintStyle::Stroke);
        painter.draw_arc_f(x, y, LOADING_SIZE, LOADING_SIZE, angle, 240., false);
        painter.restore();
    }
}

pub struct LoadingManager {
    loadings: Vec<Option<NonNull<dyn Loadable>>>,
}

impl LoadingManager {
    #[inline]
    fn new() -> Self {
        Self { loadings: vec![] }
    }

    #[inline]
    pub(crate) fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&RefCell<Self>) -> R,
    {
        INSTANCE.with(f)
    }

    #[inline]
    pub(crate) fn add_loading(&mut self, loading: &mut dyn Loadable) {
        self.loadings.push(NonNull::new(loading))
    }

    #[inline]
    pub(crate) fn process(&mut self, frame: Frame) {
        for loadings in self.loadings.iter_mut() {
            nonnull_mut!(loadings).loading_snapshot(frame);
        }
    }
}
