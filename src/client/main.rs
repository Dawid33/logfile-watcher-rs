#![allow(dead_code)]
#![allow(unused_imports)]
extern crate common;
use common::json_structs::ClientConfig;

use std::{io, io::Write, time::Duration, path::Path,sync::mpsc};
#[cfg(unix)]
use termion::{event::Key};

use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span,Spans},
    buffer::{Buffer,Cell},
    widgets::{Block, BorderType, Borders,Paragraph},
    backend::Backend,
    Terminal,
};

#[cfg(unix)]
use tui::backend::TermionBackend;
#[cfg(unix)]
use termion::raw::IntoRawMode;
#[cfg(windows)]
use tui::backend::CrosstermBackend;

pub mod networking;
#[cfg(unix)]
mod event;
mod ui;
use ui::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Configuration file for the client.
    let path = Path::new("client_config.json");
    let config = common::load_struct::<ClientConfig>(path);

    //Run code for tui.
    if cfg!(unix) {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        run_client(&config, &mut terminal)?;
    } else if cfg!(windows) {

    }
    
    //Communication between threads.
    //let (rx, _tx) = mpsc::channel::<tungstenite::error::Error>();
    //networking::connect(config.url.clone(), rx.clone());
    
    // If nothing is printed before exiting,the empty space
    // following the shell prompt is black for some reason
    // (in visual studio). So just print a new line :)
    println!("");
    Ok(())
}

pub fn run_client<B>(
    client_config: &common::json_structs::ClientConfig,
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
    let mut draw_config = DrawConfig::default().set_client_config(client_config.ui_config.clone());

    loop {
        match update_client(&mut events, client_config, &mut draw_config) {
            Ok(should_contiue) => {
                if should_contiue {
                    draw_client(terminal, &draw_config)?;
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
    draw_config: &mut DrawConfig,
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
    config : &DrawConfig,
) -> Result<(), Box<dyn std::error::Error>>
where
    B: Backend,
{
    terminal.draw(|frame| {        
        let size = frame.size();
        let outside_border = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        frame.render_widget(outside_border, size);

        let sidebar_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80)
                ]
                .as_ref(),
            )
            .split(frame.size());
        let sidebar = Block::default()
            .border_style(Style::default().fg(Color::Cyan))
            .borders(Borders::RIGHT)
            .border_type(BorderType::Thick);
        frame.render_widget(sidebar, sidebar_layout[0]);

        let text = vec![
            Spans::from("This is one sentence."),
            Spans::from("This is another sentence."),
        ];
        let paragraph = Paragraph::new(text.clone());
        let main_bar = Block::default()
            .style(Style::default().fg(Color::Blue))
            .title(Span::from(config.current_file_name.clone()));

        let main_panel_layout = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(2)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80),
                ]
                .as_ref(),
            )
            .split(frame.size());
        let paragraph_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(2)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80),
                ]
                .as_ref(),
            )
            .split(frame.size());
        
        frame.render_widget(paragraph, paragraph_layout[1]);
        frame.render_widget(main_bar, main_panel_layout[1]);
        
    })?;
    Ok(())
}