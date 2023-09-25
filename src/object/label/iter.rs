use super::Annotation;
use std::{
    fs::File,
    io,
    io::{BufReader, Cursor, Read},
    path::Path,
};

pub type AnnotationFromReaderIter<R> = csv::DeserializeRecordsIntoIter<R, Annotation>;
pub type AnnotationFromPathIter = AnnotationFromReaderIter<BufReader<File>>;
pub type AnnotationFromStrIter<'a> = AnnotationFromReaderIter<Cursor<&'a str>>;

pub fn iter_from_reader<R>(reader: R) -> AnnotationFromReaderIter<R>
where
    R: Read,
{
    let reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_reader(reader);
    reader.into_deserialize()
}

pub fn iter_from_path<P>(path: P) -> io::Result<AnnotationFromPathIter>
where
    P: AsRef<Path>,
{
    let reader = BufReader::new(File::open(path)?);
    Ok(iter_from_reader(reader))
}

pub fn iter_from_str(text: &str) -> AnnotationFromStrIter<'_> {
    let reader = Cursor::new(text);
    iter_from_reader(reader)
}
