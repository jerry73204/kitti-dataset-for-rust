use crate::annotation::Annotation;
use std::{
    fs::File,
    io,
    io::{BufReader, Cursor, Read},
    path::Path,
};

pub type AnnotationIter<R> = csv::DeserializeRecordsIntoIter<R, Annotation>;
pub type AnnotationPathIter = AnnotationIter<BufReader<File>>;
pub type AnnotationStrIter<'a> = AnnotationIter<Cursor<&'a str>>;

pub fn from_reader<R>(reader: R) -> AnnotationIter<R>
where
    R: Read,
{
    let reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_reader(reader);
    reader.into_deserialize()
}

pub fn from_path<P>(path: P) -> Result<AnnotationPathIter, io::Error>
where
    P: AsRef<Path>,
{
    let reader = BufReader::new(File::open(path)?);
    Ok(from_reader(reader))
}

pub fn from_str(text: &str) -> AnnotationStrIter<'_> {
    let reader = Cursor::new(text);
    from_reader(reader)
}
