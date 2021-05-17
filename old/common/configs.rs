use {
    serde::{Deserialize, Deserializer, Serialize, Serializer},
    serde::de::{self, Visitor},
    url::Url,
    std::fmt,
};

// Wrapper over the keys enums in other backends e.g termion::event:Key
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

impl Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            Key::Char(c) => serializer.collect_str(c),
            _ => {
                serializer.collect_str("NULL")
            }
        }
    }
}

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Key, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(KeyStringVisitor)
    }
}

struct KeyStringVisitor;
impl<'de> Visitor<'de> for KeyStringVisitor {
    type Value = Key;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Key::Char(value.chars().next().unwrap()))
    }
}

#[cfg(unix)]
impl From<termion::event::Key> for Key {
    fn from(key: termion::event::Key) -> Key {
        match key {
            termion::event::Key::Backspace => Key::Backspace,
            termion::event::Key::Left => Key::Left,
            termion::event::Key::Right => Key::Right,
            termion::event::Key::Up => Key::Up,
            termion::event::Key::Down => Key::Down,
            termion::event::Key::Home => Key::Home,
            termion::event::Key::End => Key::End,
            termion::event::Key::PageUp => Key::PageUp,
            termion::event::Key::PageDown => Key::PageDown,
            termion::event::Key::BackTab => Key::BackTab,
            termion::event::Key::Delete => Key::Delete,
            termion::event::Key::Insert => Key::Insert,
            termion::event::Key::F(x) => Key::F(x),
            termion::event::Key::Char(c) => Key::Char(c),
            termion::event::Key::Alt(c) => Key::Alt(c),
            termion::event::Key::Ctrl(c) => Key::Ctrl(c),
            termion::event::Key::Null => Key::Null,
            termion::event::Key::Esc => Key::Esc,
            _ => Key::Null,
        }
    }
}

///User configuration that is stored as a json file.
#[derive(Serialize, Deserialize)]
pub struct ClientConfig {
    pub refersh_rate_miliseconds: u64,
    pub url: Url,
    pub use_tui: bool,
    pub key_map: ShortcutKeyMap,
    pub ui_config: ClientUIConfig,
}

///Keyboard controls
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

///Configuration specifically pertaining to the ui.
#[derive(Serialize, Deserialize, Clone)]
pub struct ClientUIConfig {
    pub background_color: (u8, u8, u8),
    pub default_urls: Vec<Url>,
}

impl Default for ClientUIConfig {
    fn default() -> Self {
        Self {
            background_color: (0, 0, 0),
            default_urls: vec![Url::from_file_path(
                std::env::current_dir().unwrap().join("latest.log"),
            )
            .unwrap()],
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
            refersh_rate_miliseconds: 50,
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
