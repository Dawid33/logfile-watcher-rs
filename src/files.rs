

pub struct FileMonitor<'a> {
    buffer : &'a super::buffer::Buffer,
    exit_sender : std::sync::mpsc::Sender<bool>,
    thread_handle : std::thread::JoinHandle<()>,
}

impl<'a> FileMonitor<'a> {
    pub fn new(buffer : &'a super::buffer::Buffer) -> Self {
        let (exit_tx,exit_rx) = std::sync::mpsc::channel::<bool>();
        let file_watcher_handle = {
            std::thread::spawn(move || loop {
                if let Ok(result) = exit_rx.try_recv() {
                    if result == true {
                        break;
                    }
                } else {
                    break;
                }
            })
        };

        FileMonitor {
            buffer,
            thread_handle : file_watcher_handle,
            exit_sender : exit_tx,
        }
    }
    pub fn exit(self) {
        self.exit_sender.send(true).unwrap();
    }
}