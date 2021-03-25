use super::Span;
use crate::impl_spanning;
use std::fmt;
use crate::reporter::{Diagnostic, Report, Reporter};

#[derive(Debug, PartialEq)]
pub enum Error {
    LabelDefinedMoreThanOnce {
        label: String,
        current_label_span: Span,
        previously_defined_label_span: Span
    },
    CannotFindLabel {
        label: String,
        span: Span,
    },
}

impl Report for Error {
    fn report(&self, r: &Reporter) {
        match &self {
            Error::LabelDefinedMoreThanOnce { label, current_label_span, previously_defined_label_span } => r.write(Diagnostic {
                message: format!("label `{}` is defined more than once in this scope", label),
            }),
            Error::CannotFindLabel { label, span } => r.write(Diagnostic {
                message: format!("cannot find label `{}` in this scope", label),
            })
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::LabelDefinedMoreThanOnce { label, .. } => write!(f, "label `{}` is defined more than once in this scope", label),
            Error::CannotFindLabel { label, .. } => write!(f, "cannot find label `{}` in this scope", label),
        }
    }
}

impl std::error::Error for Error {}
