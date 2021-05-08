#![allow(dead_code)]
#![allow(unused_imports)]

use std::net::TcpListener;
use std::thread;
use tungstenite::server::accept;

mod json;

fn main() {
    /*
    let config = load_struct::<Config>(std::path::Path::new("config.json"));
    let server = TcpListener::bind(config.ip).unwrap();
    for stream in server.incoming() {
        thread::spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            handle_connection(&mut websocket).unwrap();
        });
    }
    */
}

fn handle_connection(websocket : &mut tungstenite::WebSocket<std::net::TcpStream>) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let msg = websocket.read_message().unwrap();

        // We do not want to send back ping/pong messages.
        if msg.is_binary() || msg.is_text() {
            println!("{}", msg);
            websocket.write_message(msg).unwrap();
        }
    }
    unreachable!()
}