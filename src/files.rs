use crate::buffer;
use crate::configs;
use crate::events;
use chrono::prelude::*;
use log::*;
use serde::{Deserialize, Serialize};
use std::io::BufRead;
use std::sync;
use std::sync::Arc;
use std::sync::Mutex;
use std::time;
use tui::text::Spans;
use url::Url;

pub struct FileMonitor {
    should_exit: Arc<sync::atomic::AtomicBool>,
    thread_handle: std::thread::JoinHandle<()>,
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
    pub fn new(
        event_sender_handler: std::sync::mpsc::Sender<events::Event>,
        config: &configs::Config,
        buffer: Arc<Mutex<buffer::Buffer>>,
    ) -> Self {
        let should_exit = sync::Arc::from(sync::atomic::AtomicBool::new(false));
        let mut unlocked_buffer = buffer.lock().unwrap();
        let mut file_list: crate::buffer::FileList = (*unlocked_buffer.get_file_list()).clone();
        let mut buffer_recv = unlocked_buffer.update_bus.add_rx();
        drop(unlocked_buffer);

        let file_watcher_handle = {
            let should_exit = should_exit.clone();
            let mut old_time = time::SystemTime::now();
            let force_update_refresh_rate =
            std::time::Duration::from_millis(config.force_update_miliseconds);

            
            std::thread::spawn(move || loop {
                if should_exit.load(sync::atomic::Ordering::Relaxed) {
                    trace!("Exiting Filelist thread");
                    break;
                }
                
                let force_update_all =
                    if old_time + force_update_refresh_rate < time::SystemTime::now() {
                        old_time = time::SystemTime::now();
                        false
                    } else {
                        false
                    };
                    
                for file in file_list.iter_mut() {                    
                    let modified : bool = check_has_been_modified(file).unwrap();
                    if force_update_all || modified {
                        file.last_modified = Some(chrono::Utc::now());
                        match load_url(&file.url) {
                            Ok(output) => {
                                file.contents = output;
                                event_sender_handler
                                    .send(events::Event::FileUpdate(file.clone()))
                                    .unwrap();
                            }
                            Err(e) => panic!("Cannot open url {}. {}", file.url.as_str(), e),
                        }
                    }
                }
                
                if let Ok(event) = buffer_recv.try_recv() {
                    match event {
                        buffer::BufferUpdateEvent::FullUpdate => {
                            let unlocked_buffer = buffer.lock().unwrap();
                            file_list = (*unlocked_buffer.get_file_list()).clone();
                        }
                    }
                }
                
                std::thread::sleep(time::Duration::from_millis(100));
            })
        };

        FileMonitor {
            thread_handle: file_watcher_handle,
            should_exit,
            buffer_update_counter: 0,
        }
    }
    /*
    pub fn update_file_list(&mut self, buffer: &super::buffer::Buffer) {
        *self.file_list.lock().unwrap() = (*buffer.get_file_list()).clone();
    }
    */
    pub fn exit(&mut self) {
        self.should_exit.store(true, sync::atomic::Ordering::SeqCst)
    }
}

fn check_has_been_modified(file: &File) -> Result<bool, std::io::Error> {
    if let Option::None = file.last_modified {
        return Ok(true);
    }
    match file.url.scheme() {
        "file" => {
            let file_handle = std::fs::File::open(file.url.path()).unwrap();
            file_handle.sync_all().unwrap();
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
            return Err(std::io::Error::from(std::io::ErrorKind::Other));
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
            file_handle.sync_all().unwrap();
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
