#![allow(dead_code)]
#![allow(unused_imports)]

use std::path::Path;
use std::io;
use std::io::Write;
use tui::Terminal;
use tui::backend::Backend;
use std::sync::mpsc;

#[cfg(unix)]
use tui::backend::TermionBackend;
#[cfg(unix)]
use termion::raw::IntoRawMode;
#[cfg(windows)]
use tui::backend::CrosstermBackend;



mod client_src;
mod json;
use client_src::*;
use json::json_structs::*;
use json::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Configuration file for the client.
    let path = Path::new("client_config.json");
    let config = load_struct::<ClientConfig>(path);

    //Run code for tui.
    if cfg!(unix) {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        run_client(&config, &mut terminal)?;
    } else if cfg!(windows) {

    }
    
    //Communication between threads.
    //let (rx, _tx) = mpsc::channel::<tungstenite::error::Error>();
    //networking::connect(config.url.clone(), rx.clone());
    
    // If nothing is printed before exiting,the empty space
    // following the shell prompt is black for some reason
    // (in visual studio). So just print a new line :)
    println!("");
    Ok(())
}