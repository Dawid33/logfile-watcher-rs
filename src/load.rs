use log::*;
use std::io;
use std::io::BufRead;

pub fn load_url(url: &url::Url) -> Result<Vec<String>, io::Error> {
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
            Err(io::Error::from(io::ErrorKind::Other))
        }
    }
}

fn read_file(url: &url::Url) -> Result<Vec<String>, io::Error> {
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
            return Err(io::Error::from(io::ErrorKind::Other));
        }
    } else {
        error!("Cannot turn the url [{}] to a file path.", url.as_str());
        return Err(io::Error::from(io::ErrorKind::Other));
    }
}
