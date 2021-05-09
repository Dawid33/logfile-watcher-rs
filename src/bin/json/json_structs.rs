use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::net::*;
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct ClientConfig {
    pub url: Url,
    pub use_tui: bool,
    pub key_map : ShortcutKeyMap
}

#[derive(Serialize, Deserialize)]
pub struct ShortcutKeyMap {
    pub quit : char,
    pub left : char,
    pub right : char,
}

impl Default for ShortcutKeyMap {
    fn default() -> Self {
        Self {
            quit : 'q',
            left : 'h',
            right : 'l',
        }
    }
}

impl Default for ClientConfig {
    fn default() -> ClientConfig {
        ClientConfig {
            url: Url::parse("ws://localhost:9001/socket").unwrap(),
            use_tui: true,
            key_map : ShortcutKeyMap::default(),
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
