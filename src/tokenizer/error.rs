use super::{Span, Spanned, Spanning};
use crate::impl_spanning;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidSymbol(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::InvalidSymbol(s) => write!(f, "invalid symbol `{}`", s),
        }
    }
}

impl std::error::Error for Error {}

impl_spanning!(Error);
