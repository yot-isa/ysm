use super::Span;
use std::fmt;
use crate::reporter::{Diagnostic, Report, Reporter, Label, LabelStyle};

#[derive(Debug, PartialEq, Clone)]
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
    BinaryTooLarge {
        current_size: usize,
        requested_size: usize,
    },
}

impl Report for Error {
    fn report(&self, r: &Reporter) {
        match &self {
            Error::LabelDefinedMoreThanOnce { label, current_label_span, previously_defined_label_span } => r.write(Diagnostic {
                message: format!("label `{}` is defined multiple times", label),
                labels: vec![Label {
                    style: LabelStyle::Primary,
                    span: *current_label_span,
                    message: format!("`{}` redefined here", label),
                }, Label {
                    style: LabelStyle::Secondary,
                    span: *previously_defined_label_span,
                    message: format!("previous definition of the label `{}` here", label),
                }],
            }),
            Error::CannotFindLabel { label, span } => r.write(Diagnostic {
                message: format!("cannot find label `{}` in this scope", label),
                labels: vec![Label {
                    style: LabelStyle::Primary,
                    span: *span,
                    message: "not found in this scope".to_owned(),
                }],
            }),
            Error::BinaryTooLarge { current_size, requested_size } => r.write(Diagnostic {
                message: format!("binary of size {} does not fit within the requested size constraint of {}", current_size, requested_size),
                labels: vec![],
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
