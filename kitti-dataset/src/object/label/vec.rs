use super::Label;
use crate::error::Error;
use std::{
    fs::File,
    io::{BufReader, Cursor, Read},
    path::Path,
};

impl Label {
    pub fn vec_from_reader<R>(reader: R) -> Result<Vec<Label>, Error>
    where
        R: Read,
    {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b' ')
            .from_reader(reader);
        let result: Result<Vec<Label>, _> = reader.into_deserialize().collect();
        Ok(result?)
    }

    pub fn vec_from_path<P>(path: P) -> Result<Vec<Label>, Error>
    where
        P: AsRef<Path>,
    {
        let reader = BufReader::new(File::open(path)?);
        Self::vec_from_reader(reader)
    }

    pub fn vec_from_str(text: &str) -> Result<Vec<Label>, Error> {
        let reader = Cursor::new(text);
        Self::vec_from_reader(reader)
    }
}
