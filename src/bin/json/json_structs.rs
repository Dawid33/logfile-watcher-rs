use std::net::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub ip: SocketAddr,
    pub use_tui: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            name: String::from("hello"),
            ip: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9001),
            use_tui: true,
        }
    }
}