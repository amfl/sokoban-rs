#[allow(dead_code)]
mod util;
mod my_widget;
mod app_state;

use crate::util::{Event, Events};
use crate::my_widget::MyWidget;
use crate::app_state::AppState;
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    // style::{Color},
    widgets::{
        // canvas::{Canvas},
        Block, BorderType, Borders
    },
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = AppState::new(64,64);
    state.load_level("maps.json", 0);

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Setup event handlers
    let events = Events::new();

    loop {
        terminal.draw(|mut f| {
            // Wrapping block for a group
            // Just draw the block and the group on the same area and build the group
            // with at least a margin of 1
            let size = f.size();
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Main block with round corners")
                .border_type(BorderType::Rounded);
            f.render_widget(block, size);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());
            // {
            //     let block = Block::default()
            //         .title("With background")
            //         .title_style(Style::default().fg(Color::Yellow))
            //         .style(Style::default().bg(Color::Green));
            //     // Render this widget twice
            //     f.render_widget(block, chunks[0]);
            // }
            {
                let myw = MyWidget{ state: &state };
                f.render_widget(myw, chunks[0]);
            }
            // {
            //     let canvas = Canvas::default()
            //         .block(Block::default().borders(Borders::ALL).title("World"))
            //         .paint(|ctx| {
            //             ctx.print(0.0, 0.0, "You are here", Color::Yellow);
            //         })
            //         .x_bounds([0.0, 20.0])
            //         .y_bounds([0.0, 20.0]);
            //     f.render_widget(canvas, chunks[1]);
            // }
        })?;
        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Down  => { state.view_center_y += 1; }
                Key::Up    => { state.view_center_y -= 1; }
                Key::Right => { state.view_center_x += 1; }
                Key::Left  => { state.view_center_x -= 1; }
                Key::Char('s') => { state.player_move(0, 1); }
                Key::Char('w') => { state.player_move(0, -1); }
                Key::Char('d') => { state.player_move(1, 0); }
                Key::Char('a') => { state.player_move(-1, 0); }
                _ => {}
            },
            Event::Tick => {
                // state.update();
            }
        }

    }
    Ok(())
}
