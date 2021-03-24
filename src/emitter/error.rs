use super::Span;
use crate::impl_spanning;
use std::fmt;

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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::LabelDefinedMoreThanOnce { label, .. } => write!(f, "label `{}` is defined more than once in this scope", label),
            Error::CannotFindLabel { label, .. } => write!(f, "cannot find label `{}` in this scope", label),
        }
    }
}

impl std::error::Error for Error {}
