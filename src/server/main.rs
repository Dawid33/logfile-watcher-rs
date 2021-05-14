extern crate common;

use common::configs::*;
use std::net::TcpListener;

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

fn main() {
    let config =
        common::load_struct_toml::<ServerConfig>(std::path::Path::new("server_config.toml"));
    let server = TcpListener::bind(&*config.url.socket_addrs(|| None).unwrap()).unwrap();
    for stream in server.incoming() {
        std::thread::spawn(move || {
            let callback = |req: &Request, mut response: Response| {
                println!("Received a new ws handshake");
                println!("The request's path is: {}", req.uri().path());
                println!("The request's headers are:");
                for (ref header, _value) in req.headers() {
                    println!("* {}", header);
                }

                // Let's add an additional header to our response to the client.
                let headers = response.headers_mut();
                headers.append("MyCustomHeader", ":)".parse().unwrap());
                headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());

                Ok(response)
            };
            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            loop {
                let msg = websocket.read_message().unwrap();
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}

#[allow(dead_code)]
fn handle_connection(
    websocket: &mut tungstenite::WebSocket<std::net::TcpStream>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let msg = websocket.read_message().unwrap();

        // We do not want to send back ping/pong messages.
        if msg.is_binary() || msg.is_text() {
            println!("{}", msg);
            websocket.write_message(msg).unwrap();
        }
    }
}
