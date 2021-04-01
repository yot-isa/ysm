use super::Span;
use std::fmt;
use crate::reporter::{Diagnostic, Report, Reporter, Label, LabelStyle};

#[derive(Debug, PartialEq)]
pub enum Error {
    DigitInvalid { digit: char, span: Span },
    DigitExpected { span: Span },
    IdentifierExpected { span: Span },
}

impl Report for Error {
    fn report(&self, r: &Reporter) {
        match &self {
            Error::DigitInvalid { digit, span } => r.write(Diagnostic {
                message: format!("invalid digit `{}` in a data literal", digit),
                labels: vec![Label {
                    style: LabelStyle::Primary,
                    span: *span,
                    message: String::new(),
                }],
            }),
            Error::DigitExpected { span } => r.write(Diagnostic {
                message: "expected a digit".to_owned(),
                labels: vec![Label {
                    style: LabelStyle::Primary,
                    span: *span,
                    message: String::new(),
                }],
            }),
            Error::DataLiteralTooLarge { span } => r.write(Diagnostic {
                message: "data literal too large".to_owned(),
                labels: vec![Label {
                    style: LabelStyle::Primary,
                    span: *span,
                    message: String::new(),
                }],
            }),
            Error::IdentifierExpected { span } => r.write(Diagnostic {
                message: "expected an identifier".to_owned(),
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
        write!(f, "")
    }
}

impl std::error::Error for Error {}
