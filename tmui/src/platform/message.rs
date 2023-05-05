use tlib::prelude::SystemCursorShape;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Message {
    /// VSync signal to redraw the window
    VSync,

    /// Set the cursor shape by user.
    SetCursorShape(SystemCursorShape),
}