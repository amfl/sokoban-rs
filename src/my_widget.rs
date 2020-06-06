use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
    style::Style,
};
use crate::app_state::AppState;

pub struct MyWidget<'a> {
    pub state: &'a AppState,
}

impl<'a> Widget for MyWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.left() + self.state.x, area.top() + self.state.y, "X", Style::default());
    }
}

impl<'a> MyWidget<'a> {
    pub fn state(mut self, state: &'a AppState) -> MyWidget<'a> {
        self.state = state;
        self
    }
}

