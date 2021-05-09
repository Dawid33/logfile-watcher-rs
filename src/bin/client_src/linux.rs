use std::io::Write;
use termion::input::TermRead;
use std::{error::Error, io};
use crate::json_structs::ClientConfig;
use common;
use super::*;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Terminal,
};

pub fn run_client(client_config : &crate::json_structs::ClientConfig) -> Result<(), Box<dyn std::error::Error>>{
    let stdout = io::stdout().into_raw_mode()?;
    let termion_backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(termion_backend)?;
    terminal.clear()?;
    let mut draw_config = common::DrawConfig::default();
    let mut events = event::Events::new();
    events.enable_exit_key();
    loop {
        match update_client(&mut terminal, &mut events, &mut draw_config, client_config){
            Ok(should_contiue) => {
                if should_contiue {
                    draw_client(&mut terminal, &draw_config)?;
                } else {
                    return Ok(())
                }
            },
            Err(e) => return Err(e)
        }
    }
}

fn update_client<W>(
    _terminal: &mut tui::Terminal<tui::backend::TermionBackend<W>>,
    events : &mut event::Events,
    _draw_config : &mut common::DrawConfig,
    client_config : &ClientConfig,
) -> Result<bool, Box<dyn std::error::Error>>
where
    W: Write,
{
    if let event::Event::Input(key) = events.next()? {
        if key == Key::Char(client_config.key_map.quit) {
            return Ok(false);
        }
    }
    Ok(true)
}

fn draw_client<W>(
    terminal: &mut tui::Terminal<tui::backend::TermionBackend<W>>, 
    draw_config : &common::DrawConfig,
) -> Result<(), Box<dyn std::error::Error>> 
where
    W: Write,
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
            .constraints([Constraint::Percentage(100),Constraint::Percentage(100)].as_ref())
            .split(frame.size());
        
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(draw_config.sidebar_percentage_size), Constraint::Percentage(draw_config.main_panel_percentage_size)].as_ref())
            .split(chunks[0]);
        let block = Block::default()
            .title(vec![
                Span::from("With background"),
            ]);
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