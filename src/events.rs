use crate::buffer;
use crate::configs;
use crate::files;
use std::str::FromStr;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc, Mutex,
};
use std::{io, thread, time::Duration};
use url::Url;

#[cfg(unix)]
use termion::input::TermRead;

pub enum Event {
    Input(configs::Key),
    Tick,
    FileUpdate(files::File),
    FileRemove(files::File),
    Quit,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct EventManager {
    pub tx: mpsc::Sender<Event>,
    pub file_monitor: super::files::FileMonitor,
    rx: mpsc::Receiver<Event>,
    input_handle: thread::JoinHandle<()>,
    tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: configs::Key,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            exit_key: configs::Key::Char('q'),
        }
    }
}

impl EventManager {
    pub fn new(
        config: Config,
        client_config: configs::Config,
        buffer: Arc<Mutex<buffer::Buffer>>,
    ) -> EventManager {
        let (tx, rx) = mpsc::channel::<Event>();
        let file_monitor = files::FileMonitor::new(tx.clone(), &client_config, buffer);
        let input_handle = {
            let tx = tx.clone();

            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(raw_key) = evt {
                        let key: configs::Key = raw_key.into();
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
            let tick_rate =
                std::time::Duration::from_millis(client_config.refersh_rate_miliseconds);
            thread::spawn(move || loop {
                if tx.send(Event::Tick).is_err() {
                    break;
                }
                thread::sleep(tick_rate);
            })
        };
        EventManager {
            tx,
            rx,
            input_handle,
            tick_handle,
            file_monitor,
        }
    }

    pub fn try_get_next(&self) -> Result<Event, mpsc::TryRecvError> {
        self.rx.try_recv()
    }
}

impl<'a> Drop for EventManager {
    fn drop(&mut self) {
        self.tx.send(Event::Quit).unwrap();
        self.file_monitor.exit();
    }
}
