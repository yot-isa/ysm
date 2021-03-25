use std::fmt;

#[derive(Debug)]
pub enum Error {
    YotTypeInvalid,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::YotTypeInvalid => write!(f,
                "invalid Yot type",
            ),
        }
    }
}

impl std::error::Error for Error {}
