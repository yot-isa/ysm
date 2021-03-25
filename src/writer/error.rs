use std::fmt;
use std::io;
use std::path::PathBuf;
use crate::reporter::{Diagnostic, Report, Reporter, Label, LabelStyle};

#[derive(Debug)]
pub enum Error {
    CouldNotWriteFile { file_path: PathBuf, io_error: io::Error },
}

impl Report for Error {
    fn report(&self, r: &Reporter) {
        match &self {
            Error::CouldNotWriteFile { file_path, io_error } => r.write(Diagnostic {
                message: format!("couldn't write {}: {}", file_path.to_string_lossy(), io_error),
                labels: vec![],
            }),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::CouldNotWriteFile { file_path, io_error } => write!(
                f,
                "couldn't write {}: {}",
                file_path.to_string_lossy(),
                io_error
            ),
        }
    }
}

impl std::error::Error for Error {}
