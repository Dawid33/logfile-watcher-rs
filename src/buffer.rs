use tui::text::Spans;

use crate::files::{File};

pub type FileList = Vec<File>;
pub struct Buffer {
    file_list : FileList,
    pub update_counter : u64,
}

impl Buffer{
    pub fn new(file_list : FileList) -> Self {
        Buffer {
            update_counter : 1,
            file_list
        }
    }
    pub fn get_file(&self) {

    }
    pub fn set_file(&mut self, new_file: File) {
        for file in &mut self.file_list {
            if file.url == new_file.url {
                *file = new_file.clone();
                break;
            }
        }
    }
    pub fn get_file_list(&self) -> &FileList{
        &self.file_list
    }
    pub fn rm_from_file_list(&mut self) {

        self.update_counter += 1;
    }
    pub fn add_to_file_list(&mut self, file : File) {
        self.file_list.push(file);
        self.update_counter += 1;
    }
}