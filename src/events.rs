use {
    super::common::configs::*,
    std::{
        io,
        sync::{
            atomic::{AtomicBool, Ordering},
            mpsc, Arc,
        },
        thread,
        time::Duration,
    },
};

use std::str::FromStr;

#[cfg(unix)]
use termion::input::TermRead;
use url::Url;

use crate::files::File;

pub enum Event {
    Input(Key),
    Tick,
    FileUpdate(File),
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct EventManager{
    pub tx: mpsc::Sender<Event>,
    pub file_monitor: super::files::FileMonitor,
    rx: mpsc::Receiver<Event>,
    input_handle: thread::JoinHandle<()>,
    tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            exit_key: Key::Char('q'),
            tick_rate: Duration::from_millis(100),
        }
    }
}

impl EventManager{
    #[allow(dead_code)]
    pub fn new() -> EventManager {
        EventManager::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> EventManager {
        let (tx, rx) = mpsc::channel::<Event>();
        let input_handle = {
            let tx = tx.clone();
            
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(raw_key) = evt {
                        let key: Key = raw_key.into();
                        if let Err(err) = tx.send(Event::Input(key.into())) {
                            eprintln!("{}", err);
                            return;
                        }
                        if key == config.exit_key {
                            return;
                        }
                    }
                }
            })
        };
        let tick_handle = {
            let tx = tx.clone();
            thread::spawn(move || loop {
                if tx.send(Event::Tick).is_err() {
                    break;
                }
                thread::sleep(config.tick_rate);
            })
        };
        let file_monitor = super::files::FileMonitor::new(tx.clone());
        EventManager {
            tx,
            rx,
            input_handle,
            tick_handle,
            file_monitor,
        }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}

impl<'a> Drop for EventManager {
    fn drop(&mut self) {
        self.file_monitor.exit();
    }
}