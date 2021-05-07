pub mod config;
use serde_json;
use config::*;

pub fn read_config<'a>() -> Result<Config<'a>,ConfigError> {

    Err(ConfigError::new(ConfigErrorKind::FileReadError))
}

pub fn write_config(config : &Config,path : &std::path::Path) -> serde_json::Result<()> {
    let json_config = serde_json::json!(config);
    let json_string = serde_json::to_string(&json_config)?;
    
    if let Err(e)  = std::fs::write(path, json_string) {
        Err(serde_json::Error::io(e))
    } else {
        Ok(())
    }
}