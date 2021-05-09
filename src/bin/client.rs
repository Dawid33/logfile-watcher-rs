#![allow(dead_code)]
#![allow(unused_imports)]

use std::io;
use std::io::Read;
use std::net;
use std::path::Path;
use std::sync::mpsc;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;
use tungstenite::{connect, Message};
use url::Url;

mod client_src;
mod json;

use client_src::*;
use json::json_structs::*;
use json::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Configuration file for the client.
    let path = Path::new("client_config.json");
    let _config = load_struct::<ClientConfig>(path);

    //Run code for tui.
    cli::run_client()?;

    //Communication between threads.
    //let (rx, _tx) = mpsc::channel::<tungstenite::error::Error>();
    //networking::connect(config.url.clone(), rx.clone());

    Ok(())
}
