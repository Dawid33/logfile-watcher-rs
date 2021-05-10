use tui::widgets::*;
use tui::layout::*;
use tui::style::Color;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use common::json_structs::ClientUIConfig;

pub mod list;

pub struct UIState {
    pub current_file_path : Option<PathBuf>,
}
impl Default for UIState {
    fn default() -> Self{
        Self {
            current_file_path : None, 
        }
    }
}