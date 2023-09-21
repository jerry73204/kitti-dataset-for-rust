use std::{fmt, io};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(io::Error),

    #[error("format error: {0}")]
    Format(fmt::Error),

    #[error("CSV error: {0}")]
    Csv(csv::Error),

    #[error("invalid calib configuration: {0}")]
    InvalidCalibConfig(String),
}

impl From<fmt::Error> for Error {
    fn from(v: fmt::Error) -> Self {
        Self::Format(v)
    }
}

impl From<csv::Error> for Error {
    fn from(v: csv::Error) -> Self {
        Self::Csv(v)
    }
}

impl From<io::Error> for Error {
    fn from(v: io::Error) -> Self {
        Self::Io(v)
    }
}
