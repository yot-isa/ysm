use super::Span;
use std::fmt;
use crate::reporter::{Diagnostic, Report, Reporter, Label, LabelStyle};

#[derive(Debug, PartialEq)]
pub enum Error {
    OpeningBraceUnclosed { span: Span },
    ClosingBraceUnexpected { span: Span },
}

impl Report for Error {
    fn report(&self, r: &Reporter) {
        match &self {
            Error::OpeningBraceUnclosed { span } => r.write(Diagnostic {
                message: "unclosed opening brace".to_owned(),
                labels: vec![Label {
                    style: LabelStyle::Primary,
                    span: *span,
                    message: String::new(),
                }],
            }),
            Error::ClosingBraceUnexpected { span } => r.write(Diagnostic {
                message: "unexpected closing brace".to_owned(),
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
