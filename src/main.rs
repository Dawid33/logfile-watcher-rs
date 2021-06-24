#![allow(unused_imports)]
#![allow(dead_code)]
#![recursion_limit = "24"]

use {
    common::configs::*,
    log::*,
    std::{io, path::Path, time::Duration},
    tui::{backend::Backend, Terminal},
    tui::text::Spans,
    std::sync::{
        self,
        Arc,
        Mutex,
    },
};

#[cfg(windows)]
use tui::backend::CrosstermBackend;
#[cfg(unix)]
use {termion::raw::IntoRawMode, tui::backend::TermionBackend};

pub mod networking;
pub mod common;
mod events;
mod ui;
mod update;
mod buffer;
mod files;

pub const CONFIG_FILENAME: &str = "client_config.toml";

pub struct ProgramState<'a> {
    pub events : events::EventManager,
    pub ui_state : ui::UIState<'a>,
    pub client_config : common::configs::ClientConfig,
    pub buffer : std::sync::Arc<std::sync::Mutex<buffer::Buffer>>,
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
    let config = common::load_struct_toml::<ClientConfig>(path);

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
pub fn run_client<B : Backend>(
    client_config: common::configs::ClientConfig,
    terminal: &mut Terminal<B>,
) -> Result<(), Box<dyn std::error::Error>>
{
    //UI state that dictates what ui to draw and how to draw it.
    let mut ui_state = ui::UIState::new(&client_config.ui_config);
    
    //Buffer that holds currently tracked files.
    let buffer = buffer::Buffer::new(ui_state.sidebar_list.items.clone());
    let buffer = Arc::new(Mutex::from(buffer));
    
    // Initialize event loop.
    let events = events::EventManager::with_config(events::Config {
        exit_key: client_config.key_map.quit,
        tick_rate: Duration::from_millis(client_config.refersh_rate_miliseconds),
    });

    //Bringing it all together
    let mut program_state = ProgramState {
        events: events,
        ui_state: ui_state,
        client_config:client_config,
        buffer: buffer,
    };

    //Clear the terminal to ensure a blank slate.
    terminal.clear()?;

    let result = loop {
        match update::update(&mut program_state) {
            Ok((should_run, should_draw)) => {
                if should_run {
                    if should_draw {
                        if let Err(_e) = ui::draw::draw_ui(terminal, &mut program_state.ui_state)
                        {
                            break Ok(());
                        }
                    }
                } else {
                    break Ok(());
                }
            }
            Err(e) => break Err(e),
        }
    };
    //Clear the terminal because the tui pollutes the space above the prompt after exit.
    terminal.clear()?;
    result
}