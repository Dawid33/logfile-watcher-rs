use crate::files::File;
use tui::{buffer, text::Spans};

pub type FileList = Vec<File>;
pub struct Buffer {
    pub update_bus: bus::Bus<BufferUpdateEvent>,
    file_list: FileList,
}
/*
#[derive(Debug)]
struct BufferError(String);
impl std::fmt::Display for BufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}
impl std::error::Error for BufferError {
}
*/
#[derive(Clone)]
pub enum BufferUpdateEvent {
    FullUpdate,
}

impl Buffer {
    pub fn new(file_list: FileList) -> Self {
        let update_bus: bus::Bus<BufferUpdateEvent> = bus::Bus::new(10);
        Buffer {
            file_list,
            update_bus,
        }
    }
    pub fn get_file(&self) {}
    pub fn set_file(&mut self, new_file: File) {
        for file in &mut self.file_list {
            if file.url == new_file.url {
                *file = new_file.clone();
                break;
            }
        }
        self.update_bus.broadcast(BufferUpdateEvent::FullUpdate);
    }
    pub fn get_file_list(&self) -> &FileList {
        &self.file_list
    }
    pub fn remove_file(&mut self, file: File) -> Result<(), Box<dyn std::error::Error>> {
        for buffer_file in &self.file_list {
            if buffer_file.url == file.url {}
        }
        self.update_bus.broadcast(BufferUpdateEvent::FullUpdate);
        Ok(())
    }
    pub fn add_file(&mut self, file: File) {
        self.file_list.push(file);
        self.update_bus.broadcast(BufferUpdateEvent::FullUpdate);
    }
}
