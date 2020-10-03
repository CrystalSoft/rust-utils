use std::fs::File;
use std::{io, io::Read};
use std::path::Path;
use std::str;

pub fn get_filename<S: AsRef<str>>(filename: S) -> Option<String> {
    match Path::new(filename.as_ref()).file_name() {
        Some(file_name) => match file_name.to_str() {
            Some(file_name) => Some(file_name.into()),
            None => None,
        },
        None => None,
    }
}

pub fn read_file_buf<S: AsRef<str>>(path: S) -> Result<Vec<u8>, io::Error> {
    let path = Path::new(path.as_ref());

    let mut buffer = Vec::new();
    File::open(&path)?.read_to_end(&mut buffer).map(|_| buffer)
}

pub fn read_file_string<S: AsRef<str>>(path: S) -> Result<String, io::Error> {
    let path = Path::new(path.as_ref());

    let mut buffer = String::new();
    File::open(&path)?.read_to_string(&mut buffer).map(|_| buffer)
}
