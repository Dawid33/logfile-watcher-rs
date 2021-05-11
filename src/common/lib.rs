use {
    log::*,
    serde::{Deserialize, Serialize},
    std::{
        fs,
        io,
        path::Path,
    },
};
use serde_json;

pub mod json_structs;

/// Try loading in struct from json file. If json file does not exist, load in
/// default values for the struct and create a new file with those values.
pub fn load_struct<T>(path: &Path) -> T
where
    for<'de> T: Deserialize<'de> + Serialize + Default,
{
    let json_read_result = try_read_json_file(path);
    match json_read_result {
        Ok(file) => {
            file
        }
        Err(e) => {
            warn!("Cannot load json file at [{}] because {}. Creating new json file with default values...", path.display(),e);
            let default = T::default();

            if let Err(_e) = try_write_json_file(&default, path) {
                warn!("Cannot create json file at [{}].", path.display());
                warn!("{}", e);
            }

            default
        }
    }
}

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
