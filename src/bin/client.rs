#![allow(dead_code)]
//#![allow(unused_imports)]

use std::path::Path;

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
    run_client(&config)?;

    //Communication between threads.
    //let (rx, _tx) = mpsc::channel::<tungstenite::error::Error>();
    //networking::connect(config.url.clone(), rx.clone());

    Ok(())
}
