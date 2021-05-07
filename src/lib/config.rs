
use std::fmt;
use serde::{Serialize,Deserialize};
use serde_json::*;

#[derive(Debug)]
pub enum ConfigErrorKind{
    FileReadError,
    IncorrectFormattingError,
}

#[derive(Debug)]
pub struct ConfigError{
    error_kind : ConfigErrorKind,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config Error!")
    }
}
impl fmt::Display for ConfigErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config Error Kind!")
    }
}
impl ConfigError {
    pub fn new (c : ConfigErrorKind) -> ConfigError {
        ConfigError{
            error_kind : c,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config<'a> {
    name : &'a str,
}

pub const DEFAULT_CONFIG : Config<'static> = Config {
    name : "hello"
};