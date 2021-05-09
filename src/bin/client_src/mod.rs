use crate::json_structs::ClientConfig;
use std::{io, io::Write, time::Duration};
use termion::{event::Key};
use tui::{Terminal,backend::Backend};
use super::*;
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    backend::TermionBackend,
};

pub mod networking;
#[cfg(unix)]
mod event;
mod ui;

pub fn run_client<B>(
    client_config: &crate::json_structs::ClientConfig,
    terminal : &mut Terminal<B>,
) -> Result<(), Box<dyn std::error::Error>> 
where
    B: Backend
{   
    let mut events = event::Events::with_config(event::Config {
        exit_key: Key::Char(client_config.key_map.quit),
        tick_rate: Duration::from_millis(250),
    });
    events.enable_exit_key();
    terminal.clear()?;

    loop {
        match update_client(&mut events, client_config) {
            Ok(should_contiue) => {
                if should_contiue {
                    draw_client(terminal)?;
                } else {
                    return Ok(());
                }
            }
            Err(e) => return Err(e),
        }
    }
}

fn update_client(
    events: &mut event::Events,
    client_config: &ClientConfig,
) -> Result<bool, Box<dyn std::error::Error>>
{
    if let event::Event::Input(key) = events.next()? {
        if key == Key::Char(client_config.key_map.quit) {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn draw_client<B>(
    terminal: &mut tui::Terminal<B>,
) -> Result<(), Box<dyn std::error::Error>>
where
    B: Backend,
{
    terminal.draw(|frame| {
        let size = frame.size();
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        frame.render_widget(block, size);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(100), Constraint::Percentage(100)].as_ref())
            .split(frame.size());

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(chunks[0]);
        let block = Block::default().title(vec![Span::from("With background")]);
        frame.render_widget(block, top_chunks[0]);

        let block = Block::default().title(Span::styled(
            "Styled title",
            Style::default()
                .fg(Color::White)
                .bg(Color::Red)
                .add_modifier(Modifier::BOLD),
        ));
        frame.render_widget(block, top_chunks[1]);
    })?;
    Ok(())
}
