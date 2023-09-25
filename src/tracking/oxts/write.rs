use super::Oxts;
use std::{
    borrow::Borrow,
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

pub fn write_to_writer<W, I, A>(writer: W, oxts: I) -> io::Result<()>
where
    I: IntoIterator<Item = A>,
    W: Write,
    A: Borrow<Oxts>,
{
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_writer(writer);

    for record in oxts {
        writer.serialize(record.borrow())?;
    }

    writer.flush()?;

    Ok(())
}

pub fn write_to_path<P, I, A>(path: P, oxts: I) -> io::Result<()>
where
    I: IntoIterator<Item = A>,
    P: AsRef<Path>,
    A: Borrow<Oxts>,
{
    let writer = BufWriter::new(File::create(path)?);
    write_to_writer(writer, oxts)
}

pub fn write_to_string<I, A>(oxts: I) -> io::Result<String>
where
    I: IntoIterator<Item = A>,
    A: Borrow<Oxts>,
{
    let mut buf = vec![];
    write_to_writer(&mut buf, oxts)?;
    Ok(String::from_utf8(buf).unwrap())
}
