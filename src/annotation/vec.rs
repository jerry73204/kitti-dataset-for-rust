use crate::{annotation::Annotation, error::Error};
use std::{
    fs::File,
    io::{BufReader, Cursor, Read},
    path::Path,
};

pub fn vec_from_reader<R>(reader: R) -> Result<Vec<Annotation>, Error>
where
    R: Read,
{
    let reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_reader(reader);
    let result: Result<Vec<Annotation>, _> = reader.into_deserialize().collect();
    Ok(result?)
}

pub fn vec_from_path<P>(path: P) -> Result<Vec<Annotation>, Error>
where
    P: AsRef<Path>,
{
    let reader = BufReader::new(File::open(path)?);
    vec_from_reader(reader)
}

pub fn vec_from_str(text: &str) -> Result<Vec<Annotation>, Error> {
    let reader = Cursor::new(text);
    vec_from_reader(reader)
}
