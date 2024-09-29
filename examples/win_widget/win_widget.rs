use tlib::win_widget;
use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[derive(Clone, Copy)]
pub enum CrsWinMsg {
    Test(i32, i32),
}

#[extends(Widget)]
#[win_widget(CrsWinMsg)]
pub struct MyWinWidget {}

impl CrossWinMsgHandler for MyWinWidget {
    type T = CrsWinMsg;

    fn handle(&mut self, msg: Self::T) {
        println!("Receive cross window msg {:?}", std::thread::current().name());
        match msg {
            CrsWinMsg::Test(a, b) => {
                assert_eq!(a, 122);
                assert_eq!(b, 290);
            }
        }
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

impl WidgetImpl for MyWinWidget {}

impl MyWinWidget {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
