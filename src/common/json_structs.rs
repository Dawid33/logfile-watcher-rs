use {
    serde::{Deserialize, Serialize},
    url::Url,
};

// Wrapper over the keys in other backends.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Key {
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Alt(char),
    Ctrl(char),
    Null,
    Esc,
}

#[cfg(unix)]
impl From<Key> for termion::event::Key {
    fn from(key: Key) -> termion::event::Key {
        match key {
            Key::Backspace => termion::event::Key::Backspace,
            Key::Left => termion::event::Key::Left,
            Key::Right => termion::event::Key::Right,
            Key::Up => termion::event::Key::Up,
            Key::Down => termion::event::Key::Down,
            Key::Home => termion::event::Key::Home,
            Key::End => termion::event::Key::End,
            Key::PageUp => termion::event::Key::PageUp,
            Key::PageDown => termion::event::Key::PageDown,
            Key::BackTab => termion::event::Key::BackTab,
            Key::Delete => termion::event::Key::Delete,
            Key::Insert => termion::event::Key::Insert,
            Key::F(x) => termion::event::Key::F(x),
            Key::Char(c) => termion::event::Key::Char(c),
            Key::Alt(c) => termion::event::Key::Alt(c),
            Key::Ctrl(c) => termion::event::Key::Ctrl(c),
            Key::Null => termion::event::Key::Null,
            Key::Esc => termion::event::Key::Esc,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClientConfig {
    pub refersh_rate_miliseconds: u64,
    pub url: Url,
    pub use_tui: bool,
    pub key_map: ShortcutKeyMap,
    pub ui_config: ClientUIConfig,
}

#[derive(Serialize, Deserialize)]
pub struct ShortcutKeyMap {
    pub quit: Key,
    pub help: Key,
    pub left: Key,
    pub right: Key,
    pub resize_left: Key,
    pub resize_right: Key,
    pub up: Key,
    pub down: Key,
    pub reload_config: Key,
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
            quit: Key::Char('q'),
            help: Key::Char('?'),
            left: Key::Char('h'),
            right: Key::Char('l'),
            resize_left: Key::Char('H'),
            resize_right: Key::Char('L'),
            up: Key::Char('j'),
            down: Key::Char('k'),
            reload_config: Key::Char('r'),
        }
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            url: Url::parse("ws://localhost:9001/socket").unwrap(),
            refersh_rate_miliseconds: 250,
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
