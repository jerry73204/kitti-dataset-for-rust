use crate::annotation::Annotation;
use std::{
    fs::File,
    io::{BufReader, Cursor, Read},
    path::Path,
};

pub fn from_reader<R>(reader: R) -> Result<Vec<Annotation>, csv::Error>
where
    R: Read,
{
    let reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_reader(reader);
    reader.into_deserialize().collect()
}

pub fn from_path<P>(path: P) -> Result<Vec<Annotation>, crate::error::Error>
where
    P: AsRef<Path>,
{
    let reader = BufReader::new(File::open(path)?);
    Ok(from_reader(reader)?)
}

pub fn from_str(text: &str) -> Result<Vec<Annotation>, csv::Error> {
    let reader = Cursor::new(text);
    from_reader(reader)
}
