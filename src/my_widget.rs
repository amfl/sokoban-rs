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
                    "."
                } else {
                    match self.state.get(ix, jy) {
                        Tile::Empty => " ",
                        Tile::Wall => "#",
                        Tile::Floor => " ",
                        Tile::Target => "x",
                        Tile::Crate => "o",
                        Tile::CrateOnTarget => "O",
                        Tile::Player => "p",
                        Tile::PlayerOnTarget => "p",
                        _ => " ",
                    }
                };
                buf.set_string(area.left() + i, area.top() + j, glyph, Style::default());
            }
        }

        // Draw the player
        let proj_x = self.state.player_x - offset_x;
        let proj_y = self.state.player_y - offset_y;
        if proj_x >= 0 || proj_x < self.state.w ||
                proj_y >= 0 || proj_y < self.state.h {
            buf.set_string(
                area.left() + (proj_x as u16),
                area.top() + (proj_y as u16),
                "@", Style::default());
        }
    }
}

impl<'a> MyWidget<'a> {
    pub fn state(mut self, state: &'a AppState) -> MyWidget<'a> {
        self.state = state;
        self
    }
}

