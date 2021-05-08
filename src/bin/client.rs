#![allow(dead_code)]
#![allow(unused_imports)]

use std::path::Path;
use tungstenite::{connect, Message};
use std::net;
use url::Url;
use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

mod json;
mod cli;

use json::*;
use json::json_structs::*;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    let path = Path::new("config.json");
    let config = load_struct::<Config>(path);
    let stream = std::net::TcpStream::connect(config.ip);
    let stream = stream.unwrap();
    let (mut socket, _response) = tungstenite::client("hello world!", stream).unwrap();
    socket.write_message(Message::Text("Hello WebSocket".into())).unwrap();
    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
}

