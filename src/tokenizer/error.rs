use super::Span;
use crate::impl_spanning;
use std::fmt;
use crate::reporter::{Diagnostic, Report, Reporter, Label, LabelStyle};

#[derive(Debug, PartialEq)]
pub enum Error {
    SymbolInvalid { symbol: String, span: Span },
    DigitInvalid { digit: char, span: Span },
    DigitExpected { span: Span },
    DataLiteralTooLarge { span: Span },
    IdentifierExpected { span: Span },
}

impl Report for Error {
    fn report(&self, r: &Reporter) {
        match &self {
            Error::SymbolInvalid { symbol, span } => r.write(Diagnostic {
                message: format!("invalid symbol `{}`", symbol),
                labels: vec![Label {
                    style: LabelStyle::Primary,
                    span: *span,
                    message: String::new(),
                }],
            }),
            Error::DigitInvalid { digit, span } => r.write(Diagnostic {
                message: format!("invalid digit `{}` in a data literal", digit),
                labels: vec![Label {
                    style: LabelStyle::Primary,
                    span: *span,
                    message: String::new(),
                }],
            }),
            Error::DigitExpected { span } => r.write(Diagnostic {
                message: format!("expected a digit"),
                labels: vec![Label {
                    style: LabelStyle::Primary,
                    span: *span,
                    message: String::new(),
                }],
            }),
            Error::DataLiteralTooLarge { span } => r.write(Diagnostic {
                message: format!("data literal too large"),
                labels: vec![Label {
                    style: LabelStyle::Primary,
                    span: *span,
                    message: String::new(),
                }],
            }),
            Error::IdentifierExpected { span } => r.write(Diagnostic {
                message: format!("expected an identifier"),
                labels: vec![Label {
                    style: LabelStyle::Primary,
                    span: *span,
                    message: String::new(),
                }],
            }),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::SymbolInvalid { symbol, .. } => write!(f, "invalid symbol `{}`", symbol),
            Error::DigitInvalid { digit, .. }  => write!(f, "invalid digit `{}` in a data literal", digit),
            Error::DigitExpected { .. } => write!(f, "expected a digit"),
            Error::DataLiteralTooLarge { .. } => write!(f, "data literal is too large"),
            Error::IdentifierExpected { .. } => write!(f, "expected an identifier"),
        }
    }
}

impl std::error::Error for Error {}
