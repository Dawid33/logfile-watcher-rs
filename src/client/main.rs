extern crate chrono;
extern crate common;

use {
    common::configs::*,
    log::*,
    std::{io, path::Path, time::Duration},
    tui::{backend::Backend, Terminal},
};

#[cfg(windows)]
use tui::backend::CrosstermBackend;
#[cfg(unix)]
use {termion::raw::IntoRawMode, tui::backend::TermionBackend};

mod events;
pub mod networking;
mod ui;
mod update;

pub const CONFIG_FILENAME: &str = "client_config.toml";
pub const LOGS_DIR_NAME: &str = "logs";
//const MAX_AMOUNT_OF_LOGS: u16 = 1;
//const DEBUG_FILE_NAME_WITH_FULL_TIMESTAMP: bool = false;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(debug_assertions) {
        //start_logger();
        if let Err(_e) = std::fs::File::open("latest.log") {
            std::fs::File::create("lastest.log").unwrap();
        }
        simple_logging::log_to_file("latest.log", LevelFilter::Trace).unwrap();
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
 * - terminal handle to the current terminal
 *
 * This function creates the neccesary structs to store program state. These structs
 * are passed into update_client() and draw_client().
 */
pub fn run_client<B>(
    mut client_config: common::configs::ClientConfig,
    terminal: &mut Terminal<B>,
) -> Result<(), Box<dyn std::error::Error>>
where
    B: Backend,
{
    let mut state = ui::UIState::default().load_from_client_config(&client_config);
    
    state.sidebar_list.items = client_config.ui_config.default_urls.iter().map(|path|{
        let items = path.path_segments().ok_or_else(|| "cannot be base").unwrap();
        let items : Vec<&str> = items.collect();
        (path.clone(), String::from(*items.last().unwrap()))
    }).collect();

    let mut events = events::Events::with_config(events::Config {
        exit_key: client_config.key_map.quit,
        tick_rate: Duration::from_millis(client_config.refersh_rate_miliseconds),
    });

    events.enable_exit_key();
    //Clear the terminal to ensure a blank slate.
    terminal.clear()?;

    let result = loop {
        match update::update_client(&mut events, &mut client_config, &mut state) {
            Ok((should_run,should_draw)) => {
                if should_run {
                    if should_draw {
                        if let Err(_e) = ui::draw::draw_client(terminal, &client_config, &mut state) {
                            break Ok(());
                        }
                    }
                } else {
                    break Ok(())
                }
            }
            Err(e) => break Err(e),
        }
    };
    //Clear the terminal because the tui pollutes the space above the prompt after exit.
    terminal.clear()?;
    result
}

/*
fn start_logger() {
    // Create directory to store logs.
    let directory : std::fs::ReadDir = std::fs::read_dir(LOGS_DIR_NAME).or_else::<std::fs::ReadDir,_>(|x| {
        std::fs::create_dir(LOGS_DIR_NAME).expect(format!("Cannot create logs directory [{}] because {}", LOGS_DIR_NAME, x).as_str());
        Ok(std::fs::read_dir(LOGS_DIR_NAME).expect(format!("Cannot read logs directory [{}]", LOGS_DIR_NAME).as_str()))
    }).expect("Cannot read logs directory. How can this app create a dir and not be able to access it... if thats the problem");

    //If there are too many logfiles, delete the oldest ones.
    //This seems like a bad way of doing it, rewrite this at some point.
    let mut dir_entries: Vec<Result<std::fs::DirEntry, std::io::Error>> = directory.collect();
    if dir_entries.len() >= MAX_AMOUNT_OF_LOGS.into() {
        &dir_entries.sort_by(|a, b| {
            a.as_ref()
                .unwrap()
                .metadata()
                .unwrap()
                .created()
                .unwrap()
                .partial_cmp(&b.as_ref().unwrap().metadata().unwrap().created().unwrap())
                .unwrap() // This is sooooooo nasty.
        });
        let mut old_len = dir_entries.len();
        while old_len >= MAX_AMOUNT_OF_LOGS.into() {
            std::fs::remove_file(dir_entries.first().unwrap().as_ref().unwrap().path()).unwrap();
            dir_entries.remove(0).unwrap();
            old_len -= 1;
        }
    }
    // Create logging facility into stderr.
    let logfile_name = if DEBUG_FILE_NAME_WITH_FULL_TIMESTAMP {
        std::path::PathBuf::from(format!(
            "{}/debug {}.log",
            LOGS_DIR_NAME,
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
        ))
    } else {
        std::path::PathBuf::from(format!("{}/debug.log", LOGS_DIR_NAME))
    };
    simple_logging::log_to_file(logfile_name, LevelFilter::Info).unwrap();
}
*/