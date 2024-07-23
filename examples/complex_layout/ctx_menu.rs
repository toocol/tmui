use tlib::global_watch;
use tmui::{
    graphics::box_shadow::{BoxShadow, ShadowSide},
    prelude::*,
    scroll_area::LayoutMode,
    tlib::object::{ObjectImpl, ObjectSubclass},
    views::{
        cell::{cell_render::TextCellRender, Cell},
        list_view::{list_view_object::ListViewObject, ListView},
        node::node_render::NodeRender,
    },
    widget::WidgetImpl,
};

#[extends(Popup)]
#[global_watch(MouseReleased)]
#[derive(Childable)]
pub struct CtxMenu {
    #[child]
    list: Box<ListView>,
}

impl ObjectSubclass for CtxMenu {
    const NAME: &'static str = "CtxMenu";
}

impl ObjectImpl for CtxMenu {
    fn initialize(&mut self) {
        self.width_request(200);
        self.height_request(400);

        self.set_borders(1., 1., 1., 1.);
        self.set_border_color(Color::GREY_LIGHT);
        self.set_box_shadow(BoxShadow::new(
            8.,
            Color::BLACK,
            None,
            Some(ShadowSide::new(&[ShadowSide::RIGHT, ShadowSide::BOTTOM])),
            None,
        ));

        self.list.set_vexpand(true);
        self.list.set_hexpand(true);
        self.list.set_layout_mode(LayoutMode::Overlay);
        self.list.set_mouse_tracking(true);

        let scroll_bar = self.list.scroll_bar_mut();
        scroll_bar.set_slider_radius(5.);
        scroll_bar.set_background(Color::TRANSPARENT);
        scroll_bar.set_color(Color::GREY_LIGHT.with_a(155));
        scroll_bar.set_active_color(Some(Color::GREY_MEDIUM.with_a(155)));
        scroll_bar.set_visible_in_valid(true);
        self.list.add_node(&Selection {
            val: "Selection".to_string(),
        });

        self.list.register_node_pressed(|node, _, _| {
            let _ = node.get_view();
        })
    }
}

impl WidgetImpl for CtxMenu {}

impl PopupImpl for CtxMenu {
    #[inline]
    fn calculate_position(&self, _: Rect, point: Point) -> Point {
        point
    }

    #[inline]
    fn is_modal(&self) -> bool {
        true
    }
}

impl GlobalWatchImpl for CtxMenu {
    fn on_global_mouse_released(&mut self, evt: &tlib::events::MouseEvent) -> bool {
        if !self.visible() {
            return false;
        }
        let pos: Point = evt.position().into();
        if !self.rect().contains(&pos) {
            self.hide();
        }

        true
    }
}

impl CtxMenu {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}

struct Selection {
    val: String,
}
impl ListViewObject for Selection {
    #[inline]
    fn cells(&self) -> Vec<Cell> {
        vec![Cell::string()
            .value(self.val.clone())
            .cell_render(TextCellRender::builder().color(Color::BLACK).build())
            .build()]
    }

    #[inline]
    fn node_render(&self) -> NodeRender {
        NodeRender::builder().build()
    }
}
