use super::events::Event;
use chrono::prelude::*;
use log::*;
use serde::{Deserialize, Serialize};
use std::io::BufRead;
use std::time;
use std::sync;
use std::sync::Arc;
use std::sync::Mutex;
use tui::text::Spans;
use url::Url;

pub struct FileMonitor {
    should_exit: Arc<sync::atomic::AtomicBool>,
    thread_handle: std::thread::JoinHandle<()>,
    file_list: Arc<Mutex<super::buffer::FileList>>,
    buffer_update_counter: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct File {
    pub url: url::Url,
    pub display_name: String,
    pub contents: Vec<String>,
    pub last_modified: Option<chrono::DateTime<Utc>>,
}

impl FileMonitor {
    pub fn new(event_sender_handler: std::sync::mpsc::Sender<Event>) -> Self {
        let should_exit = sync::Arc::from(sync::atomic::AtomicBool::new(false));
        let file_list = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

        let file_watcher_handle = {
            let file_list: Arc<Mutex<super::buffer::FileList>> = file_list.clone();
            let should_exit = should_exit.clone();
            std::thread::spawn(move || loop {
                if should_exit.load(sync::atomic::Ordering::Relaxed) {
                    warn!("Exiting sender");
                    break;
                }

                let mut owned_file_list = file_list.lock().unwrap();
                for file in owned_file_list.iter_mut() {
                    if check_if_file_updated(file).unwrap() {
                        file.last_modified = Some(chrono::Utc::now());
                        match load_url(&file.url) {
                            Ok(output) => {
                                file.contents = output;
                                event_sender_handler.send(Event::FileUpdate(file.clone())).unwrap();
                            }
                            Err(e) => panic!("Cannot open url {}. {}", file.url.as_str(), e),
                        }
                    }
                }
                drop(owned_file_list);
                std::thread::sleep(std::time::Duration::from_millis(500));
            })
        };

        FileMonitor {
            thread_handle: file_watcher_handle,
            should_exit,
            file_list,
            buffer_update_counter: 0,
        }
    }
    pub fn update_file_list(&mut self, buffer: &super::buffer::Buffer) {
        //If the buffer has been updated, then update the file list.
        if buffer.update_counter > self.buffer_update_counter {
            *self.file_list.lock().unwrap() = (*buffer.get_file_list()).clone();
            self.buffer_update_counter = buffer.update_counter;
        }
    }
    pub fn exit(&mut self) {
        self.should_exit.store(true, sync::atomic::Ordering::SeqCst)
    }
}

fn check_if_file_updated(file : &File) -> Result<bool, std::io::Error> {
    if let Option::None = file.last_modified {
        return Ok(true);
    }
    match file.url.scheme() {
        "file" => {
            let file_handle = std::fs::File::open(file.url.path()).unwrap();
            let meta_data = file_handle.metadata().unwrap();
            let time = meta_data.modified().unwrap();
            let time = time.duration_since(time::SystemTime::UNIX_EPOCH).unwrap();
            if time.as_secs() > file.last_modified.unwrap().timestamp() as u64 {
                return Ok(true);
            }
        }
        _ => {
            error!(
                "Unknown url scheme {} in url {}",
                file.url.scheme(),
                file.url.as_str()
            );
            return Err(std::io::Error::from(std::io::ErrorKind::Other))
        }
    }
    Ok(false)
}

fn load_url(url: &url::Url) -> Result<Vec<String>, std::io::Error> {
    match url.scheme() {
        "file" => match read_file(url) {
            Ok(content) => Ok(content),
            Err(e) => {
                error!("Cannot read file path {}", url.as_str());
                Err(e)
            }
        },
        _ => {
            error!(
                "Unknown url scheme {} in url {}",
                url.scheme(),
                url.as_str()
            );
            Err(std::io::Error::from(std::io::ErrorKind::Other))
        }
    }
}

fn read_file(url: &url::Url) -> Result<Vec<String>, std::io::Error> {
    if let Ok(file_path) = url.to_file_path() {
        if let Ok(file_handle) = std::fs::File::open(file_path) {
            let mut buf_reader = std::io::BufReader::new(file_handle);
            let mut buffer = String::new();
            let mut output: Vec<String> = Vec::new();
            trace!("Reading file [{}]", url.as_str());
            while let Ok(x) = buf_reader.read_line(&mut buffer) {
                if x == 0 {
                    break;
                } else {
                    output.push(buffer.clone());
                    buffer.clear();
                }
            }
            Ok(output)
        } else {
            error!("Cannot open file using the url {}", url.as_str());
            return Err(std::io::Error::from(std::io::ErrorKind::Other));
        }
    } else {
        error!("Cannot turn the url [{}] to a file path.", url.as_str());
        return Err(std::io::Error::from(std::io::ErrorKind::Other));
    }
}
