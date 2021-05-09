#[cfg_attr(target_family = "unix", path = "linux.rs")]
#[cfg_attr(target_family = "windows", path = "windows.rs")]
pub mod cli;
mod event;
pub mod networking;
mod ui;

use crate::json_structs::ClientConfig;

use std::{io, io::Write, time::Duration};

use termion::{event::Key, raw::IntoRawMode};

use tui::{backend::TermionBackend, Terminal};

pub fn run_client(
    client_config: &crate::json_structs::ClientConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut draw_config = ui::DrawConfig::default();

    let stdout = io::stdout().into_raw_mode()?;
    let termion_backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(termion_backend)?;
    terminal.clear()?;

    let mut events = event::Events::with_config(event::Config {
        exit_key: Key::Char(client_config.key_map.quit),
        tick_rate: Duration::from_millis(250),
    });
    events.enable_exit_key();

    loop {
        match update_client(&mut terminal, &mut events, &mut draw_config, client_config) {
            Ok(should_contiue) => {
                if should_contiue {
                    cli::draw_client(&mut terminal, &draw_config)?;
                } else {
                    return Ok(());
                }
            }
            Err(e) => return Err(e),
        }
    }
}

fn update_client<W>(
    _terminal: &mut tui::Terminal<tui::backend::TermionBackend<W>>,
    events: &mut event::Events,
    _draw_config: &mut ui::DrawConfig,
    client_config: &ClientConfig,
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
