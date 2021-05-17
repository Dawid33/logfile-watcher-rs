#![allow(dead_code)]
#![recursion_limit = "16"]

use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
//use log::*;

#[cfg(unix)]
type Backend = tui::backend::TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>;
#[cfg(windows)]
type Backend = tui::backend::CrosstermBackend<std::io::Stdout>;

pub mod ui;
pub mod events;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let stdout = std::io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;
    terminal.clear()?;

    //Create struct that can contain the entire state of the ui.
    let mut ui_state : Box<dyn ui::state::UIState<Backend>> = Box::new(ui::state::UIMainState::default());
    
    let config = events::Config {
        exit_key : events::Key::Char('q'),
        ui_refresh_rate : std::time::Duration::from_millis(200),
    };
    let event_manager = events::EventManager::new(config);

    loop {
        let event = event_manager.next()?;
        match ui_state.update(&mut terminal, &event)? {
            ui::state::UpdateResult::ReplaceUIWith(ui) => {
                ui_state = ui;
            },
            ui::state::UpdateResult::DoNothing => ()
        }

        match event {
            events::Event::KeyPressed(key) => {
                match key {
                    events::Key::Char(c) => {
                        if c == 'q' {
                            break;
                        }
                    }
                    _ => {
                        
                    }
                }
               
            }
            events::Event::Tick => {
                terminal.draw(|frame| {
                    ui_state.draw(frame);
                }).unwrap();
            }
        }
    }
    terminal.clear()?;
    Ok(())
}
