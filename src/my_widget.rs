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
        // print!("rendering in area: {}x{} {}x{}", area.x, area.y,
        //        area.width, area.height);
        let offset_x = self.state.view_center_x - (area.width as i16) / 2;
        let offset_y = self.state.view_center_y - (area.height as i16) / 2;
        for i in 0..area.width {
            for j in 0..area.height {
                let ix = (i as i16) + offset_x;
                let jy = (j as i16) + offset_y;
                let glyph = if ix < 0 || jy < 0 || ix >= self.state.w as i16 || jy >= self.state.h as i16 {
                    "?"
                } else {
                    match self.state.get(ix, jy) {
                        Tile::Wall => "#",
                        Tile::Crate => "o",
                        _ => " ",
                    }
                };
                buf.set_string(area.left() + i, area.top() + j, glyph, Style::default());
            }
        }
    }
}

impl<'a> MyWidget<'a> {
    pub fn state(mut self, state: &'a AppState) -> MyWidget<'a> {
        self.state = state;
        self
    }
}

