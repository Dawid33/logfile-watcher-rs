#![allow(unused_imports)]
#![allow(dead_code)]
#![recursion_limit = "100"]

use chrono::prelude::*;
use events::Config;
use log::*;
use std::sync::{self, Arc, Mutex};
use std::{io, path::Path, time::Duration};
use tui::{backend::Backend, text::Spans, Terminal};

#[cfg(windows)]
use tui::backend::CrosstermBackend;
#[cfg(unix)]
use {termion::raw::IntoRawMode, tui::backend::TermionBackend};

mod buffer;
mod configs;
mod events;
mod files;
mod networking;
mod ui;
mod update;

pub const CONFIG_FILENAME: &str = "config.toml";

#[derive(PartialEq)]
pub enum UpdateResult {
    Quit,
    DrawCall,
    None,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(debug_assertions) {
        if let Err(_e) = std::fs::File::open("latest.log") {
            std::fs::File::create("latest.log").unwrap();
        }
        simple_logging::log_to_file("latest.log", LevelFilter::Info).unwrap();
        info!("Running in debug mode.");
    } else {
        simple_logging::log_to_stderr(LevelFilter::Warn);
    }

    //Configuration file for the client.
    let path = Path::new(CONFIG_FILENAME);
    let config = configs::load_struct_toml::<configs::Config>(path);

    //Run code for tui.
    if cfg!(unix) {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        run_client(config, &mut terminal)?;
    } else if cfg!(windows) {
        unimplemented!("No windows implementation :)");
    }

    Ok(())
}

/**
 * ## run_client()
 * - `client_config` is a struct loaded in from a json file that contains
 * all of the users preferences. The only reason it is mutable is in case the
 * user wants to reload it.
 *
 * - `terminal` handle to the current terminal
 *
 * This function creates the neccesary structs to store program state. These structs
 * are passed into update_client() and draw_client().
 */
pub fn run_client<B: Backend>(
    client_config: configs::Config,
    terminal: &mut Terminal<B>,
) -> Result<(), Box<dyn std::error::Error>> {
    //UI state that dictates what ui to draw and how to draw it.
    let mut ui_state = ui::UIState::new(&client_config.ui_config);

    //Buffer that holds currently tracked files.
    let buffer = buffer::Buffer::new(ui_state.sidebar_list.items.clone());
    //Initialize buffer with initial files from ui_state ( origininally from ui_config )
    let mut buffer = Arc::new(Mutex::from(buffer));
    // Initialize event loop.
    let mut events = events::EventManager::new(
        events::Config {
            exit_key: client_config.key_map.quit,
        },
        client_config.clone(),
        buffer.clone(),
    );
    let mut unlocked_buffer = buffer.lock().unwrap();
    for file in &ui_state.sidebar_list.items {
        unlocked_buffer.add_file(file.clone());
    }
    drop(unlocked_buffer);
    //Clear the terminal to ensure a blank slate.
    terminal.clear()?;

    //Draw to the screen once, then only update the screen when something changes.
    ui::draw::draw_ui(terminal, &mut ui_state, &client_config)?;

    let result = loop {
        match update::update(&mut ui_state, &mut events, &client_config, &mut buffer) {
            Ok(result) => match result {
                UpdateResult::DrawCall => {
                    if let Err(e) = ui::draw::draw_ui(terminal, &mut ui_state, &client_config) {
                        break Err(e);
                    }
                }
                UpdateResult::Quit => {
                    break Ok(());
                }
                UpdateResult::None => (),
            },
            Err(e) => break Err(e),
        }
    };
    //Clear the terminal because the tui pollutes the space above the prompt after exit.
    terminal.clear()?;
    result
}

pub fn run_server(
    _buffer: Arc<Mutex<buffer::Buffer>>,
    _config: configs::ServerConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
