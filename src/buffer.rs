use tui::{buffer, text::Spans};

use crate::files::File;

pub type FileList = Vec<File>;
pub struct Buffer {
    file_list: FileList,
    pub update_counter: u64,
}
#[derive(Debug)]
struct BufferError(String);
impl std::fmt::Display for BufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}
impl std::error::Error for BufferError {

}

impl Buffer {
    pub fn new(file_list: FileList) -> Self {
        Buffer {
            update_counter: 1,
            file_list,
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
    }
    pub fn get_file_list(&self) -> &FileList {
        &self.file_list
    }
    pub fn remove_file(&mut self, file : File) -> Result<(), Box<dyn std::error::Error>> {
        self.update_counter += 1;
        for buffer_file in &self.file_list {
            if buffer_file.url == file.url {

            }
        }
        Ok(())
    }
    pub fn add_file(&mut self, file: File) {
        self.file_list.push(file);
        self.update_counter += 1;
    }
}
