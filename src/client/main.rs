extern crate common;
extern crate chrono;

use {
    common::json_structs::ClientConfig,
    log::*,
    tui::{
        backend::Backend,
        Terminal,
    },
    std::{
        io, path::Path,
        time::Duration,
    }
};
    
#[cfg(unix)]
use {
    termion::raw::IntoRawMode,
    tui::backend::TermionBackend,
    termion::event::Key,
};
#[cfg(windows)]
use {
    tui::backend::CrosstermBackend,
};

mod events;
pub mod networking;
mod update;
mod ui;

pub const CONFIG_FILENAME : &str = "client_config.json";
pub const LOGS_DIR_NAME : &str = "logs";
const MAX_AMOUNT_OF_LOGS : u16 = 1;
const DEBUG_FILE_NAME_WITH_FULL_TIMESTAMP : bool = false;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(debug_assertions) {
        start_logger();
        info!("Running in debug mode.")
    } else {
        simple_logging::log_to_stderr(LevelFilter::Warn);
    }
    
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
    let mut events = events::Events::with_config(events::Config {
        exit_key: Key::from(client_config.key_map.quit),
        tick_rate: Duration::from_millis(client_config.refersh_rate_miliseconds),
    });
    events.enable_exit_key();
    terminal.clear()?;

    let result = loop {
        match update::update_client(&mut events, &mut client_config, &mut state) {
            Ok(should_contiue) => {
                if should_contiue {
                    if let Err(_e) = ui::draw::draw_client(terminal, &client_config, &mut state)
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

///This whole function is a bit nasty. fix at some point.
fn start_logger() {
    // Create directory to store logs.
    let directory : std::fs::ReadDir = std::fs::read_dir(LOGS_DIR_NAME).or_else::<std::fs::ReadDir,_>(|x| {
        std::fs::create_dir(LOGS_DIR_NAME).expect(format!("Cannot create logs directory [{}] because {}", LOGS_DIR_NAME, x).as_str());
        Ok(std::fs::read_dir(LOGS_DIR_NAME).expect(format!("Cannot read logs directory [{}]", LOGS_DIR_NAME).as_str()))
    }).expect("Cannot read logs directory");
    //If there are too many logfiles, delete the oldest ones.
    //This seems like a bad way of doing it, rewrite this at some point.
    let mut dir_entries : Vec<Result<std::fs::DirEntry, std::io::Error>> = directory.collect();
    if dir_entries.len() >= MAX_AMOUNT_OF_LOGS.into() {
        &dir_entries.sort_by(|a,b|{
            a.as_ref().unwrap().metadata().unwrap().created().unwrap().partial_cmp(&b.as_ref().unwrap().metadata().unwrap().created().unwrap()).unwrap() // This is sooooooo nasty.
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
        std::path::PathBuf::from(format!("{}/debug {}.log",LOGS_DIR_NAME,chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()))
    } else {
        std::path::PathBuf::from(format!("{}/debug.log",LOGS_DIR_NAME))
    };
    simple_logging::log_to_file(logfile_name, LevelFilter::Info).unwrap();
}
//Communication between threads.
//let (rx, _tx) = mpsc::channel::<tungstenite::error::Error>();
//networking::connect(config.url.clone(), rx.clone());
