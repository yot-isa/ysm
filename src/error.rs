use super::reader;
use super::tokenizer;
use super::emitter;
use super::writer;
use std::fmt;
use super::reporter::{Report, Reporter};

#[derive(Debug)]
pub enum Error {
    Reader(reader::Error),
    Tokenizer(tokenizer::Error),
    Emitter(emitter::Error),
    Writer(writer::Error),
}

impl From<reader::Error> for Error {
    fn from(error: reader::Error) -> Error {
        Error::Reader(error)
    }
}

impl From<tokenizer::Error> for Error {
    fn from(error: tokenizer::Error) -> Error {
        Error::Tokenizer(error)
    }
}

impl From<emitter::Error> for Error {
    fn from(error: emitter::Error) -> Error {
        Error::Emitter(error)
    }
}

impl From<writer::Error> for Error {
    fn from(error: writer::Error) -> Error {
        Error::Writer(error)
    }
}

impl Report for Error {
    fn report(&self, r: &Reporter) {
        match &self {
            Error::Reader(error) => error.report(r),
            Error::Tokenizer(error) => error.report(r),
            Error::Emitter(error) => error.report(r),
            Error::Writer(error) => error.report(r),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for Error {}
