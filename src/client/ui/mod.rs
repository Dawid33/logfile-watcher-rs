use super::draw;
use common::json_structs::ClientConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tui::layout::*;
use tui::style::Color;
use tui::text::Spans;
use tui::widgets::*;

pub mod list;

pub struct UIState<'a> {
    pub current_file_path: Option<PathBuf>,
    pub percent_size_of_panes: (u16, u16),
    pub background_color: Color,
    pub default_main_panel_title: String,
    pub debug: String,
    pub current_mode : UIMode,
    pub previous_mode : UIMode,
    pub sidebar_list : list::StatefulList<String>,
    pub content : Vec<Spans<'a>>,
}
#[derive(Copy,Clone)]
pub enum UIMode {
    Main,
    Help
}
impl UIState<'_> {
    pub fn load_from_client_config(mut self, config: &ClientConfig) -> Self {
        self.background_color = draw::rgb_tuple_to_color(&config.ui_config.background_color);
        self
    }
}
impl Default for UIState<'_> {
    fn default() -> Self {
        Self {
            current_file_path: None,
            percent_size_of_panes: (20, 80),
            background_color: Color::Black,
            debug: String::from("Debug"),
            default_main_panel_title: String::from("Viewer"),
            current_mode : UIMode::Main,
            previous_mode : UIMode::Main,
            sidebar_list : list::StatefulList::new(),
            content : Vec::new(),
        }
    }
}