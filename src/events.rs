#[cfg(unix)]
mod termion_backend {
    use termion::event::Key;
    pub use termion::input::TermRead;
    impl From<termion::event::Key> for super::Key {
        fn from(key: termion::event::Key) -> Self {
            match key {
                Key::Char(c) => super::Key::Char(c),
                Key::Alt(c) => super::Key::Alt(c),
                _ => super::Key::NotImplemented,
            }
        }
    }
}
#[cfg(unix)]
use termion_backend::*;

#[derive(Clone)]
pub struct Config {
    pub ui_refresh_rate: std::time::Duration,
    pub exit_key: Key,
}

pub enum Event {
    KeyPressed(Key),
    Tick,
}

/*
#[derive(Copy, Clone, PartialEq)]
pub enum UIEvent {
    QuitProgram,
    ReloadConfig,
}
*/

#[derive(Clone, PartialEq)]
pub enum Key {
    Char(char),
    Alt(char),
    Esc,
    NotImplemented,
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Key::Char(c) => format!("{}",c),
            _ => "Unknown".to_string(),
        };
        write!(f,"{}", result)
    }
}

pub struct EventManager {
    event_reciever: std::sync::mpsc::Receiver<Event>,
    //ui_event_sender: std::sync::mpsc::Sender<UIEvent>,
    //ui_event_handle: std::thread::JoinHandle<()>,
    input_handle: std::thread::JoinHandle<()>,
    tick_handle: std::thread::JoinHandle<()>,
    ui_refresh_rate: std::time::Duration,
    exit_key: Key,
}

impl EventManager {
    pub fn new(config: Config) -> Self {
        let (tx, rx) = std::sync::mpsc::channel::<Event>();
        let input_tx = tx.clone();
        let tick_tx = tx.clone();
        let tick_config = config.clone();
        let result_config = config.clone();
        //let (ui_tx, ui_rx) = std::sync::mpsc::channel::<UIEvent>();

        //Start threads to listen for events.
        /*
        let ui_event_handle = std::thread::spawn(move || {
            //Handle ui events
            loop {config.exit_key
        });
        */
        let input_handle = std::thread::spawn(move || {
            let exit_key = config.exit_key.clone();
            let stdin = std::io::stdin();
            if cfg!(unix) {
                for event in stdin.keys() {
                    let key: Key = event.unwrap().into();
                    let result = input_tx.send(Event::KeyPressed(key.clone()));
                    if result.is_err() || key == exit_key {
                        break;
                    }
                }
            }
        });
        let tick_handle =
            std::thread::spawn(move || {
                loop {
                    if tick_tx.send(Event::Tick).is_err() {
                        break;
                    }
                    std::thread::sleep(tick_config.ui_refresh_rate);
                }
            });
        Self {
            input_handle,
            tick_handle,
            //ui_event_handle,
            //ui_event_sender : ui_tx,
            event_reciever: rx,
            ui_refresh_rate: result_config.ui_refresh_rate,
            exit_key: result_config.exit_key,
        }
    }
    pub fn ui_refresh_rate(mut self, time: std::time::Duration) -> Self {
        self.ui_refresh_rate = time;
        self
    }
    pub fn next(&self) -> Result<Event, std::sync::mpsc::RecvError> {
        self.event_reciever.recv()
    }
}