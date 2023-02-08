use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(io::Error),
    #[error("CSV error: {0}")]
    Csv(csv::Error),
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
