use super::Oxts;
use crate::error::Error;
use std::{
    fs::File,
    io::{BufReader, Cursor, Read},
    path::Path,
};

pub fn vec_from_reader<R>(reader: R) -> Result<Vec<Oxts>, Error>
where
    R: Read,
{
    let reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_reader(reader);
    let result: Result<Vec<Oxts>, _> = reader.into_deserialize().collect();
    Ok(result?)
}

pub fn vec_from_path<P>(path: P) -> Result<Vec<Oxts>, Error>
where
    P: AsRef<Path>,
{
    let reader = BufReader::new(File::open(path)?);
    vec_from_reader(reader)
}

pub fn vec_from_str(text: &str) -> Result<Vec<Oxts>, Error> {
    let reader = Cursor::new(text);
    vec_from_reader(reader)
}
