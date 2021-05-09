use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::net::*;
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct ClientConfig {
    pub name: String,
    pub url: Url,
    pub use_tui: bool,
}

impl Default for ClientConfig {
    fn default() -> ClientConfig {
        ClientConfig {
            name: String::from("hello"),
            url: Url::parse("ws://localhost:9001/socket").unwrap(),
            use_tui: true,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub url: Url,
    pub use_tui: bool,
}

impl Default for ServerConfig {
    fn default() -> ServerConfig {
        ServerConfig {
            name: String::from("hello"),
            url: Url::parse("ws://localhost:9001/socket").unwrap(),
            use_tui: true,
        }
    }
}
