use super::Label;
use std::{
    borrow::Borrow,
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

impl Label {
    pub fn write_to_writer<W, I, A>(writer: W, labels: I) -> io::Result<()>
    where
        I: IntoIterator<Item = A>,
        W: Write,
        A: Borrow<Label>,
    {
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .delimiter(b' ')
            .from_writer(writer);

        for record in labels {
            writer.serialize(record.borrow())?;
        }

        writer.flush()?;

        Ok(())
    }

    pub fn write_to_path<P, I, A>(path: P, labels: I) -> io::Result<()>
    where
        I: IntoIterator<Item = A>,
        P: AsRef<Path>,
        A: Borrow<Label>,
    {
        let writer = BufWriter::new(File::create(path)?);
        Self::write_to_writer(writer, labels)
    }

    pub fn write_to_string<I, A>(labels: I) -> io::Result<String>
    where
        I: IntoIterator<Item = A>,
        A: Borrow<Label>,
    {
        let mut buf = vec![];
        Self::write_to_writer(&mut buf, labels)?;
        Ok(String::from_utf8(buf).unwrap())
    }
}
