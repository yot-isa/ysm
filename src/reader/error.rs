use std::fmt;
use std::io;
use std::path::PathBuf;
use crate::reporter::{Diagnostic, Report, Reporter};

#[derive(Debug)]
pub enum Error {
    CouldNotReadFile {
        file_path: PathBuf,
        io_error: io::Error
    },
}

impl Report for Error {
    fn report(&self, r: &Reporter) {
        match &self {
            Error::CouldNotReadFile { file_path, io_error } => r.write(Diagnostic {
                message: format!("couldn't read {}: {}", file_path.to_string_lossy(), io_error),
                labels: vec![],
            }),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for Error {}
