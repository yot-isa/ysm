use super::Span;
use crate::impl_spanning;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    SymbolInvalid(String, Span),
    DigitInvalid(char, Span),
    DigitExpected(Span),
    DataLiteralTooLarge(Span),
    IdentifierExpected(Span),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::SymbolInvalid(s, _) => write!(f, "invalid symbol `{}`", s),
            Error::DigitInvalid(c, _) => write!(f, "invalid digit `{}` in a data literal", c),
            Error::DigitExpected(_) => write!(f, "expected a digit"),
            Error::DataLiteralTooLarge(_) => write!(f, "data literal is too large"),
            Error::IdentifierExpected(_) => write!(f, "expected an identifier"),
        }
    }
}

impl std::error::Error for Error {}
