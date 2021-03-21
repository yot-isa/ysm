use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    CouldNotWriteFile(PathBuf, io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::CouldNotWriteFile(file_path, io_error) => write!(
                f,
                "couldn't write {}: {}",
                file_path.to_string_lossy(),
                io_error
            ),
        }
    }
}

impl std::error::Error for Error {}
