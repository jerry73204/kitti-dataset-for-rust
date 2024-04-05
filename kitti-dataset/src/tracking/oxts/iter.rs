use super::Oxts;
use std::{
    fs::File,
    io,
    io::{BufReader, Cursor, Read},
    path::Path,
};

pub type OxtsFromReaderIter<R> = csv::DeserializeRecordsIntoIter<R, Oxts>;
pub type OxtsFromPathIter = OxtsFromReaderIter<BufReader<File>>;
pub type OxtsFromStrIter<'a> = OxtsFromReaderIter<Cursor<&'a str>>;

impl Oxts {
    pub fn iter_from_reader<R>(reader: R) -> OxtsFromReaderIter<R>
    where
        R: Read,
    {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b' ')
            .from_reader(reader);
        reader.into_deserialize()
    }

    pub fn iter_from_path<P>(path: P) -> io::Result<OxtsFromPathIter>
    where
        P: AsRef<Path>,
    {
        let reader = BufReader::new(File::open(path)?);
        Ok(Self::iter_from_reader(reader))
    }

    pub fn iter_from_str(text: &str) -> OxtsFromStrIter<'_> {
        let reader = Cursor::new(text);
        Self::iter_from_reader(reader)
    }
}
