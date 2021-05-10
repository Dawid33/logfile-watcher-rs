use common::json_structs::ClientConfig;

use std::{io, io::Write, time::Duration, path::Path,sync::mpsc,path::PathBuf};
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

use super::{
    event,
    ui,
};

pub fn run_client<B>(
    mut client_config: common::json_structs::ClientConfig,
    terminal : &mut Terminal<B>,
) -> Result<(), Box<dyn std::error::Error>> 
where
    B: Backend
{   
    let mut state = ui::UIState::default();
    state.current_file_path = Some(PathBuf::from(String::from(" This is my file ")));
    let mut events = event::Events::with_config(event::Config {
        exit_key: Key::Char(client_config.key_map.quit),
        tick_rate: Duration::from_millis(250),
    });
    events.enable_exit_key();
    terminal.clear()?;

    loop {
        match update_client(&mut events, &mut client_config) {
            Ok(should_contiue) => {
                if should_contiue {
                    super::draw::draw_client(terminal, &mut client_config, &state)?;
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
    client_config: &mut ClientConfig
) -> Result<bool, Box<dyn std::error::Error>>
{
    if let event::Event::Input(key) = events.next()? {
        if key == Key::Char(client_config.key_map.quit) {
            return Ok(false);
        }
    }
    Ok(true)
}