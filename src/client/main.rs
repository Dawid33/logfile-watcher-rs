#![allow(dead_code)]
#![allow(unused_imports)]
extern crate common;
use common::json_structs::ClientConfig;

use std::{io, io::Write, time::Duration, path::Path,sync::mpsc};
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

#[cfg(unix)]
use tui::backend::TermionBackend;
#[cfg(unix)]
use termion::raw::IntoRawMode;
#[cfg(windows)]
use tui::backend::CrosstermBackend;

pub mod networking;
#[cfg(unix)]
mod event;
mod ui;
mod run;
mod draw;
use ui::*;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Configuration file for the client.
    let path = Path::new("client_config.json");
    let config = common::load_struct::<ClientConfig>(path);

    //Run code for tui.
    if cfg!(unix) {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        run::run_client(config, &mut terminal)?;
    } else if cfg!(windows) {
        unimplemented!("No windows implementation :)");
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