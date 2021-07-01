use std::sync::{self, Arc, Mutex};
use termion::event;
use {crate::configs, log::*, std::io, std::io::BufRead, tui::text::Span, tui::text::Spans};

use crate::{
    buffer::{self, Buffer},
    ui::UIState,
    UpdateResult,
};

use super::{events, ui};

pub fn update(
    ui_state: &mut UIState,
    events: &mut events::EventManager,
    config: &configs::Config,
    buffer: &mut Arc<Mutex<Buffer>>,
) -> Result<UpdateResult, Box<dyn std::error::Error>> {
    let mut result : Result<UpdateResult, Box<dyn std::error::Error>> = Ok(UpdateResult::None);
    while let Ok(event) = events.try_get_next() {
        result = match event {
            events::Event::Input(key) => {
                let mut buffer = buffer.lock().unwrap();
                handle_keyboard_input(key, ui_state, config, &mut buffer)
            }
            events::Event::Tick => {
                let buffer = buffer.lock().unwrap();
                update_ui_state_from_buffer(&buffer, ui_state);
                Ok(UpdateResult::DrawCall)
            }
            events::Event::FileUpdate(file) => {
                let mut buffer = buffer.lock().unwrap();
                for buffer_file in buffer.get_file_list() {
                    if buffer_file.url == file.url {
                        buffer.set_file(file);
                        break;
                    }
                }
                Ok(UpdateResult::DrawCall)
            }
            events::Event::FileRemove(file) => {
                let mut buffer = buffer.lock().unwrap();
                for buffer_file in buffer.get_file_list() {
                    if buffer_file.url == file.url {
                        if let Err(e) = buffer.remove_file(file) {
                            return Err(e);
                        }
                        break;
                    }
                }
                Ok(UpdateResult::DrawCall)
            }
            _ => {
                Ok(UpdateResult::DrawCall)
            }
        };
        if *result.as_ref().unwrap() == UpdateResult::DrawCall || *result.as_ref().unwrap() == UpdateResult::Quit {
            return result;
        }
    }
    result
}

fn handle_keyboard_input(
    key: configs::Key,
    ui_state: &mut ui::UIState,
    client_config: &configs::Config,
    buffer: &mut Buffer,
) -> Result<UpdateResult, Box<dyn std::error::Error>> {
    if key == client_config.key_map.quit {
        return Ok(UpdateResult::Quit);
    }
    if key == client_config.key_map.resize_left && ui_state.percent_size_of_panes.0 > 2 {
        ui_state.percent_size_of_panes.0 -= 2;
        ui_state.percent_size_of_panes.1 += 2;
    }
    if key == client_config.key_map.resize_right && ui_state.percent_size_of_panes.1 > 2 {
        ui_state.percent_size_of_panes.0 += 2;
        ui_state.percent_size_of_panes.1 -= 2;
    }
    if key == client_config.key_map.help {
        if let ui::UIMode::Help = ui_state.current_mode {
            ui_state.current_mode = ui_state.previous_mode;
        } else {
            ui_state.current_mode = ui::UIMode::Help;
        }
    }
    if key == client_config.key_map.menu {
        if let ui::UIMode::Menu = ui_state.current_mode {

            ui_state.current_mode = ui_state.previous_mode;
        } else {
            ui_state.current_mode = ui::UIMode::Menu;
        }
    }
    if key == client_config.key_map.reload_config {
        //let config = common::load_struct_toml::<ClientConfig>(Path::new(super::CONFIG_FILENAME));
        //*client_config = config;
        info!("Pressed reload config key");
    }
    if key == client_config.key_map.up || key == client_config.key_map.down {
        if key == client_config.key_map.up {
            // Highlight next item in list
            ui_state.sidebar_list.next();
        } else if key == client_config.key_map.down {
            // Highlight previous item in list
            ui_state.sidebar_list.previous();
        }
    }
    Ok(UpdateResult::DrawCall)
}

fn update_ui_state_from_buffer(buffer: &buffer::Buffer, ui_state: &mut ui::UIState) {
    // Index of currently selected item.
    let index = ui_state.sidebar_list.state.selected();
    let index = if index.is_none() {
        return;
    } else {
        index.unwrap()
    };
    // Set the name of the file as the title.
    ui_state.current_content_panel_title = ui_state.sidebar_list.items[index].display_name.clone();

    //If the newly selected item exists in the buffer, copy it into the ui_state struct.
    let url = ui_state.sidebar_list.items[index].url.clone();
    let mut file_exists_in_buffer: bool = false;
    for file in buffer.get_file_list() {
        if file.url == url {
            file_exists_in_buffer = true;
            let mut new = Vec::new();
            for s in &file.contents {
                new.push(Spans::from(Span::from(s.clone())));
            }
            ui_state.current_content = new;
            ui_state.current_content_panel_title = file.display_name.clone();
        }
    }
    if !file_exists_in_buffer {
        error!("File does not exist in buffer when it should.");
    }
}
