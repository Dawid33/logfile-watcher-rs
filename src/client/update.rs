use {common::json_structs::ClientConfig, std::path::Path, tui::text::Spans};

use super::{events, ui};

/**
 * ## update_client
 *
 *- ` events` is the event handler.
 *- `client_config` is mutable in case the user wants to reload
 *  the config file during runtime. \\
 *- `ui_state` records ui_state for the draw_client() function \\
 *
 * Update the `ui_state` based on input from `events`.
 */
pub fn update_client(
    events: &mut events::Events,
    client_config: &mut ClientConfig,
    ui_state: &mut ui::UIState,
) -> Result<bool, Box<dyn std::error::Error>> {
    if let events::Event::Input(key) = events.next()? {
        if key == client_config.key_map.quit.into() {
            return Ok(false);
        }
        if key == client_config.key_map.resize_left.into() && ui_state.percent_size_of_panes.0 > 2 {
            ui_state.percent_size_of_panes.0 -= 2;
            ui_state.percent_size_of_panes.1 += 2;
        }
        if key == client_config.key_map.resize_right.into() && ui_state.percent_size_of_panes.1 > 2
        {
            ui_state.percent_size_of_panes.0 += 2;
            ui_state.percent_size_of_panes.1 -= 2;
        }
        if key == client_config.key_map.help.into() {
            if let ui::UIMode::Help = ui_state.current_mode {
                ui_state.current_mode = ui_state.previous_mode;
            } else {
                ui_state.current_mode = ui::UIMode::Help;
            }
        }
        if key == client_config.key_map.reload_config.into() {
            let config = common::load_struct::<ClientConfig>(Path::new(super::CONFIG_FILENAME));
            *client_config = config;
        }
        if key == client_config.key_map.up.into() {
            ui_state.sidebar_list.next();
            ui_state.debug = ui_state.sidebar_list.items
                [ui_state.sidebar_list.state.selected().unwrap()]
            .clone();
            ui_state.content.clear();
            ui_state.content.push(Spans::from(
                ui_state.sidebar_list.items[ui_state.sidebar_list.state.selected().unwrap()]
                    .clone(),
            ));
        }
        if key == client_config.key_map.down.into() {
            ui_state.sidebar_list.previous();
            ui_state.debug = ui_state.sidebar_list.items
                [ui_state.sidebar_list.state.selected().unwrap()]
            .clone();
            ui_state.content.clear();
            ui_state.content.push(Spans::from(
                ui_state.sidebar_list.items[ui_state.sidebar_list.state.selected().unwrap()]
                    .clone(),
            ));
        }
    }
    Ok(true)
}
