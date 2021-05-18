#![allow(dead_code)]
#![recursion_limit = "16"]

use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
//use log::*;

#[cfg(unix)]
type Backend = tui::backend::TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>;
#[cfg(windows)]
type Backend = tui::backend::CrosstermBackend<std::io::Stdout>;

pub mod events;
pub mod ui;
pub mod serde;
pub mod load;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = std::io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;
    terminal.clear()?;

    //Create struct that can contain the entire state of the ui.
    let mut ui_state: Box<dyn ui::UIState> =
        Box::new(ui::UIMainState::default());
    let mut previous_ui_state: Box<dyn ui::UIState> =
        Box::new(ui::UIMainState::default());

    let config = events::Config {
        exit_key: events::Key::Char('q'),
        ui_refresh_rate: std::time::Duration::from_millis(50),
    };
    let event_manager = events::EventManager::new(config);
    let config = serde::Config::default();

    loop {
        let event = event_manager.next()?;
        match ui_state.update(&mut terminal, &event, &config)? {
            ui::UpdateResult::GoToPreviousUI => {
                let temp = previous_ui_state.clone();
                previous_ui_state = temp;
                ui_state = previous_ui_state.clone();
            }
            ui::UpdateResult::ReplaceUIWith(ui) => {
                previous_ui_state = ui_state;
                ui_state = ui;
            }
            ui::UpdateResult::DoNothing => (),
        }

        match event {
            events::Event::KeyPressed(key) => {
                if key == config.key_map.exit_key {
                    break;
                }
            }
            events::Event::Tick => {
                terminal
                    .draw(|frame| {
                        ui_state.draw(frame, &config);
                    })
                    .unwrap();
            }
        }
    }
    terminal.clear()?;
    Ok(())
}
