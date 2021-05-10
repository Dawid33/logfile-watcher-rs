#![allow(dead_code)]
#![allow(unused_imports)]
extern crate common;
use common::json_structs::ClientConfig;

use std::{io, io::Write, path::Path, sync::mpsc, time::Duration};
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

#[cfg(unix)]
use termion::raw::IntoRawMode;
#[cfg(windows)]
use tui::backend::CrosstermBackend;
#[cfg(unix)]
use tui::backend::TermionBackend;

mod draw;
#[cfg(unix)]
mod event;
pub mod networking;
mod run;
mod ui;
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
        run::run_client(&config, &mut terminal)?;
    } else if cfg!(windows) {
        unimplemented!("No windows implementation :)");
    }

    // If nothing is printed before exiting,the empty space
    // following the shell prompt is black for some reason
    // (in visual studio). So just print a new line :)
    println!("");
    Ok(())
}

//Communication between threads.
//let (rx, _tx) = mpsc::channel::<tungstenite::error::Error>();
//networking::connect(config.url.clone(), rx.clone());
