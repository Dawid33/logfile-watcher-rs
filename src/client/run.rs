use common::json_structs::ClientConfig;

use std::{io, io::Write, path::Path, path::PathBuf, sync::mpsc, time::Duration};
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

use super::{event, ui};

pub fn run_client<B>(
    client_config: &common::json_structs::ClientConfig,
    terminal: &mut Terminal<B>,
) -> Result<(), Box<dyn std::error::Error>>
where
    B: Backend,
{
    let mut state = ui::UIState::default().load_client_config(&client_config);

    let mut events = event::Events::with_config(event::Config {
        exit_key: Key::Char(client_config.key_map.quit),
        tick_rate: Duration::from_millis(250),
    });
    events.enable_exit_key();
    terminal.clear()?;

    let result = loop {
        match update_client(&mut events, client_config, &mut state) {
            Ok(should_contiue) => {
                if should_contiue {
                    if let Err(_e) = super::draw::draw_client(terminal, client_config, &state)
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

fn update_client(
    events: &mut event::Events,
    client_config: &ClientConfig,
    ui_state: &mut ui::UIState,
) -> Result<bool, Box<dyn std::error::Error>> {
    if let event::Event::Input(key) = events.next()? {
        if key == Key::Char(client_config.key_map.quit) {
            return Ok(false);
        }
        if key == Key::Char(client_config.key_map.left) && ui_state.percent_size_of_panes.0 > 0 {
            ui_state.percent_size_of_panes.0 -= 1;
            ui_state.percent_size_of_panes.1 += 1;
        }
        if key == Key::Char(client_config.key_map.right) && ui_state.percent_size_of_panes.1 > 0 {
            ui_state.percent_size_of_panes.0 += 1;
            ui_state.percent_size_of_panes.1 -= 1;
        }
        if key == Key::Char(client_config.key_map.help) {
            if let ui::UIMode::Help = ui_state.current_mode {
                ui_state.current_mode = ui_state.previous_mode;
            } else {
                ui_state.current_mode = ui::UIMode::Help;
            }
        }
    }
    Ok(true)
}
