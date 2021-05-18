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

#[derive(Copy, Clone)]
pub struct Config {
    pub ui_refresh_rate: std::time::Duration,
    pub exit_key: Key,
}

pub trait Event {
    fn into_box(self) -> Box<Self>
    where
        Self : Sized;
}

#[derive(Copy, Clone, PartialEq)]
pub enum Key {
    Char(char),
    Alt(char),
    Esc,
    NotImplemented,
}

#[derive(Default)]
pub struct Tick {}

impl Event for Key {
    fn into_box(self) -> Box<Key> {
        Box::new(self)
    }
}

impl Event for Tick {
    fn into_box(self) -> Box<Tick> {
        Box::new(self)
    }
}

impl Tick { 
    fn new() -> Self {
        Self {}
    }
}

pub struct EventManager {
    event_reciever: std::sync::mpsc::Receiver<Box<dyn Event>>,
    input_handle: std::thread::JoinHandle<()>,
    tick_handle: std::thread::JoinHandle<()>,
    ui_refresh_rate: std::time::Duration,
    exit_key: Key,
}

impl EventManager {
    pub fn new(config: Config) -> Self {
        let (tx, rx) = std::sync::mpsc::channel::<Box<dyn Event>>();
        let input_tx = tx.clone();
        let tick_tx = tx.clone();

        //Start threads to listen for events.
        let input_handle = std::thread::spawn(move || {
            let exit_key = config.exit_key.clone();
            let stdin = std::io::stdin();
            if cfg!(unix) {
                for event in stdin.keys() {
                    let key: Key = event.unwrap().into();
                    let result = input_tx.send(key.into_box());
                    if result.is_err() || key == exit_key {
                        break;
                    }
                }
            }
        });
        let tick_handle = {
            std::thread::spawn(move || {
                let config = &config.clone();
                loop {
                    if tick_tx.send(Tick::new().into_box()).is_err() {
                        break;
                    }
                    std::thread::sleep(config.ui_refresh_rate);
                }
            })
        };
        Self {
            input_handle,
            tick_handle,
            event_reciever: rx,
            ui_refresh_rate: config.ui_refresh_rate,
            exit_key: config.exit_key,
        }
    }
    pub fn ui_refresh_rate(mut self, time: std::time::Duration) -> Self {
        self.ui_refresh_rate = time;
        self
    }
    pub fn next(&self) -> Result<Box<dyn Event>, std::sync::mpsc::RecvError> {
        self.event_reciever.recv()
    }
}