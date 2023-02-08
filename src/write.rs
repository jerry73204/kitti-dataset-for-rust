use crate::Annotation;
use std::{
    borrow::Borrow,
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

pub fn to_writer<W, I, A>(writer: W, annotations: I) -> Result<(), io::Error>
where
    I: IntoIterator<Item = A>,
    W: Write,
    A: Borrow<Annotation>,
{
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_writer(writer);

    for record in annotations {
        writer.serialize(record.borrow())?;
    }

    writer.flush()?;

    Ok(())
}

pub fn to_path<P, I, A>(path: P, annotations: I) -> Result<(), io::Error>
where
    I: IntoIterator<Item = A>,
    P: AsRef<Path>,
    A: Borrow<Annotation>,
{
    let writer = BufWriter::new(File::create(path)?);
    to_writer(writer, annotations)
}

pub fn to_string<I, A>(annotations: I) -> Result<String, io::Error>
where
    I: IntoIterator<Item = A>,
    A: Borrow<Annotation>,
{
    let mut buf = vec![];
    to_writer(&mut buf, annotations)?;
    Ok(String::from_utf8(buf).unwrap())
}
