use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
    style::Style,
};
use crate::app_state::{
    AppState,
    Tile
};

pub struct MyWidget<'a> {
    pub state: &'a AppState,
}

impl<'a> Widget for MyWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for i in 0..self.state.w {
            for j in 0..self.state.h {
                let t = match self.state.get(i,j) {
                    Tile::Wall => "#",
                    Tile::Crate => "o",
                    _ => " ",
                };
                buf.set_string(area.left() + i, area.top() + j, t, Style::default());
            }
        }
        buf.set_string(area.left() + self.state.w, area.top() + self.state.h, "X", Style::default());
    }
}

impl<'a> MyWidget<'a> {
    pub fn state(mut self, state: &'a AppState) -> MyWidget<'a> {
        self.state = state;
        self
    }
}

