use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
    style::{Style, Color},
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
                let tile = if ix < 0 || jy < 0 || ix >= self.state.w as i16 || jy >= self.state.h as i16 {
                    (" ", Style::default())
                } else {
                    match self.state.get(ix, jy) {
                        Tile::Empty => (" ", Style::default()),
                        Tile::Wall => ("#", Style::default().bg(Color::Gray)),
                        Tile::Floor => (" ", Style::default()),
                        Tile::Target => (".", Style::default().fg(Color::Green)),
                        Tile::Crate => ("X", Style::default().bg(Color::Magenta)),
                        Tile::CrateOnTarget => ("O", Style::default().bg(Color::Blue)),
                        Tile::Player => ("p", Style::default()),
                        Tile::PlayerOnTarget => ("p", Style::default()),
                        _ => (" ", Style::default()),
                    }
                };
                let (glyph, style) = tile;
                buf.set_string(area.left() + i, area.top() + j, glyph, style);
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
                "@", Style::default().fg(Color::Yellow));
        }
    }
}

impl<'a> MyWidget<'a> {
    pub fn state(mut self, state: &'a AppState) -> MyWidget<'a> {
        self.state = state;
        self
    }
}

