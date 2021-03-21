use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    CouldNotReadFile(PathBuf, io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::CouldNotReadFile(file_path, io_error) => write!(
                f,
                "couldn't read {}: {}",
                file_path.to_string_lossy(),
                io_error
            ),
        }
    }
}

impl std::error::Error for Error {}
