#![allow(unused_imports)]
#![allow(dead_code)]

use {
    common::configs::*,
    log::*,
    std::{io, path::Path, time::Duration},
    tui::{backend::Backend, Terminal},
    tui::text::Spans,
    std::sync,
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
    pub buffer : std::sync::Arc<buffer::Buffer>,
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
    let mut ui_state = ui::UIState::default().load_from_client_config(&client_config);

    // Get the default sidebar items from the configuration file.
    ui_state.sidebar_list.items = client_config
        .ui_config
        .default_urls
        .iter()
        .map(|path| {
            let items = path
                .path_segments()
                .ok_or_else(|| "cannot be base")
                .unwrap();
            let items: Vec<&str> = items.collect();
            (path.clone(), String::from(*items.last().unwrap()))
        })
        .collect();
        
    let buffer = buffer::Buffer {
        files : ui_state.sidebar_list.items.clone().into(),
    };
    let arc_buffer = std::sync::Arc::from(buffer);
    
    // Initialize event loop.
    let mut events = events::EventManager::with_config(events::Config {
        exit_key: client_config.key_map.quit,
        tick_rate: Duration::from_millis(client_config.refersh_rate_miliseconds),
    });
    events.enable_exit_key();

    let mut program_state = ProgramState {
        events: events,
        ui_state: ui_state,
        client_config:client_config,
        buffer: arc_buffer,
    };


    //Clear the terminal to ensure a blank slate.
    terminal.clear()?;

    let result = loop {
        match update::update(&mut program_state) {
            Ok((should_run, should_draw)) => {
                if should_run {
                    if should_draw {
                        if let Err(_e) = ui::draw::draw_ui(terminal, &mut program_state)
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