use super::span::{Span, Spanned, Spanning};
use super::tokenizer;
use crate::impl_spanning;
use std::fmt;
use std::io;

pub struct Error {
    pub kind: ErrorKind,
    pub span: Option<Span>,
}

pub enum ErrorKind {
    Io(io::Error),
    Tokenizer(tokenizer::error::Error),
}

impl_spanning!(Error);

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error {
            kind: ErrorKind::Io(error),
            span: None,
        }
    }
}

impl From<Spanned<tokenizer::error::Error>> for Error {
    fn from(error: Spanned<tokenizer::error::Error>) -> Error {
        Error {
            kind: ErrorKind::Tokenizer(error.node),
            span: Some(error.span),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::Io(inner) => write!(f, "{}", inner),
            ErrorKind::Tokenizer(inner) => write!(f, "{}", inner),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::Io(inner) => write!(f, "{}", inner),
            ErrorKind::Tokenizer(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::Io(inner) => Some(inner),
            ErrorKind::Tokenizer(inner) => Some(inner),
        }
    }
}
