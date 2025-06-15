use tlib::{run_after, win_widget};
use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[derive(Clone, Copy, Debug)]
pub enum CrsWinMsg {
    Test(i32, i32),
}

#[extends(Widget, Layout(HBox))]
#[win_widget(o2s = "CrsWinMsg", s2o(CrsWinMsg))]
#[run_after]
pub struct MyWinWidget {}

impl CrossWinMsgHandler for MyWinWidget {
    type T = CrsWinMsg;

    fn handle(&mut self, msg: Self::T) {
        println!(
            "[{}] Receive cross window msg {:?}",
            std::thread::current().name().unwrap(),
            msg
        );
        match msg {
            CrsWinMsg::Test(a, b) => {
                assert_eq!(a, 122);
                assert_eq!(b, 290);
            }
        }
    }
}

impl CrossWinMsgHandler for CorrMyWinWidget {
    type T = CrsWinMsg;

    fn handle(&mut self, msg: Self::T) {
        println!(
            "[{}] Receive cross window msg {:?}",
            std::thread::current().name().unwrap(),
            msg
        );
    }
}

impl ObjectSubclass for MyWinWidget {
    const NAME: &'static str = "MyWinWidget";
}

impl ObjectImpl for MyWinWidget {
    fn initialize(&mut self) {
        self.set_hexpand(true);
        self.set_vexpand(true);

        self.set_background(Color::BLUE);
    }
}

impl WidgetImpl for MyWinWidget {
    #[inline]
    fn run_after(&mut self) {
        self.send_cross_win_msg(CrsWinMsg::Test(100, 100));
    }
}
