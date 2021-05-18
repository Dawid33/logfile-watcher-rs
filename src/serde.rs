use super::events::Key;

#[derive(Clone)]
pub struct Config {
    pub key_map : KeyConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            key_map: KeyConfig::default(),
        }
    }
}

#[derive(Clone)]
pub struct KeyConfig {
    pub exit_key : Key,
}
impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            exit_key : Key::Char('q'),
        }
    }
}