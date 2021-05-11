#![allow(dead_code)]
#![allow(unused_imports)]
extern crate common;
use common::json_structs::ClientConfig;

use std::{io, io::Write, path::Path,path::PathBuf, sync::mpsc, time::Duration};
#[cfg(unix)]
use termion::event::Key;

use tui::{
    backend::Backend,
    buffer::{Buffer, Cell},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};

#[cfg(unix)]
use termion::raw::IntoRawMode;
#[cfg(windows)]
use tui::backend::CrosstermBackend;
#[cfg(unix)]
use tui::backend::TermionBackend;

#[cfg_attr(windows, path = "src/client/events_crossterm.rs")]
#[cfg_attr(unix, path = "events_termion.rs")]
mod event;

mod draw;
pub mod networking;
mod update;
mod ui;
use ui::*;

pub const CONFIG_FILENAME : &str = "client_config.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Configuration file for the client.
    let path = Path::new(CONFIG_FILENAME);
    let config = common::load_struct::<ClientConfig>(path);

    //Run code for tui.
    if cfg!(unix) {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        run_client(config, &mut terminal)?;
    } else if cfg!(windows) {
        unimplemented!("No windows implementation :)");
    }

    // If nothing is printed before exiting,the empty space
    // following the shell prompt is black for some reason
    // (in visual studio). So just print a new line :)
    Ok(())
}

pub fn run_client<B>(
    mut client_config: common::json_structs::ClientConfig,
    terminal: &mut Terminal<B>,
) -> Result<(), Box<dyn std::error::Error>>
where
    B: Backend,
{
    let mut state = ui::UIState::default().load_from_client_config(&client_config);
    state.sidebar_list.items = vec![
        String::from("Item 1"),
        String::from("Item 2"),
        String::from("Item 3"),
        String::from("Item 4"),
        String::from("Item 5"),
    ];
    let mut events = event::Events::with_config(event::Config {
        exit_key: Key::from(client_config.key_map.quit),
        tick_rate: Duration::from_millis(client_config.refersh_rate_miliseconds),
    });
    events.enable_exit_key();
    terminal.clear()?;

    let result = loop {
        match update::update_client(&mut events, &mut client_config, &mut state) {
            Ok(should_contiue) => {
                if should_contiue {
                    if let Err(_e) = draw::draw_client(terminal, &client_config, &mut state)
                    {
                        break Ok(());
                    }
                } else {
                    break Ok(());
                }
            }
            Err(e) => break Err(e),
        }
    };
    terminal.clear()?;
    result
}

//Communication between threads.
//let (rx, _tx) = mpsc::channel::<tungstenite::error::Error>();
//networking::connect(config.url.clone(), rx.clone());
