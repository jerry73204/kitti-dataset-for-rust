use measurements::{Angle, Length};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    fs::File,
    io::{self, prelude::*, BufReader, BufWriter, Cursor},
    path::Path,
};

pub use crate::object::{BoundingBox, Class, Extents, Location, Occlusion};
use crate::Error;

pub type LabelFromReaderIter<R> = csv::DeserializeRecordsIntoIter<R, Label>;
pub type LabelFromPathIter = LabelFromReaderIter<BufReader<File>>;
pub type LabelFromStrIter<'a> = LabelFromReaderIter<Cursor<&'a str>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "SerializedLabel", into = "SerializedLabel")]
pub struct Label {
    pub frame: u32,
    pub track_id: u32,
    pub class: Class,
    pub truncation: f64,
    pub occlusion: Occlusion,
    pub alpha: Angle,
    pub bbox: BoundingBox,
    pub extents: Extents,
    pub location: Location,
    pub rotation_y: Angle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SerializedLabel {
    pub frame: u32,
    pub track_id: u32,
    pub class: Class,
    pub truncation: f64,
    pub occlusion: Occlusion,
    pub alpha: f64,
    pub xmin: f64,
    pub ymin: f64,
    pub xmax: f64,
    pub ymax: f64,
    pub height: f64,
    pub width: f64,
    pub length: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub rotation_y: f64,
}

impl From<SerializedLabel> for Label {
    fn from(from: SerializedLabel) -> Self {
        let SerializedLabel {
            frame,
            track_id,
            class,
            truncation,
            occlusion,
            alpha,
            xmin,
            ymin,
            xmax,
            ymax,
            height,
            width,
            length,
            x,
            y,
            z,
            rotation_y,
        } = from;

        Self {
            frame,
            track_id,
            class,
            truncation,
            occlusion,
            alpha: Angle::from_radians(alpha),
            bbox: BoundingBox {
                xmin,
                ymin,
                xmax,
                ymax,
            },
            extents: Extents {
                height: Length::from_meters(height),
                width: Length::from_meters(width),
                length: Length::from_meters(length),
            },
            location: Location {
                x: Length::from_meters(x),
                y: Length::from_meters(y),
                z: Length::from_meters(z),
            },
            rotation_y: Angle::from_radians(rotation_y),
        }
    }
}

impl From<Label> for SerializedLabel {
    fn from(from: Label) -> Self {
        let Label {
            frame,
            track_id,
            class,
            truncation,
            occlusion,
            alpha,
            bbox:
                BoundingBox {
                    xmin,
                    ymin,
                    xmax,
                    ymax,
                },
            extents:
                Extents {
                    height,
                    width,
                    length,
                },
            location: Location { x, y, z },
            rotation_y,
        } = from;

        SerializedLabel {
            frame,
            track_id,
            class,
            truncation,
            occlusion,
            alpha: alpha.as_radians(),
            xmin,
            ymin,
            xmax,
            ymax,
            height: height.as_meters(),
            width: width.as_meters(),
            length: length.as_meters(),
            x: x.as_meters(),
            y: y.as_meters(),
            z: z.as_meters(),
            rotation_y: rotation_y.as_radians(),
        }
    }
}

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

    pub fn vec_from_reader<R>(reader: R) -> Result<Vec<Label>, Error>
    where
        R: Read,
    {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b' ')
            .from_reader(reader);
        let result: Result<Vec<Label>, _> = reader.into_deserialize().collect();
        Ok(result?)
    }

    pub fn vec_from_path<P>(path: P) -> Result<Vec<Label>, Error>
    where
        P: AsRef<Path>,
    {
        let reader = BufReader::new(File::open(path)?);
        Self::vec_from_reader(reader)
    }

    pub fn vec_from_str(text: &str) -> Result<Vec<Label>, Error> {
        let reader = Cursor::new(text);
        Self::vec_from_reader(reader)
    }

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
