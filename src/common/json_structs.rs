use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct ClientConfig {
    pub url: Url,
    pub use_tui: bool,
    pub key_map: ShortcutKeyMap,
    pub ui_config: ClientUIConfig,
}

#[derive(Serialize, Deserialize)]
pub struct ShortcutKeyMap {
    pub quit: char,
    pub help: char,
    pub left: char,
    pub right: char,
    pub reload_config: char,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ClientUIConfig {
    pub background_color: (u8, u8, u8),
}

impl Default for ClientUIConfig {
    fn default() -> Self {
        Self {
            background_color: (0, 0, 0),
        }
    }
}

impl Default for ShortcutKeyMap {
    fn default() -> Self {
        Self {
            quit: 'q',
            help: '?',
            left: 'h',
            right: 'l',
            reload_config : 'r'
        }
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            url: Url::parse("ws://localhost:9001/socket").unwrap(),
            use_tui: true,
            key_map: ShortcutKeyMap::default(),
            ui_config: ClientUIConfig::default(),
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
