use tui::widgets::*;
use tui::layout::*;
use tui::style::Color;
use serde::{Deserialize, Serialize};
use common::json_structs::ClientUIConfig;

pub mod list;
pub struct DrawConfig {
    pub current_file_name : String,
    pub client_ui_config : Option<ClientUIConfig>,
}

impl Default for DrawConfig {
    fn default() -> Self {
        Self {
            current_file_name : String::from("Viewer"),
            client_ui_config : None,
        }
    }
}

impl DrawConfig {
    pub fn set_client_config(mut self, config : ClientUIConfig) -> Self {
        self.client_ui_config = Some(config);
        self
    }
}

/*
pub struct UI<'a> {
    pub main_panel : UIElement<Block<'a>>,
}

pub struct UIElement<W : Widget> {
    pub widget : W,
    pub layout : Layout,
}

impl UI<'_> {
    pub fn new(size : &Rect) -> Self {

        Self {
            main_panel : UIElement {
                widget : Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
                layout : Layout::default()
            }
        }
    }
}
*/