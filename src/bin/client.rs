use discord_clone_lib::*;
// use config::*;

fn main() {
    let _config = if let Ok(c) = read_config() {
        c
    } else {
        config::DEFAULT_CONFIG
    };


}