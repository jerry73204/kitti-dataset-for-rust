use super::Label;
use std::{
    fs::File,
    io,
    io::{BufReader, Cursor, Read},
    path::Path,
};

pub type LabelFromReaderIter<R> = csv::DeserializeRecordsIntoIter<R, Label>;
pub type LabelFromPathIter = LabelFromReaderIter<BufReader<File>>;
pub type LabelFromStrIter<'a> = LabelFromReaderIter<Cursor<&'a str>>;

impl Label {
    pub fn iter_from_reader<R>(reader: R) -> LabelFromReaderIter<R>
    where
        R: Read,
    {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b' ')
            .from_reader(reader);
        reader.into_deserialize()
    }

    pub fn iter_from_path<P>(path: P) -> io::Result<LabelFromPathIter>
    where
        P: AsRef<Path>,
    {
        let reader = BufReader::new(File::open(path)?);
        Ok(Self::iter_from_reader(reader))
    }

    pub fn iter_from_str(text: &str) -> LabelFromStrIter<'_> {
        let reader = Cursor::new(text);
        Self::iter_from_reader(reader)
    }
}
