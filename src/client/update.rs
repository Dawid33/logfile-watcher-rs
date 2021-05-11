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

pub fn update_client(
    events: &mut event::Events,
    client_config: &mut ClientConfig,
    ui_state: &mut ui::UIState,
) -> Result<bool, Box<dyn std::error::Error>> {
    if let event::Event::Input(key) = events.next()? {
        if key == Key::Char(client_config.key_map.quit) {
            return Ok(false);
        }
        if key == Key::Char(client_config.key_map.left) && ui_state.percent_size_of_panes.0 > 2 {
            ui_state.percent_size_of_panes.0 -= 2;
            ui_state.percent_size_of_panes.1 += 2;
        }
        if key == Key::Char(client_config.key_map.right) && ui_state.percent_size_of_panes.1 > 2 {
            ui_state.percent_size_of_panes.0 += 2;
            ui_state.percent_size_of_panes.1 -= 2;
        }
        if key == Key::Char(client_config.key_map.help) {
            if let ui::UIMode::Help = ui_state.current_mode {
                ui_state.current_mode = ui_state.previous_mode;
            } else {
                ui_state.current_mode = ui::UIMode::Help;
            }
        }
        if key == Key::Char(client_config.key_map.reload_config) {
            let config = common::load_struct::<ClientConfig>(Path::new(super::CONFIG_FILENAME));
            *client_config = config;
        }
        if key == Key::Left {
            ui_state.sidebar_list.unselect();
        }
        if key == Key::Down {
            ui_state.sidebar_list.next();
        }
        if key == Key::Up {
            ui_state.sidebar_list.previous();
        }
    }
    Ok(true)
}
