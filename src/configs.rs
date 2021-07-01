use chrono::prelude::*;
use log::*;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{fmt, fs, io, io::BufReader, io::BufWriter, io::Read, io::Write, path::Path};
use url::Url;

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
            _ => serializer.collect_str("NULL"),
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
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub refersh_rate_miliseconds: u64,
    pub use_tui: bool,
    pub run_server: bool,
    pub force_update_miliseconds: u64,
    pub key_map: ShortcutKeyMap,
    pub ui_config: ClientUIConfig,
    pub server_config: ServerConfig,
}

///Keyboard controls
#[derive(Serialize, Deserialize, Clone)]
pub struct ShortcutKeyMap {
    pub quit: Key,
    pub help: Key,
    pub menu: Key,
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
    pub default_files: Vec<crate::files::File>,
}

impl Default for ClientUIConfig {
    fn default() -> Self {
        Self {
            background_color: (0, 0, 0),
            default_files: vec![
                crate::files::File {
                    url: Url::from_file_path(std::env::current_dir().unwrap().join("latest.log"))
                        .unwrap(),
                    display_name: "latest.log".to_string(),
                    contents: vec!["Did not read log yet...".to_string()],
                    last_modified: Option::None,
                },
                crate::files::File {
                    url: Url::from_file_path(
                        std::env::current_dir().unwrap().join("assets/testing1.txt"),
                    )
                    .unwrap(),
                    display_name: "testing1.txt".to_string(),
                    contents: vec!["Did not read log yet...".to_string()],
                    last_modified: Option::None,
                },
            ],
        }
    }
}

impl Default for ShortcutKeyMap {
    fn default() -> Self {
        Self {
            menu: Key::Char('m'),
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

impl Default for Config {
    fn default() -> Self {
        Self {
            refersh_rate_miliseconds: 50,
            force_update_miliseconds: 1000,
            run_server: false,
            use_tui: true,
            key_map: ShortcutKeyMap::default(),
            ui_config: ClientUIConfig::default(),
            server_config: ServerConfig::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
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

/// Try loading in struct from json file. If json file does not exist, load in
/// default values for the struct and create a new file with those values.
/*
pub fn load_struct_json<T>(path: &Path) -> T
where
    for<'de> T: Deserialize<'de> + Serialize + Default,
{
    let json_read_result = try_read_json_file(path);
    match json_read_result {
        Ok(file) => file,
        Err(e) => {
            let result = match e.classify() {
                serde_json::error::Category::Data => {
                    let default = T::default();
                    if let Err(_e) = try_write_json_file(&default, path) {
                        warn!("Cannot create json file at [{}].", path.display());
                        warn!("{}", e);
                    }
                    default
                }
                _ => {
                    warn!(
                        "Cannot load json file at [{}] because {}. Starting with default values...",
                        path.display(),
                        e
                    );
                    T::default()
                }
            };
            result
        }
    }
}
*/

pub fn load_struct_toml<T>(path: &Path) -> T
where
    for<'de> T: Deserialize<'de> + Serialize + Default,
{
    let json_read_result = try_read_toml_file(path);
    match json_read_result {
        Ok(file) => file,
        Err(e) => {
            warn!("Cannot load toml file at [{}] because {}. Creating new toml file with default values...", path.display(),e);
            let default = T::default();

            if let Err(_e) = try_write_toml_file(&default, path) {
                warn!("Cannot create toml file at [{}].", path.display());
                warn!("{}", e);
            }

            default
        }
    }
}

/*
fn try_read_json_file<'a, T>(path: &Path) -> serde_json::Result<T>
where
    for<'de> T: Deserialize<'de>,
{
    let file_handle_result = fs::File::open(path);
    match file_handle_result {
        Ok(file_handle) => {
            let buf_reader_handle = std::io::BufReader::new(file_handle);
            let config = serde_json::from_reader::<io::BufReader<fs::File>, T>(buf_reader_handle)?;
            return Ok(config);
        }
        Err(e) => Err(serde_json::Error::io(e)),
    }
}

fn try_write_json_file<T>(config: &T, path: &Path) -> serde_json::Result<()>
where
    T: Serialize,
{
    let json_config = serde_json::json!(config);
    let file_handle_result = fs::File::create(path);

    match file_handle_result {
        Ok(file_handle) => {
            let buf_writer_handle = std::io::BufWriter::new(file_handle);
            serde_json::to_writer_pretty(buf_writer_handle, &json_config)
        }
        Err(e) => Err(serde_json::Error::io(e)),
    }
}
*/

fn try_read_toml_file<'a, T>(path: &Path) -> Result<T, Box<dyn std::error::Error>>
where
    for<'de> T: Deserialize<'de>,
{
    let file_handle_result = fs::File::open(path);
    match file_handle_result {
        Ok(file_handle) => {
            let mut buf_reader_handle = BufReader::new(file_handle);
            let mut buf = String::new();
            if let Err(e) = buf_reader_handle.read_to_string(&mut buf) {
                error!(
                    "Cannot read toml file [{}] because {}",
                    path.to_str().unwrap(),
                    e
                )
            }
            match toml::from_str(buf.as_str()) {
                Ok(config) => Ok(config),
                Err(e) => {
                    error!(
                        "Cannot deserialize contents of the file [{}] because {}",
                        path.to_str().unwrap(),
                        e
                    );
                    Err(e.into())
                }
            }
        }
        Err(e) => Err(e.into()),
    }
}

fn try_write_toml_file<T>(config: &T, path: &Path) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
{
    let file_handle_result = fs::File::create(path);

    match file_handle_result {
        Ok(file_handle) => {
            let mut buf_writer_handle = BufWriter::new(file_handle);
            let buf: String = toml::to_string(config).unwrap();
            buf_writer_handle.write(buf.as_str().as_bytes()).unwrap();
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}
