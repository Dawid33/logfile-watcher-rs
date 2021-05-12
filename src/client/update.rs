use {common::configs::*, tui::text::Spans,tui::text::Span,std::io,std::io::{BufRead}, log::*};

use super::{events, ui};

/**
 * ## update_client
 *
 *- ` events` is the event handler.
 *- `client_config` is mutable in case the user wants to reload
 *  the config file during runtime. \\
 *- `ui_state` records ui_state for the draw_client() function \\
 *- (bool,bool) The first bool in the output tells the caller function
 *  whether or not to exit the application. The second bool determines whether
 *  or not to update the screen.
 * Update the `ui_state` based on input from `events`.
 */
pub fn update_client(
    events: &mut events::Events,
    client_config: &mut ClientConfig,
    ui_state: &mut ui::UIState,
) -> Result<(bool,bool), Box<dyn std::error::Error>> {
    match events.next()? {
        events::Event::Input(key) => {
            return handle_keyboard_input(key, ui_state, client_config);
        },
        events::Event::Tick => {
            return Ok((true,true));
        },
    }
}

fn handle_keyboard_input(key : common::configs::Key, ui_state : &mut ui::UIState, client_config : &mut common::configs::ClientConfig) -> Result<(bool,bool), Box<dyn std::error::Error>>{
    if key == client_config.key_map.quit {
        return Ok((false,false));
    }
    if key == client_config.key_map.resize_left && ui_state.percent_size_of_panes.0 > 2 {
        ui_state.percent_size_of_panes.0 -= 2;
        ui_state.percent_size_of_panes.1 += 2;
    }
    if key == client_config.key_map.resize_right && ui_state.percent_size_of_panes.1 > 2
    {
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
    if key == client_config.key_map.reload_config {
        //let config = common::load_struct_toml::<ClientConfig>(Path::new(super::CONFIG_FILENAME));
        //*client_config = config;
    }
    if key == client_config.key_map.up || key == client_config.key_map.down{
        if key == client_config.key_map.up{
            // Highlight next item in list
            ui_state.sidebar_list.next();
        } else if key == client_config.key_map.down {
            // Highlight previous item in list
            ui_state.sidebar_list.previous();
        }

        // Index of currently selected item.
        let index = ui_state.sidebar_list.state.selected().unwrap();
        // Url of the currently selected list item.
        let url = ui_state.sidebar_list.items[index].0.clone();
        // Set the name of the file as the title.
        ui_state.current_content_panel_title = String::from(url.path());

        
        match load_url(&url) {
            Ok(output) => {
                let mut new_output : Vec<Spans> = Vec::new();
                for s in output {
                    new_output.push(Spans::from(Span::from(s)));
                }
                ui_state.content.clear();
                ui_state.content = new_output.clone();
                trace!("File [{}] successfully added to ui_state.content.", url.as_str());
            }
            Err(e) => error!("Cannot open url {}. {}", url.as_str(), e)
        }
    }
    Ok((true, false))
}

fn load_url(url : &url::Url) -> Result<Vec<String>, io::Error>{
    match url.scheme() {
        "file" => {
            match read_file(url) {
                Ok(content) => Ok(content),
                Err(e) => {
                    error!("Cannot read file path {}", url.as_str());
                    Err(e)
                }
            }    
        }
        _ => {
            error!("Unknown url scheme {} in url {}",url.scheme(),url.as_str());
            Err(io::Error::from(io::ErrorKind::Other))
        }
    }   
}

fn read_file(url : &url::Url) -> Result<Vec<String>,io::Error>{
    if let Ok(file_path) = url.to_file_path() {                  
        if let Ok(file_handle) = std::fs::File::open(file_path) {
            let mut buf_reader = std::io::BufReader::new(file_handle);
            let mut buffer = String::new();
            let mut output : Vec<String> = Vec::new();
            trace!("Reading file [{}]", url.as_str());
            while let Ok(x) = buf_reader.read_line(&mut buffer) {
                if x == 0 {
                    break
                } else {
                    output.push(buffer.clone());
                    buffer.clear();
                }
            }
            Ok(output)
        } else {
            error!("Cannot open file using the url {}", url.as_str());
            return Err(io::Error::from(io::ErrorKind::Other))
        }
    } else {
        error!("Cannot turn the url [{}] to a file path.", url.as_str());
        return Err(io::Error::from(io::ErrorKind::Other))
    }
}